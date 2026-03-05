use std::io::{self, Read};
use crypto_bigint::U256;

use crate::{algorithms::endian::int_to_little_endian, bitcoin::{utils::{read_bytes, read_u64_le}, varint::{encode_varint, read_varint}}};

pub struct TxOut {
    pub amount: u64,
    pub script_pubkey: Vec<u8>,
}

impl TxOut {
    pub fn new(amount: u64, script_pubkey: Vec<u8>) -> Self {
        Self { amount, script_pubkey }
    }

    pub fn parse<R: Read>(stream: &mut R) -> io::Result<TxOut> {

        let amount= read_u64_le(stream)?;

        let pubkey_len = read_varint(stream)? as usize;
        let script_pubkey = read_bytes(stream, pubkey_len)?;

        Ok(TxOut::new(
            amount, 
            script_pubkey, 
        ))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&int_to_little_endian(U256::from_u64(self.amount), 8));

        let pubkey_len = self.script_pubkey.len() as u64;
        let pubkey_len_varint = encode_varint(U256::from_u64(pubkey_len));
        result.extend_from_slice(&pubkey_len_varint);

        result.extend_from_slice(&self.script_pubkey);
        result
    }
}
