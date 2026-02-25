use crypto_bigint::{U256};
use crate::primitives::field_element::{FieldElement, Modulus};

#[derive(Debug)]
pub struct ScalarPrime;
impl Modulus for ScalarPrime {
    fn modulus() -> U256 {
        U256::from_be_hex(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141"
        )
    }
}
pub type Scalar = FieldElement<ScalarPrime>;