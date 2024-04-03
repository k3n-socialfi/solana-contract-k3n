use anchor_lang::prelude::*;
mod contexts;
mod service;
use contexts::*;
use service::service_model::PaymentType;
mod util;
use util::*;

declare_id!("7EsEmSeRQ98ni2qed55nfMFYyD1omasLPsRn7jeWL7jF");

#[program]
pub mod k3n {
    use super::*;

    pub fn create_service(
        ctx: Context<CreateService>,
        kol: Pubkey,
        platform: String,
        service_fee: u64,
        currency: String,
        payment_method: PaymentType,
        description: String,
    ) -> Result<()> {
        let _create_service = ctx.accounts.internal_create_service(
            kol,
            platform,
            service_fee,
            currency,
            payment_method,
            description,
        );

        emit!(CreatedServiceEvent {
            data: 5,
            label: [1, 2, 3, 4, 5],
        });
        Ok(())
    }
}

// Error handling
// #[error]
// pub enum Error {
//     #[msg("Insufficient funds")]
//     InsufficientFunds,
// }
