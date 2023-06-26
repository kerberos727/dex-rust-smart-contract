use thiserror::Error;

/// ## Description
/// This enum describes nft contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("The specified domain is already minted")]
    Minted,
}
