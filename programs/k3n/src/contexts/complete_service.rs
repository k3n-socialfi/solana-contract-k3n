use crate::{model::Service, K3NError};
use anchor_lang::prelude::*;

impl<'info> CompleteService<'info> {
    pub fn internal_complete_service(&mut self) -> Result<()> {
        //let service = &mut ctx.accounts.service;
        // Verify that the signer is the hirer
        require!(
            self.hirer.key() == self.service.hirer.key(),
            K3NError::Permission
        );
        // Update the `is_completed` field
        self.service.is_completed = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CompleteService<'info> {
    #[account(mut, has_one = hirer)]
    pub service: Account<'info, Service>,
    /// CHECK: This is safe because we're explicitly checking the hirer's pubkey in the function
    #[account(signer)]
    pub hirer: AccountInfo<'info>,
}
