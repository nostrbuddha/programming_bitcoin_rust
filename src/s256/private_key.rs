use crypto_bigint::{Random, U256, rand_core::OsRng};
use crate::s256::{s256_field::S256Field, s256_point::S256Point, scalar::Scalar, signature::Signature};

pub struct PrivateKey {
    pub secret: U256,
    pub point: S256Point
}

impl PrivateKey {
    pub fn new(secret: U256) -> Self {
        let point = S256Point::g().rmul(secret);
        Self {
            secret,
            point
        }
    }

    pub fn sign(&self, z: U256) -> Signature {
        let n = S256Point::n();
        let k = U256::from_u128(1234567890u128); //U256::random(&mut OsRng);
        let r = S256Point::g().rmul(k).x().unwrap().value();

        let k_inv = Scalar::new(k).inv();
        println!("{}", hex::encode(k_inv.value().to_be_bytes()));

        let z_scalar = Scalar::new(z);
        let r_scalar = Scalar::new(r);
        let secret_scalar = Scalar::new(self.secret);

        // s = k_inv * (z + r * secret) mod N
        let s_scalar = k_inv * (z_scalar + (r_scalar * secret_scalar));

        let mut s = s_scalar.value();

        /*
        if s > n / U256::from_u8(2) {
            s = n - s;
        }
        */

        println!("r: {r:?}");
        println!("s: {s:?}");

        Signature { r: S256Field::new(r), s: S256Field::new(s) }
    }
}
#[cfg(test)]
mod private_keys_test {
    use crate::s256::hash256::hash256;

    use super::*;

    #[test]
    fn sign() {
    }
}