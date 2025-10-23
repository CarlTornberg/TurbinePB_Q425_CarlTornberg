use anchor_lang::prelude::*;

mod states;
pub use states::*;
mod errors;
pub use errors::*;
mod instructions;
pub use instructions::*;

declare_id!("CyETWpBt8o5hw7Wr7nurskz6rtF4ETSxqXWWwcdERpE2");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize_account(ctx: Context<InitializeCountAccount>) -> Result<()> {
        instructions::initialize_count_account(ctx)
    }

    pub fn count(ctx: Context<IncreaseCount>) -> Result<()> {
        instructions::increase_count(ctx)
    }

}

#[derive(Accounts)]
pub struct Initialize {}
