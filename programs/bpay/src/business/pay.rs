use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TokenAccount, TransferChecked},
    token_interface::{Mint, TokenInterface},
};

#[derive(Accounts)]
pub struct MerchantPay<'info> {
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
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> MerchantPay<'info> {
    pub fn pay(&mut self, amount: u64) -> Result<()> {
        transfer_checked(
            self.into_pay_context(),
            amount,
            self.mint.decimals,
        )
    }

    fn into_pay_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.taker_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.initializer_ata.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        CpiContext::new(self.associated_token_program.to_account_info(), cpi_accounts)
    }
}

