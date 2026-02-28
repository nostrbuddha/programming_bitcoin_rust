use crypto_bigint::{U256};
use crate::{algorithms::base58::base58_check, s256::{s256_field::S256Field, s256_point::S256Point, scalr::Scalar, signature::Signature}};

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

    pub fn sign(self, z: U256) -> Signature {
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

    pub fn wif(self, compressed: bool, testnet: bool) -> String {
        let bytes = self.secret.to_be_bytes();
        let mut res: Vec<u8> = Vec::new();

        if testnet {
            res.push(0xef);
        } else {
            res.push(0x80);
        }

        res.extend_from_slice(&bytes);

        if compressed {
            res.push(0x01);
        }

        base58_check(&res)
    }
}
#[cfg(test)]
mod private_keys_test {

    #[test]
    fn sign() {
    }
}