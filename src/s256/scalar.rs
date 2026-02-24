use core::ops;
use crypto_bigint::{U256, NonZero, Integer};
use crate::s256::s256_point::S256Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scalar(pub U256);

impl Scalar {
    pub fn new(x: U256) -> Self {
        let n = S256Point::n();
        Scalar(x % n)
    }

    pub fn value(self) -> U256 {
        self.0
    }

    /// Modular inverse in Z_n: a^(n-2) mod n (valid because n is prime)
    pub fn inv(self) -> Self {
        assert!(self.0 != U256::ZERO, "cannot invert zero");

        let n = S256Point::n();

        let n_nz: NonZero<U256> = NonZero::new(n).unwrap();

        let mut result = U256::ONE;
        let mut base = self.0 % n;
        let mut exp = n - U256::from_u8(2);

        while exp > U256::ZERO {
            if bool::from(exp.is_odd()) {
                result = result.mul_mod(&base, &n_nz);
            }
            // base = (base * base) % N
            base = base.mul_mod(&base, &n_nz);
            exp >>= 1;
        }

        Scalar(result)
    }
}


impl ops::Add for Scalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Scalar {
        let n = S256Point::n();
        let n_nz = NonZero::new(n).unwrap();

        Scalar(self.0.add_mod(&other.0, &n_nz))

        //Scalar((self.0 + other.0) % n)
        /*
        let sum = self.0.wrapping_add(&other.0);

        if sum >= n {
            Scalar(sum - n)
        } else {
            Scalar(sum)
        }
        */
    }

}

impl ops::Mul for Scalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Scalar {
        let n = S256Point::n();
        let n_nz: NonZero<U256> = NonZero::new(n).unwrap();
        Scalar(self.0.mul_mod(&other.0, &n_nz))
    }
}