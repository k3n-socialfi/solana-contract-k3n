use anchor_lang::prelude::*;

declare_id!("8hWteE8pQPnnMyCiCiPT6TRzc7nKDyE2KQy3pG8Vze3w");

#[program]
pub mod soulbound {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
