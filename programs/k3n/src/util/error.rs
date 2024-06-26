use anchor_lang::error_code;

#[error_code]
pub enum K3NError {
    #[msg("You do not have permisson to do this action")]
    Permission,
    #[msg("The KOL address does not match with KOL address in service, please check again")]
    CheckKolAddressFail,
}

// Example
// Err(error!(K3NError::Permission))
