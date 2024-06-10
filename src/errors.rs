use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("Invalid account type, expected one of 'legacy', 'segwit', 'native_segwit', 'taproot' : {0}")]
    InvalidAddressType(String),
    #[error("Invalid mnemonic")]
    InvalidMnemonic,
    #[error("Invalid HD path: {0}")]
    InvalidHDPath(String),
    #[error("Other error: {0}")]
    Other(String),

}

pub type Result<T> = anyhow::Result<T, AccountError>;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_account_type_error() {
        let error = AccountError::InvalidAddressType("invalid".to_string());
        assert_eq!(error.to_string(), "Invalid account type, expected one of 'legacy', 'segwit', 'native_segwit', 'taproot' : invalid");
    }
}