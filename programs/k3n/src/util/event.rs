use anchor_lang::prelude::*;

// handler function inside #[program]
#[event]
pub struct CreatedServiceEvent {
    pub data: u64,
    pub label: [u8; 5],
}
