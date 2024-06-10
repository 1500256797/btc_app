use std::str::FromStr;
use bitcoin::{psbt::{Input, PsbtSighashType}, secp256k1::{All, Secp256k1}, Psbt, ScriptBuf, Transaction, TxOut, Witness, TxIn, PublicKey, OutPoint, Script, Address, Amount};
use bitcoin::absolute::LockTime;
use bitcoin::transaction::Version;


#[derive(Debug, Clone)]
pub struct UnSignedPsbt {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub public_keys: Vec<PublicKey>,
}


pub struct WitnessUtxo {
    pub script_pubkey: ScriptBuf,
    pub value: Amount,
}

impl UnSignedPsbt {
    pub fn new() -> Self {
        UnSignedPsbt {
            inputs: Vec::new(),
            outputs: Vec::new(),
            public_keys: Vec::new(),
        }
    }

    pub fn add_input(mut self, txid: &str, vout: u32) -> Self {
        let prev_tx_id = txid.parse().unwrap();
        let input = TxIn {
            /// The reference to the previous output that is being used as an input.
            previous_output: OutPoint { txid: prev_tx_id, vout },
            /// The script which pushes values on the stack which will cause the referenced output’s script to be accepted.
            script_sig: ScriptBuf::new(),
            /// The sequence number, which suggests to miners which of two conflicting transactions should be preferred, or 0xFFFFFFFF to ignore this feature. This is generally never used since the miner behavior cannot be enforced.
            sequence: bitcoin::Sequence::from_consensus(0xffffffff),
            /// Witness data: an array of byte-arrays. Note that this field is not (de)serialized with the rest of the TxIn in Encodable/Decodable, as it is (de)serialized at the end of the full Transaction. It is (de)serialized with the rest of the TxIn in other (de)serialization routines.
            witness: Witness::new()
        };
        self.inputs.push(input);
        self
    }

    pub fn add_output( mut self, address: &str, amount: u64) -> Self {
        let recipient_address = Address::from_str(address).unwrap().assume_checked();
        let output = TxOut {
            value: Amount::from_sat(amount),
            script_pubkey: recipient_address.script_pubkey(),
        };
        self.outputs.push(output);
        self
    }

    pub fn add_signer(mut self, public_key: &[u8]) -> Self {
        let public_key = PublicKey::from_slice(public_key).unwrap();
        self.public_keys.push(public_key);
        self
    }

    pub fn build(self) -> Transaction {
        let unsigned_tx = Transaction {
            version: Version::TWO,
            lock_time: LockTime::ZERO,
            input: self.inputs,
            output: self.outputs,
        };
        unsigned_tx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_unsigned_psbt() {
        let public_key = "03a603b1312a59d2d2b9b1c8d1cbc2972f5eb6079b1610570f1418359b926cbb84";
        let unsigned_psbt = UnSignedPsbt::new()
            .add_input("7f6eea7dfcc5bb1e443e63e86c6a63d0b8ed9c367f6b9e4f1e5d6e4f4e3d2c1b", 0)
            .add_output("mv4rnyY3Su5gjcDNzbMLKBQkBicCtHUtFB", 50_000)
            .add_signer(&hex::decode(public_key).unwrap())
            .build();
        assert!(true);
    }
}