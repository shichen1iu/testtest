use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct PaySpl<'info> {
    #[account(mut)]
    /// CHECK:
    pub receiver: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = receiver,
    )]
    pub receiver_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub payer_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info, 'a, 'b, 'c> PaySpl<'info> {
    fn transfer_cpi_context(&self) -> CpiContext<'a, 'b, 'c, 'info, TransferChecked<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.payer_token_account.to_account_info(),
                to: self.receiver_token_account.to_account_info(),
                mint: self.mint.to_account_info(),
                authority: self.payer.to_account_info(),
            },
        )
    }
}

pub fn pay_spl(ctx: Context<PaySpl>, amount: u64) -> Result<()> {
    let cpi_ctx = ctx.accounts.transfer_cpi_context();
    transfer_checked(cpi_ctx, amount, 6)?;
    Ok(())
}
