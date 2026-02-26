use crypto_bigint::U256;

use crate::s256::s256_field::S256Field;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Signature {
    pub r: S256Field,
    pub s: S256Field,
}

impl Signature {

    fn encode_int(x: U256) -> Vec<u8> {
        let mut bytes = x.to_be_bytes().to_vec();

        // Remove leading zeros
        while bytes.len() > 1 && bytes[0] == 0 {
            bytes.remove(0);
        }

        // If high bit is set, prepend 0x00
        if bytes[0] & 0x80 != 0 {
            let mut prefixed = vec![0x00];
            prefixed.extend_from_slice(&bytes);
            bytes = prefixed;
        }

        bytes
    }

    pub fn der(&self) -> Vec<u8> {

        let rbin = Self::encode_int(self.r.num);
        let sbin = Self::encode_int(self.s.num);

        let mut result = Vec::with_capacity(72);

        // INTEGER r
        result.push(0x02);
        result.push(rbin.len() as u8);
        result.extend_from_slice(&rbin);

        // INTEGER s
        result.push(0x02);
        result.push(sbin.len() as u8);
        result.extend_from_slice(&sbin);

        // Wrap in SEQUENCE
        let mut der = Vec::with_capacity(result.len() + 2);
        der.push(0x30);
        der.push(result.len() as u8);
        der.extend_from_slice(&result);

        der
    }
}