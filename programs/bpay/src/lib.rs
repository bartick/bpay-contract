use anchor_lang::prelude::*;

pub mod escrow;
pub use escrow::*;

pub mod state;
pub use state::*;

declare_id!("6pekx4ZX86H6QqSKUD4NxM1HVLgcL8cyP42uMej9HauC");

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
        Ok(())
    }
}