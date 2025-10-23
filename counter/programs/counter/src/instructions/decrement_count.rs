use anchor_lang::prelude::*;

use crate::{CountAccount, CountError};

pub fn decrement_count(ctx: Context<DecrementCount>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    if let Some(x) = counter.count.checked_sub(1) {
        counter.count = x;
        msg!("The account {}'s count is now {}", counter.key(), counter.count);
    }
    else {
        return err!(CountError::CountOverflowUnderflow);
    }
    Ok(())
}


#[derive(Accounts)]
pub struct DecrementCount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
   
    #[account(
        mut,
        seeds = [b"count", signer.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, CountAccount>,
}
