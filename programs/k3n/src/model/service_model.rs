use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Service {
    #[max_len(50)]
    pub platform: String,
    pub service_fee: u64,
    #[max_len(20)]
    pub currency: String,
    pub payment_method: PaymentType,
    pub kol: Pubkey,
    pub hirer: Pubkey,
    #[max_len(200)]
    pub description: String,
    //pub is_completed: bool,
    //pub service_state: ServiceState,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum PaymentType {
    OnetimePayment,
    Milestone,
    //Won { winner: Pubkey },
}

// impl Space for SerService::INIT_SPACEvice {
//     // First 8 Bytes are Discriminator (u64)
//     const INIT_SPACE: usize = 8 + 8 + 1 + 32 + 32 + 32 + 8 + 8;
// }
