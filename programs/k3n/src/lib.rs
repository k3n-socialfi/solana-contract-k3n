use anchor_lang::prelude::*;
mod contexts;
mod model;
use contexts::*;
use model::service_model::PaymentType;
mod util;
use contexts::CompleteService;
use util::*;

declare_id!("7EsEmSeRQ98ni2qed55nfMFYyD1omasLPsRn7jeWL7jF");

#[program]
pub mod k3n {

    use super::*;

    pub fn create_service(
        ctx: Context<CreateService>,
        kol: Pubkey,
        service_name: String,
        platform: String,
        service_fee: u64,
        currency: String,
        payment_method: PaymentType,
        description: String,
    ) -> Result<()> {
        let created_service = ctx.accounts.internal_create_service(
            kol,
            service_name.clone(),
            platform,
            service_fee,
            currency,
            payment_method,
            description,
        );

        emit!(CreatedServiceEvent {
            service_created: created_service.unwrap(),
            service_name,
            kol,
            hier: ctx.accounts.hirer.key(),
        });
        Ok(())
    }

    pub fn complete_service(ctx: Context<CompleteService>) -> Result<()> {
        let _make_complete = ctx.accounts.internal_complete_service();
        Ok(())
    }
}
