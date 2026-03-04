use std::io::{self, Read};

use crypto_bigint::U256;

use crate::algorithms::endian::int_to_little_endian;

pub fn read_varint<R: Read>(r: &mut R) -> io::Result<u64> {
    let mut first_byte = [0u8; 1];
    r.read_exact(&mut first_byte)?;
    if first_byte[0] == 0xfd {
        // next 2 bytes are number
        let mut num_bytes = [0u8; 2];
        r.read_exact(&mut num_bytes)?;
        Ok(u16::from_le_bytes(num_bytes) as u64)
    } else if first_byte[0] == 0xfe {
        // next 4 bytes are number
        let mut num_bytes = [0u8; 4];
        r.read_exact(&mut num_bytes)?;
        Ok(u32::from_le_bytes(num_bytes) as u64)
    } else if first_byte[0] == 0xff {
        // next 8 bytes are number
        let mut num_bytes = [0u8; 8];
        r.read_exact(&mut num_bytes)?;
        Ok(u64::from_le_bytes(num_bytes))
    } else {
        Ok(u8::from_le_bytes(first_byte) as u64)
    }
}

pub fn encode_varint(variant: U256) -> Vec<u8> {
    let mut v = Vec::new();
    if variant < U256::from(0xfdu32) {
        int_to_little_endian(variant, 1)
    } else if variant < U256::from_be_slice(b"10000") {
        v.push(0xfd);
        v.copy_from_slice(&int_to_little_endian(variant, 2));
        v
    } else if variant < U256::from_be_slice(b"100000000") {
        v.push(0xfe);
        v.copy_from_slice(&int_to_little_endian(variant, 4));
        v
    } else if variant < U256::from_be_slice(b"10000000000000000") {
        v.push(0xff);
        v.copy_from_slice(&int_to_little_endian(variant, 8));
        v
    } else {
        vec![]
    }
}