use crate::{model::Service, K3NError};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;

#[derive(Accounts)]
pub struct CompleteService<'info> {
    #[account(mut, constraint = service.hirer.key() == hirer.key(), has_one = hirer)]
    pub service: Account<'info, Service>,
    /// CHECK: This is safe because we're explicitly checking the hirer's pubkey in the function
    #[account(signer)]
    pub hirer: AccountInfo<'info>,
    /// CHECK: The recipient of the SOL transfer.
    #[account(mut)]
    pub kol: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CompleteService<'info> {
    pub fn internal_complete_service(&mut self) -> Result<()> {
        require!(
            self.hirer.key() == self.service.hirer.key(),
            K3NError::Permission
        );
        require!(
            self.kol.key() == self.service.kol.key(),
            K3NError::CheckKolAddressFail
        );

        let transfer_ix = system_instruction::transfer(
            &self.hirer.key(),
            &self.kol.key(),
            self.service.service_fee,
        );

        match invoke(
            &transfer_ix,
            &[self.hirer.to_account_info(), self.kol.to_account_info()],
        ) {
            Ok(_) => {
                self.service.is_completed = true;
                Ok(())
            }
            Err(e) => {
                // Handle or log the error more specifically here
                msg!("Transfer failed with error: {:?}", e);
                Err(e.into())
            }
        }
    }
}
