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

    pub fn increment_account_count(ctx: Context<IncrementCount>) -> Result<()> {
        instructions::increment_count(ctx)
    }

    pub fn devrement_account_count(ctx: Context<DecrementCount>) -> Result<()> {
        instructions::decrement_count(ctx)
    }
}
