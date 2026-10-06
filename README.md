# Escrow Project (Counter Program)

A Solana on-chain program written in Rust with the [Anchor](https://www.anchor-lang.com/) framework.
The program manages a single PDA-based **counter account**:

- **`initialize`** — creates the counter PDA (seeds: `b"counter"` + payer pubkey, so each
  user gets their own counter), sets `count = 0`, stores the payer as the `authority`,
  and transfers `1` lamport as a "Hello, world!" CPI.
- **`increment`** — increments the counter. Only the stored `authority` may call it,
  and the count is capped at `MAX_COUNT = 10`.

The on-chain program lives in `programs/escrow-project`:

```
programs/escrow-project/src/
├── lib.rs                  # program entrypoint (declare_id!, #[program] handlers)
├── constants.rs            # COUNTER_SEED, HELLO_WORLD_LAMPORTS, MAX_COUNT
├── error.rs                # custom error codes (Unauthorized, CounterOverflow)
├── state.rs                # Counter account struct (count, authority)
└── instructions/
    ├── initialize.rs       # Initialize accounts struct + handler
    └── increment.rs        # Increment accounts struct + handler
```

Tests use [LiteSVM](https://github.com/LiteSVM/litesvm) for fast in-process testing
against the compiled `.so` — no local validator needed.

## Prerequisites

- Rust (see `rust-toolchain.toml` for the pinned version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (`cargo build-sbf` / Agave toolchain)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (optional, for `anchor` commands)
- Solana platform-tools **v1.54** (important — see note below)

```bash
cargo build-sbf --tools-version v1.54 --install-only   # one-time install
```

## Build

Build the program to BPF/SBF (produces `target/deploy/escrow_project.so`):

```bash
cargo build-sbf --tools-version v1.54
```

> ⚠️ **Do not use a newer platform-tools version (or plain `anchor build` with the latest
> tools).** Binaries built with newer toolchains are rejected by the LiteSVM 0.10 loader
> used in tests (`InvalidAccountData` error in `add_program`). Stick to **v1.54**.

For a normal host build (type-check / native compile):

```bash
cargo build
```

## Test

The LiteSVM test embeds the compiled `.so` via `include_bytes!`, so **build the SBF
binary first**, then run:

```bash
cargo build-sbf --tools-version v1.54
cargo test
```

`programs/escrow-project/tests/test_initialize.rs` covers:

1. Loading the program into LiteSVM.
2. `initialize` — creates the counter PDA and verifies `count == 0` and the authority.
3. `increment` — verifies `count == 1`.

## Deploy Locally

1. Start a local validator (in a separate terminal):

   ```bash
   solana-test-validator
   ```

2. Configure the CLI for localhost and make sure you have a funded wallet:

   ```bash
   solana config set --url localhost
   solana airdrop 2
   ```

3. Make sure the program ID in `declare_id!` (`programs/escrow-project/src/lib.rs`)
   matches the keypair at `target/deploy/escrow_project-keypair.json` and the entry in
   `Anchor.toml` under `[programs.localnet]`.

4. Deploy:

   ```bash
   solana program deploy target/deploy/escrow_project.so
   ```

   or, with Anchor:

   ```bash
   anchor deploy
   ```

5. Verify:

   ```bash
   solana program show <PROGRAM_ID>
   ```

## Project ID

Program ID: `ELVJ68pUNHjqvF5qLwbvM1UdTV9oX7HRfVX28jsupPPv` (declared in `lib.rs`).
