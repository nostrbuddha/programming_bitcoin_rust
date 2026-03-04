use std::io::{self, Read};
use crypto_bigint::U256;

use crate::{algorithms::endian::int_to_little_endian, bitcoin::{utils::{read_bytes, read_u32_le}, varint::{encode_varint, read_varint}}};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OutPoint {
    pub tx_id: [u8; 32], // little-endian as found on the wire
    pub vout: u32,
}

pub struct TxIn {
    pub previous_output: OutPoint,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

impl TxIn {
    pub fn new(tx_id: [u8; 32], vout: u32, script_sig: Vec<u8>, sequence: u32) -> Self {
        Self { previous_output: OutPoint { tx_id, vout }, script_sig, sequence }
    }

    pub fn parse<R: Read>(stream: &mut R) -> io::Result<TxIn> {

        let mut tx_id_bytes = [0u8; 32];
        stream.read_exact(&mut tx_id_bytes)?;

        let tx_index= read_u32_le(stream)?;

        let sig_len = read_varint(stream)? as usize;
        let script_sig = read_bytes(stream, sig_len)?;

        let sequence = read_u32_le(stream)?;

        Ok(TxIn::new(
            tx_id_bytes, 
            tx_index, 
            script_sig, 
            sequence
        ))
    }

    pub fn serialize(self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&self.previous_output.tx_id);
        result.extend_from_slice(&int_to_little_endian(U256::from_u32(self.previous_output.vout), 4));

        let script_sig_len = self.script_sig.len() as u64;
        let script_sig_len_varint = encode_varint(U256::from_u64(script_sig_len));
        result.extend_from_slice(&script_sig_len_varint);

        result.extend_from_slice(&self.script_sig);

        result.extend_from_slice(&int_to_little_endian(U256::from_u32(self.sequence), 4));

        result
    }
}