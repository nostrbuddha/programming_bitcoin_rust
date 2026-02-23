use core::ops;
use crypto_bigint::{U256, NonZero};

/// secp256k1 curve order n
pub const N: U256 = U256::from_be_hex(
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141"
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scalar(pub U256);

impl Scalar {
    pub fn new(x: U256) -> Self {
        Scalar(x % N)
    }

    pub fn value(self) -> U256 {
        self.0
    }

    /// Modular inverse in Z_n: a^(n-2) mod n (valid because n is prime)
    pub fn inv(self) -> Self {
        assert!(self.0 != U256::ZERO, "cannot invert zero");

        let n_nz: NonZero<U256> = NonZero::new(N).unwrap();

        let mut result = U256::ONE;
        let mut base = self.0 % N;
        let mut exp = N - U256::from_u8(2);

        while exp > U256::ZERO {
            if bool::from(exp.bit(0)) {
                // result = (result * base) % N
                result = result.mul_mod(&base, &n_nz);
            }
            // base = (base * base) % N
            base = base.mul_mod(&base, &n_nz);
            exp >>= 1;
        }

        Scalar(result)
    }
}

impl ops::Mul for Scalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Scalar {
        let n_nz: NonZero<U256> = NonZero::new(N).unwrap();
        Scalar(self.0.mul_mod(&other.0, &n_nz))
    }
}