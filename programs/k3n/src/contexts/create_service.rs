use anchor_lang::{accounts::signer, prelude::*};

use crate::service::{service::PaymentType, Service};

#[derive(Accounts)]
pub struct CreateService<'info> {
    #[account(mut)]
    pub hirer: Signer<'info>,
    #[account(init, payer = hirer, space = Service::INIT_SPACE)]
    pub service: Account<'info, Service>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateService<'info> {
    pub fn internal_create_service(
        &mut self,
        kol: Pubkey,
        platform: String,
        service_fee: u64,
        currency: String,
        payment_method: PaymentType,
        description: String,
    ) -> Result<()> {
        self.service.set_inner(Service {
            platform,
            service_fee,
            currency,
            payment_method,
            kol,
            hirer: self.hirer.key(),
            description,
        });
        Ok(())
    }
}
