use crypto_bigint::{U256};
use crate::s256::{s256_field::S256Field, s256_point::S256Point, scalr::Scalar, signature::Signature};

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
        let k = U256::from_u128(1234567890u128); //TODO: U256::random(&mut OsRng);
        let r = S256Point::g().rmul(k).x().unwrap().num;

        let k_inv = Scalar::new(k).inv();
        println!("{}", hex::encode(k_inv.num.to_be_bytes()));

        let z_scalar = Scalar::new(z);
        let r_scalar = Scalar::new(r);
        let secret_scalar = Scalar::new(self.secret);

        // s = k_inv * (z + r * secret) mod N
        let s_scalar = k_inv * (z_scalar + (r_scalar * secret_scalar));

        let s = s_scalar.num;

        Signature { r: S256Field::new(r), s: S256Field::new(s) }
    }
}
#[cfg(test)]
mod private_keys_test {

    #[test]
    fn sign() {
    }
}