use anchor_lang::prelude::*;

#[account]
pub struct Service {
    pub platform: String,
    pub service_fee: u64,
    pub currency: String,
    pub payment_method: PaymentType,
    pub kol: Pubkey,
    pub hirer: Pubkey,
    pub description: String,
    //pub service_state: ServiceState,
}

impl Space for Service {
    // First 8 Bytes are Discriminator (u64)
    const INIT_SPACE: usize = 8 + 8 + 1 + 32 + 32 + 32 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PaymentType {
    OnetimePayment,
    Milestone,
    //Won { winner: Pubkey },
}

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
// pub enum ServiceState {
//     OnetimePayment,
//     Milestone,
//     //Won { winner: Pubkey },
// }
