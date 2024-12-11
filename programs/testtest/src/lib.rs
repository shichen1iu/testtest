use anchor_lang::prelude::*;
mod error;
mod instructions;

use instructions::*;

declare_id!("CC3bHAaxZR4KCrNTbG9PqZDqckVQn7rzm6Bzir1yuRN2");

#[program]
pub mod testtest {
    use super::*;

    pub fn pay_sol(ctx: Context<PaySol>, amount: u64, reference: Pubkey) -> Result<()> {
        instructions::pay_sol(ctx, amount, reference)
    }

    pub fn pay_spl(ctx: Context<PaySpl>, amount: u64) -> Result<()> {
        instructions::pay_spl(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
