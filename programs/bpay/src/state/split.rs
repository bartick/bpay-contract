use anchor_lang::prelude::*;

// EqualSplit struct including BaseSplit
#[account]
#[derive(Default)]
pub struct EqualSplit {
    pub seed: u64,
    pub bumps: u8,
    pub initializer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub paid_by: Vec<Pubkey>,
    pub total_payee: u8,
}

pub trait SplitSpace {
    fn space(total_payee: u8) -> usize {
        8 +  // seed
        1 +  // bump
        32 + // initializer
        4 + (total_payee as usize * 32) + // paidBy (4 bytes for the length + size of Pubkeys)
        32 + // mint
        8 +  // amount
        1    // total_payee
    }
}

impl SplitSpace for EqualSplit {}

// CustomSplit struct including BaseSplit
#[account]
#[derive(Default, InitSpace)]
pub struct CustomSplit {
    pub seed: u64,
    pub bumps: u8,
    pub initializer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub paid_by: Pubkey,
}