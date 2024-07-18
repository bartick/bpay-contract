use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"state".as_ref(), initializer.key().as_ref(), &seed.to_le_bytes()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_escrow(&mut self, seed: u64, bumps: &InitializeBumps, taker_amount: u64) {
        self.escrow.set_inner(Escrow {
            seed,
            bump: bumps.escrow,
            initializer: self.initializer.key(),
            mint: self.mint.key(),
            taker_amount,
        });
    }
}
