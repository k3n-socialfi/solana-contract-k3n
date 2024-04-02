use anchor_lang::prelude::*;

declare_id!("8gc7QHvsreuPTUrYpJ7dJxr6c8UB8kapeLW9Uoot8G1g");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
