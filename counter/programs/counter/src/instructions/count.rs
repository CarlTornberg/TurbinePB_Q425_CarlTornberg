use anchor_lang::prelude::*;

use crate::{CountAccount, CountError};

pub fn increase_count(ctx: Context<IncreaseCount>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    if let Some(x) = counter.count.checked_add(1) {
        counter.count = x;
        msg!("The account {}'s count is now {}", counter.key(), counter.count);
    }
    else {
        return Err(CountError::CountOverflow.into());
    }
    Ok(())
}


#[derive(Accounts)]
pub struct IncreaseCount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
   
    #[account(
        mut,
        seeds = [b"count", signer.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, CountAccount>,
}
