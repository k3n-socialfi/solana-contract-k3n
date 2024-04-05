use crate::model::{service_model::PaymentType, Service};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateService<'info> {
    #[account(mut)]
    pub hirer: Signer<'info>,
    #[account(init, payer = hirer, space = 8 + Service::INIT_SPACE)]
    pub service: Account<'info, Service>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateService<'info> {
    pub fn internal_create_service(
        &mut self,
        kol: Pubkey,
        service_name: String,
        platform: String,
        service_fee: u64,
        currency: String,
        payment_method: PaymentType,
        description: String,
    ) -> Result<Pubkey> {
        self.service.set_inner(Service {
            service_name,
            platform,
            service_fee,
            currency,
            payment_method,
            kol,
            hirer: self.hirer.key(),
            description,
            is_completed: false,
        });
        Ok(self.service.key())
    }
}
