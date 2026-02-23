use std::ops;
use crypto_bigint::{I256, U256};
use crypto_bigint::NonZero;

// p = 2^256 - 2^32 - 977
const S256_PRIME: U256 = U256::from_be_hex(
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"
);

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct S256Field(U256);

impl S256Field {
    pub fn new(num: U256) -> Self {
        assert!(num < S256_PRIME, "Num {} not in field range 0 to {}", num, S256_PRIME - U256::ONE);
        Self(num)
    }

    pub fn new_zero() -> Self {
        Self(U256::ZERO)
    }

    pub fn new_one() -> Self {
        Self(U256::ONE)
    }

    pub fn new_two() -> Self {
        Self(U256::from_u32(2u32))
    }

    pub fn new_three() -> Self {
        Self(U256::from_u32(3u32))
    }

    pub fn value(&self) -> U256 {
        self.0
    }

    pub fn inv(self) -> Self {
        assert!(self.0 != U256::ZERO, "cannot invert zero");
        let mut result = S256Field::new(U256::ONE);
        let mut base = self;
        let mut exp = S256_PRIME - U256::from(2u8);

        while exp > U256::ZERO {
            if exp.bit(0).into() {
                result = result * base;
            }
            base = base * base;
            exp >>= 1;
        }

        result
    }

    pub fn pow(self, exponent: I256) -> S256Field {
        let order: U256 = S256_PRIME - U256::ONE;

        let mut exp: U256 = if exponent >= I256::ZERO {
            exponent.as_uint() % order
        } else {
            let neg: U256 = exponent.wrapping_neg().as_uint() % order;
            (order - neg) % order
        };

        let mut result = S256Field::new_one();
        let mut base = self;

        while exp > U256::ZERO {
            if exp.bit(0).into() {
                result = result * base;
            }
            base = base * base;
            exp >>= 1;
        }

        result
    }
}

impl ops::Add for S256Field {
    type Output = S256Field;

    fn add(self, other: S256Field) -> S256Field {
        let num = (self.0 + other.0) % S256_PRIME;
        S256Field(num)
    }
}

impl ops::Sub for S256Field {
    type Output = S256Field;

    fn sub(self, other: S256Field) -> S256Field {
        let num = if self.0 >= other.0 {
            (self.0 - other.0) % S256_PRIME
        } else {
            (S256_PRIME - (other.0 - self.0) % S256_PRIME) % S256_PRIME
        };
        S256Field(num)
    }

}

impl ops::Mul for S256Field {
    type Output = S256Field;

    fn mul(self, other: S256Field) -> S256Field {
        let prime_nz = NonZero::new(S256_PRIME).unwrap();
        let num = self.0.mul_mod(&other.0, &prime_nz) % S256_PRIME;

        S256Field(num)
    }

}