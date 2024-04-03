use anchor_lang::prelude::*;
mod contexts;
mod service;
use contexts::*;
use service::service::PaymentType;

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
        ctx.accounts.internal_create_service(
            kol,
            platform,
            service_fee,
            currency,
            payment_method,
            description,
        )
    }
}

// Error handling
// #[error]
// pub enum Error {
//     #[msg("Insufficient funds")]
//     InsufficientFunds,
// }
