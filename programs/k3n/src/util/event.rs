use anchor_lang::prelude::*;

// handler function inside #[program]
#[event]
pub struct CreatedServiceEvent {
    pub service_created: Pubkey,
    pub service_name: String,
    pub kol: Pubkey,
    pub hier: Pubkey,
}
