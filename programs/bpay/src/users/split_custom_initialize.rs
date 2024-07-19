use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};

use crate::CustomSplit;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct SplitCustomInitialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        space = 8 + CustomSplit::INIT_SPACE,
        seeds = [b"state".as_ref(), initializer.key().as_ref(), &seed.to_le_bytes()],
        bump
    )]
    pub split: Account<'info, CustomSplit>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> SplitCustomInitialize<'info> {
    pub fn initialize_split(&mut self, seed: u64, bumps: &SplitCustomInitializeBumps, amount: u64) {
        self.split.set_inner(CustomSplit {
            seed,
            bumps: bumps.split,
            initializer: self.initializer.key(),
            mint: self.mint.key(),
            amount,
            paid_by: Pubkey::default(),
        });
    }
}