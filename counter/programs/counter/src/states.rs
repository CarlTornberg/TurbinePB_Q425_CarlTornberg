use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CountAccount {
    pub owner: Pubkey,
    pub count: u64,
    pub bump: u8,
}
