use crate::errors::{AccountError, Result};
pub enum AddressType {
    /// legacy m/44'/0'/0'
    P2PKH,
    /// native segwit m/84'/0'/0'
    P2WPKH,
    /// taproot m/86'/0'/0'
    P2TR,
    /// nested segwit m/49'/0'/0'
    P2SH_P2WPKH,
}

impl AddressType {
    pub fn from_str(s: &str) -> Result<AddressType> {
        match s {
            "p2pkh" => Ok(AddressType::P2PKH),
            "p2wpkh" => Ok(AddressType::P2WPKH),
            "p2tr" => Ok(AddressType::P2TR),
            "p2sh_p2wpkh" => Ok(AddressType::P2SH_P2WPKH),
            _ => Err(AccountError::InvalidAddressType(s.to_string())),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AddressType::P2PKH => "p2pkh".to_string(),
            AddressType::P2WPKH => "p2wpkh".to_string(),
            AddressType::P2TR => "p2tr".to_string(),
            AddressType::P2SH_P2WPKH => "p2sh_p2wpkh".to_string(),

        }
    }

    pub fn default_path(&self) -> &str {
        match self {
            AddressType::P2PKH => "m/44'/0'/0'",
            AddressType::P2SH_P2WPKH => "m/49'/0'/0'",
            AddressType::P2WPKH => "m/84'/0'/0'",
            AddressType::P2TR=> "m/86'/0'/0'",
        }
    }
}





