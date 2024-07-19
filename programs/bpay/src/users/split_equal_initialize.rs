use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};

use crate::{state::EqualSplit, SplitSpace};

#[derive(Accounts)]
#[instruction(seed: u64, total_payee: u8)]
pub struct SplitEqualInitialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        space = 8 + EqualSplit::space(total_payee),
        seeds = [b"state".as_ref(), initializer.key().as_ref(), &seed.to_le_bytes()],
        bump
    )]
    pub split: Account<'info, EqualSplit>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> SplitEqualInitialize<'info> {
    pub fn initialize_split(&mut self, seed: u64, bumps: &SplitEqualInitializeBumps, total_amount: u64, total_payee: u8) {
        let amount = total_amount / total_payee as u64;
        self.split.set_inner(EqualSplit {
            seed,
            bumps: bumps.split,
            initializer: self.initializer.key(),
            mint: self.mint.key(),
            amount,
            paid_by: vec![self.initializer.key(); total_payee as usize],
            total_payee,
        });
    }
}