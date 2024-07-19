use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TokenAccount, TransferChecked},
    token_interface::{Mint, TokenInterface},
};

use crate::state::EqualSplit;

#[derive(Accounts)]
pub struct SplitEqual<'info> {
    #[account(mut)]
    pub payee: Signer<'info>,
    #[account(mut)]
    pub initializer: SystemAccount<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payee,
        associated_token::mint = mint,
        associated_token::authority = payee,
    )]
    pub payee_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = initializer,
    )]
    pub initializer_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        has_one = mint,
        constraint = payee_ata.amount >= split.amount,
        seeds=[b"state", initializer.key().as_ref(), split.seed.to_le_bytes().as_ref()],
        bump = split.bumps,
    )]
    pub split: Account<'info, EqualSplit>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> SplitEqual<'info> {
    pub fn pay(&mut self) -> Result<()> {
        transfer_checked(
            self.into_deposit_context(),
            self.split.amount,
            self.mint.decimals,
        )
    }

    fn into_deposit_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.payee_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.initializer_ata.to_account_info(),
            authority: self.payee.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}