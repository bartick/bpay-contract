use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Merchant {
    pub seed: u64,
    pub bump: u8,
    pub initializer: Pubkey,
    pub mint: Pubkey,
    pub taker_amount: u64,
}