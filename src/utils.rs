use bitcoin::PublicKey;
use crate::account::account::Account;
use crate::account::address_type::AddressType;

/// this is a utility functions for generate dummy utxos for testing
///
///

pub struct UnspentOutput {
    tx_id:String,
    vout:u32,
    satoshis:i32,
    script_publkey:String,
    public_key: PublicKey,
    address_type: AddressType,
    inscriptions: Vec<Inscriptions>,
    runes: Option<Vec<Runes>>,
    raw_tx:Option<String>

}

pub struct Inscriptions {
    inscription_id:String,
    inscription_number:u32,
    inscription_type: InscriptionType,
    ticker:Option<String>,
}

enum InscriptionType {
    NFT,
    FT
}


pub struct Runes {
    rune_id:String,
    amount:String,
}


pub struct Assets {
    inscriptions: Option<Vec<Inscriptions>>,
    runes: Option<Vec<Runes>>,
    tx_id:Option<String>,
    vout:Option<u32>,
}

