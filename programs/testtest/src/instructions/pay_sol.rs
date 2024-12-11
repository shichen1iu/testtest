use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(Accounts)]
pub struct PaySol<'info> {
    #[account(mut)]
    /// CHECK:
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn pay_sol(ctx: Context<PaySol>, amount: u64, reference: Pubkey) -> Result<()> {
    require!(
        ctx.accounts.payer.key() == reference,
        ErrorCode::InvalidReceiver
    );

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.receiver.to_account_info(),
            },
        ),
        amount,
    )?;
    Ok(())
}
