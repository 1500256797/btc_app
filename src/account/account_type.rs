use crate::errors::{AccountError, Result};
pub enum AccountType {
    Legacy,
    Segwit,
    NativeSegwit,
    Taproot,
}

impl AccountType {
    pub fn from_str(s: &str) -> Result<AccountType> {
        match s {
            "legacy" => Ok(AccountType::Legacy),
            "segwit" => Ok(AccountType::Segwit),
            "native_segwit" => Ok(AccountType::NativeSegwit),
            "taproot" => Ok(AccountType::Taproot),
            _ => Err(AccountError::InvalidAccountType(s.to_string())),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AccountType::Legacy => "legacy".to_string(),
            AccountType::Segwit => "segwit".to_string(),
            AccountType::NativeSegwit => "native_segwit".to_string(),
            AccountType::Taproot => "taproot".to_string(),
        }
    }

    pub fn default_path(&self) -> &str {
        match self {
            AccountType::Legacy => "m/44'/0'/0'",
            AccountType::Segwit => "m/49'/0'/0'",
            AccountType::NativeSegwit => "m/84'/0'/0'",
            AccountType::Taproot => "m/86'/0'/0'",
        }
    }
}





