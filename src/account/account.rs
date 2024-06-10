use std::{default, str::FromStr};
use bitcoin::{bip32::{ChildNumber, DerivationPath, Xpriv, Xpub}, key::Secp256k1, Address, Network, PrivateKey, PublicKey, CompressedPublicKey};
use crate::errors::{AccountError, Result};
use super::address_type::{self, AddressType};
use bip39::{Mnemonic, Language};
use bitcoin::hex::DisplayHex;
use bitcoin::key::UntweakedPublicKey;
use bitcoin::secp256k1::SecretKey;

#[derive(Debug)]
pub struct Account {
    pub address:Address,
    pub wif_private_key:String,
    pub public_key: PublicKey,
    pub xpub:Xpub,
    pub xpriv:Xpriv,
    pub hd_path:DerivationPath  
}

impl Account {

    pub fn from_random(address_type: AddressType, network:Network) -> Result<Account> {
        let mnemonic = Mnemonic::generate_in(Language::English, 12).map_err(|_| AccountError::InvalidMnemonic)?;
        Account::from_mnemonic(network,mnemonic.to_string().as_str(), address_type)
    }

    pub fn from_mnemonic(network:Network, mnemonic:&str, account_type: AddressType) -> Result<Account> {

        let mnemonic = Mnemonic::from_str(mnemonic).map_err(|_| AccountError::InvalidMnemonic)?;

        let seed = mnemonic.to_seed("");

        let root = Xpriv::new_master(network, &seed).map_err(|e| AccountError::Other(e.to_string()))?;


        // derive the account xpriv
        let secp256k1 = Secp256k1::new();
        let hd_path = DerivationPath::from_str(account_type.default_path()).map_err(|_| AccountError::InvalidHDPath(account_type.default_path().to_string()))?;
        let xpriv = root.derive_priv(&secp256k1, &hd_path).map_err(|e| AccountError::Other(e.to_string()))?;
        let xpub = Xpub::from_priv(&secp256k1, &xpriv);

        // derive the account from xpriv or xpub
        let zero = ChildNumber::from_normal_idx(0).map_err(|e| AccountError::Other(e.to_string()))?;
        let public_key = xpub.derive_pub(&secp256k1, &[zero, zero]).map_err(|e| AccountError::Other(e.to_string()))?.public_key;

        let private_key = xpriv.derive_priv(&secp256k1, &[zero, zero]).map_err(|e| AccountError::Other(e.to_string()))?.private_key;
        // convert the hex private key to wif format
        let wif_private_key = PrivateKey::new(private_key, network).to_wif();

        let public_key = PublicKey::new(public_key);
        let address =  match account_type {
            AddressType::P2PKH => Address::p2pkh(&public_key, network),
            AddressType::P2SH_P2WPKH => {
                let compressed = CompressedPublicKey(public_key.inner);
                Address::p2shwpkh(&compressed, network)
            }
            AddressType::P2WPKH => {
                let compressed = CompressedPublicKey(public_key.inner);
                Address::p2wpkh(&compressed, network)
            },
            AddressType::P2TR => {
                let untweaked_public_key = public_key.inner;
                Address::p2tr(&secp256k1, UntweakedPublicKey::from(untweaked_public_key), None, network)
            },
            _ => Address::p2pkh(&public_key, network)
        };

        Ok(
            Account {
                address,
                xpub:xpub,
                xpriv:xpriv,
                wif_private_key:wif_private_key,
                public_key: public_key,
                hd_path,
            }
        )
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_derive_account_from_mnemonic() {
        // THERE IS NO FUCKING ASSETS ! SO DONT STEAL MONEY !
        let test_mnemoic = "wish film peasant much sure thought speed print napkin hard crumble envelope";
        let account = Account::from_mnemonic(Network::Bitcoin, test_mnemoic, AddressType::P2PKH).unwrap();

        // assert eq
        assert_eq!(
            "xprv9y6ctoiHQPqekbRQi8EYzw1DxD47E6S5iERL4kREhuTWXfTzkd12CcfVeRen942QQ7cnszCYgattcBdNSR2r4WCzj4hcfCo5cyjr8AFoqEa".to_string(),
            account.xpriv.to_string()
        );

        assert_eq!(
            "xpub6C5yJKFBEmPwy5Vsp9mZN4wxWEtbdZ9w5TLvs8prGEzVQTo9JAKGkQyyViBS7frzQeMWK4mb1sUYNgQSfkn1vpfU21cJiTjyy32wZnrXWUP".to_string(),
            account.xpub.to_string()
        );

        assert_eq!(
            "13sBVfioESgW4ZGfmpCowq5utCQnySD98s".to_string(),
            account.address.to_string()
        );
        assert_eq!(
            "030fbdddfaf0d460eddd4542cc365dfc99e397b0580a6d6a554fac1750a180f7da".to_string(),
            account.public_key.to_string()
        );
        assert_eq!(
            "Kz4AiXLbrKWPnghcLcVTdDmN3hSBEKBX2vqTVrvG7MchpMYqzL7N".to_string(),
            account.wif_private_key
        );
    }


    #[test]
    fn test_derive_account_from_mnemonic_bip84() {
        // THERE IS NO FUCKING ASSETS ! SO DONT STEAL MONEY !
        let test_mnemoic = "wish film peasant much sure thought speed print napkin hard crumble envelope";
        let account = Account::from_mnemonic(Network::Bitcoin, test_mnemoic, AddressType::P2WPKH).unwrap();

        assert_eq!(
            "bc1qvgsr2jt7wzxcalv82045rzd9ed8pxljdwck269".to_string(),
            account.address.to_string()
        );
        assert_eq!(
            "024e093f4d7dac92860d6e54cf9e553dbedb209fbcfb4482013da48e37a933b7cb".to_string(),
            account.public_key.to_string()
        );
        assert_eq!(
            "KzNg2w1LmYkDSCUkVBo9W7F3ioLVVF8WkDshxjQGh7iSDrAau798".to_string(),
            account.wif_private_key
        );
    }


    #[test]
    fn test_derive_account_from_mnemonic_taproot() {
        // THERE IS NO FUCKING ASSETS ! SO DONT STEAL MONEY !
        let test_mnemoic = "wish film peasant much sure thought speed print napkin hard crumble envelope";
        let account = Account::from_mnemonic(Network::Bitcoin, test_mnemoic, AddressType::P2TR).unwrap();

        assert_eq!(
            "bc1pvtta6s57eza05pd5xjnnrzya6qxw5ua5d2strh3wss5yla7qytsqdzp5d2".to_string(),
            account.address.to_string()
        );
        assert_eq!(
            "027d97199dc0e285e9432b644afbc406964422aba9271b91167c5de45ebf05a608".to_string(),
            account.public_key.to_string()
        );
        assert_eq!(
            "L3vQW2hH334dzaSEFxukFxSP2665XwR5R2qqg5aTzwpPWK8ppdd4".to_string(),
            account.wif_private_key
        );
    }


    #[test]
    fn test_from_random() {
        let account = Account::from_random(AddressType::P2PKH, Network::Bitcoin).unwrap();
        println!("Account: {:?}", account);
    }
}