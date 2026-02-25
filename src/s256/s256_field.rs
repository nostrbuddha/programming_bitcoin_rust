use crypto_bigint::{U256};
use crate::primitives::field_element::{FieldElement, Modulus};

#[derive(Debug)]
pub struct Secp256k1Prime;
impl Modulus for Secp256k1Prime {
    fn modulus() -> U256 {
        U256::from_be_hex(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"
        )
    }
}
pub type S256Field = FieldElement<Secp256k1Prime>;

impl FieldElement<Secp256k1Prime> {
    pub fn new_zero() -> Self {
        Self::new(U256::ZERO)
    }

    pub fn new_one() -> Self {
        Self::new(U256::ONE)
    }

    pub fn new_two() -> Self {
        Self::new(U256::from_u32(2u32))
    }

    pub fn new_three() -> Self {
        Self::new(U256::from_u32(3u32))
    }
}