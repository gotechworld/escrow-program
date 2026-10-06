use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("ELVJ68pUNHjqvF5qLwbvM1UdTV9oX7HRfVX28jsupPPv");

#[program]
pub mod escrow_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        handle_increment(ctx)
    }
}
