use anchor_lang::prelude::*;
use crate::CountAccount;

pub fn initialize_count_account(ctx: Context<InitializeCountAccount>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    counter.owner = *ctx.accounts.signer.key;
    counter.count = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCountAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
   
    #[account(
        init,
        payer = signer,
        space = 8 + CountAccount::INIT_SPACE,
        seeds = [b"count", signer.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, CountAccount>,

    pub system_program: Program<'info, System>,
}
