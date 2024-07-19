use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TokenAccount, TransferChecked},
    token_interface::{Mint, TokenInterface},
};

use crate::state::Merchant;

#[derive(Accounts)]
pub struct MerchantExchange<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub initializer: SystemAccount<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = initializer,
    )]
    pub initializer_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        has_one = mint,
        constraint = taker_ata.amount >= escrow.taker_amount,
        seeds=[b"state", initializer.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Merchant>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> MerchantExchange<'info> {
    pub fn deposit(&mut self) -> Result<()> {
        transfer_checked(
            self.into_deposit_context(),
            self.escrow.taker_amount,
            self.mint.decimals,
        )
    }

    fn into_deposit_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.taker_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.initializer_ata.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn close(&mut self) -> Result<()> {
        self.escrow.close(self.taker_ata.to_account_info())
    }
}