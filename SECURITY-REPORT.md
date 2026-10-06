# Security Audit Report — Escrow Project (Counter Program)

**Date:** 2026-10-06
**Scope:** the Solana/Anchor program in `programs/escrow-project` (the `initialize` and `increment` instructions)
**Program ID:** `ELVJ68pUNHjqvF5qLwbvM1UdTV9oX7HRfVX28jsupPPv`

## Executive Summary

The program has a small attack surface (2 instructions, a single CPI to the System Program).
The audit **found no exploitable critical, high, or medium severity vulnerabilities**.
**One low-severity issue** was identified (global counter authority takeover via
front-running).

| # | Severity | File | Lines | Vulnerability | Confidence |
|---|----------|------|-------|---------------|------------|
| 1 | ⚪ LOW | `instructions/initialize.rs` | 8-18 | Global singleton counter PDA — the first caller of `initialize` becomes the permanent `authority` | 9/10 |

## Finding 1 — Global Counter Authority Takeover (LOW)

**File:** `programs/escrow-project/src/instructions/initialize.rs:8-18`
(seeds defined in `constants.rs`)

**Description:** The counter PDA is derived only from `[COUNTER_SEED]` (`b"counter"`),
with no user-specific or deployer-specific component. The first account that calls
`initialize` becomes the permanent `authority` stored in `Counter.authority`.
There is no admin, close, or re-initialization mechanism.

**Exploit scenario:** an attacker can front-run the deployer's `initialize`
transaction (or simply call it first after deployment) and permanently take
exclusive control of the only counter the program can ever have. No funds are at
risk (only 1 lamport + rent), so the impact is limited to griefing / taking over
ownership of the counter.

**Evidence:**
- `seeds = [COUNTER_SEED]` — no user-specific component;
- `handle_initialize` unconditionally sets `authority = payer.key()`;
- `increment` enforces `counter.authority == authority.key()` — whoever initializes
  first owns the singleton forever.

**Recommended remediation (one of):**
1. Include the initializer's pubkey in the PDA seeds so each user gets their own
   counter:
   ```rust
   seeds = [COUNTER_SEED, payer.key().as_ref()]
   ```
2. If a single global counter is intended, restrict `initialize` to a hardcoded
   admin or one verified via the upgrade authority.

## Areas Checked and Confirmed SAFE

- **Reinitialization:** safe. Anchor's `init` constraint fails if the PDA account
  already exists and is initialized.
- **Signer checks:** `payer` is a `Signer` in `Initialize`; `authority` is a
  `Signer` in `Increment`, enforced via `require_keys_eq!` against
  `counter.authority` (`increment.rs:13-17`). Correct.
- **PDA verification:** both instructions use `seeds`/`bump` constraints; Anchor
  verifies that `counter` is the canonical PDA, and `Account<'info, Counter>`
  enforces program ownership + discriminator — no account confusion or fake-account
  substitution is possible.
- **CPI (lamport transfer, `initialize.rs:24-29`):** not a vulnerability. `init`
  already funds the account to the rent-exempt minimum; transferring 1 extra lamport
  into a program-owned account is valid and does not affect ownership.
- **Arithmetic:** `count += 1` is guarded by `require!(count < MAX_COUNT)` with
  `MAX_COUNT = 10` — overflow is unreachable. No subtractions exist.
- **Reentrancy:** the only CPI is the System Program transfer, which cannot re-enter
  this program. `increment` performs no CPIs.
- **Unchecked accounts:** all accounts in both instructions are constrained
  (Signer, PDA-verified Account, `Program<System>`). No
  `UncheckedAccount`/`AccountInfo` is used.
- **Dependencies/configuration:** `anchor-lang 1.2.0` / `anchor-spl 1.2.0` — no
  known advisories affecting this usage (`anchor-spl` is actually unused by the
  program code). `Anchor.toml` targets localnet; nothing security-relevant.

## Conclusion

The program is **safe to deploy** from the perspective of directly exploitable
vulnerabilities.

**Update (2026-10-06):** Finding 1 has been **fixed**. The PDA seeds now include
the user's pubkey (`[COUNTER_SEED, payer.key()]`), so each user has their own
counter and no one can take over another counter's authority. The change was
applied to `initialize.rs`, `increment.rs`, and the tests; all tests pass.
