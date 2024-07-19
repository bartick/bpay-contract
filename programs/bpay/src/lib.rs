use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod business;
pub use business::*;

pub mod users;
pub use users::*;

declare_id!("HsJ8eBYS75GzUBDBMjSrhm1hY8r1ipeVbrM9FdZ5fqqQ");

#[program]
mod gpay {
    use super::*;

    // Initialize the account to store the amount to pay the initializer
    pub fn merchant_initialize(ctx: Context<MerchantInitialize>, seed: u64, taker_amount: u64) -> Result<()> {
        ctx.accounts
            .initialize_escrow(seed, &ctx.bumps, taker_amount);
        Ok(())
    }

    // Send the amount to the initializer when the taker is ready
    pub fn merchant_exchange(ctx: Context<MerchantExchange>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.close()?;
        Ok(())
    }

    // Custom payment
    pub fn merchant_pay(ctx: Context<MerchantPay>, amount: u64) -> Result<()> {
        ctx.accounts.pay(amount)?;
        Ok(())
    }

    pub fn split_equal_init(ctx: Context<SplitEqualInitialize>, seed: u64, total_payee: u8, total_amount: u64) -> Result<()> {
        ctx.accounts
            .initialize_split(seed, &ctx.bumps, total_amount, total_payee);
        Ok(())
    }

    pub fn split_equal(ctx: Context<SplitEqual>) -> Result<()> {
        ctx.accounts.pay()?;
        Ok(())
    }

    pub fn split_custom_init(ctx: Context<SplitCustomInitialize>, seed: u64, amount: u64) -> Result<()> {
        ctx.accounts
            .initialize_split(seed, &ctx.bumps, amount);
        Ok(())
    }

    pub fn split_custom(ctx: Context<SplitCustom>) -> Result<()> {
        ctx.accounts.pay()?;
        ctx.accounts.close()?;
        Ok(())
    }
}