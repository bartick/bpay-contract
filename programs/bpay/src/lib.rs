use anchor_lang::prelude::*;

pub mod escrow;
pub use escrow::*;

pub mod state;
pub use state::*;

declare_id!("HsJ8eBYS75GzUBDBMjSrhm1hY8r1ipeVbrM9FdZ5fqqQ");

#[program]
mod gpay {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, taker_amount: u64) -> Result<()> {
        ctx.accounts
            .initialize_escrow(seed, &ctx.bumps, taker_amount);
        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.close()?;
        Ok(())
    }
}