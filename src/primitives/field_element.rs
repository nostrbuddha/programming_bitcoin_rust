use std::ops;
use crypto_bigint::U256;
use crypto_bigint::I256;
use crypto_bigint::NonZero;

pub trait Modulus {
    fn modulus() -> U256;
}

#[derive(Debug)]
pub struct FieldElement<M: Modulus> {
    pub num: U256,
    _marker: core::marker::PhantomData<M>, // Just to make use of M, without saving it
}

impl<M: Modulus> FieldElement<M> {
    pub fn new(num: U256) -> Self {
        let p = M::modulus();
        assert!(num < p, "Num {} not in field range 0 to {}", num, p - U256::ONE);
        Self {
            num,
            _marker: std::marker::PhantomData,
        }
    }

    // TODO: Write test for this
    // (p + 1) / 4
    pub fn sqrt(self) -> Self {
        let p = M::modulus();
        let exponent = (p + U256::ONE) >> 2;
        self.pow_u256(exponent)
    }

    // TODO: Write test for this
    pub fn pow(self, exponent: I256) -> Self {
        let p = M::modulus();
        let order = p - U256::ONE; // p-1

        // r = exponent mod (p-1), mapped into [0, p-2]
        let r: U256 = if exponent >= I256::ZERO {
            exponent.as_uint() % order
        } else {
            // For negative exponent: (-k mod order) == order - (k mod order) (and 0 stays 0)
            let k = exponent.wrapping_neg().as_uint() % order;
            if k == U256::ZERO { U256::ZERO } else { order - k }
        };

        self.pow_u256(r)
    }

    // TODO: Write test for this
    pub fn inv(self) -> Self {
        assert!(self.num != U256::ZERO, "cannot invert zero");
        let p = M::modulus();
        let mut result = Self::new(U256::ONE);
        let mut base = self;
        let mut exp = p - U256::from(2u8);

        while exp > U256::ZERO {
            if exp.bit(0).into() {
                result = result * base;
            }
            base = base * base;
            exp >>= 1;
        }

        result
    }

    pub fn get_prime(self) -> U256 {
        return M::modulus()
    }

    fn pow_u256(self, mut exp: U256) -> Self {
        let p = M::modulus();
        let p_nz = NonZero::new(p).expect("modulus must be non-zero");

        let mut result = U256::ONE;
        let mut base = self.num;

        while exp > U256::ZERO {
            if bool::from(exp.bit(0)) {
                result = result.mul_mod(&base, &p_nz);
            }
            base = base.mul_mod(&base, &p_nz);
            exp >>= 1;
        }

        Self::new(result)
    }

}

impl<M: Modulus> Clone for FieldElement<M> {
    fn clone(&self) -> Self {
        Self {
            num: self.num,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<M: Modulus> Copy for FieldElement<M> {}

impl<M: Modulus> ops::Add for FieldElement<M> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let p = M::modulus();
        let num = self.num.add_mod(&other.num, &p);
        Self::new(num)
    }

}

impl<M: Modulus> ops::Sub for FieldElement<M> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let p = M::modulus();
        let num = if self.num >= other.num {
            self.num - other.num
        } else {
            p - (other.num - self.num) % p
        };
        Self::new(num)
    }

}

impl<M: Modulus> ops::Mul for FieldElement<M> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let p = M::modulus();
        let prime_nz = NonZero::new(p).unwrap();
        let num = self.num.mul_mod(&other.num, &prime_nz);

        Self::new(num)
    }

}

impl<M: Modulus> ops::Div for FieldElement<M> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let p = M::modulus();
        let exp = p.as_int() - I256::from(2);
        let divisor = other.pow(exp);
        let num = (self.num * divisor.num) % p;
        Self::new(num)
    }

}

impl<M: Modulus> PartialEq for FieldElement<M> {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl<M: Modulus> Eq for FieldElement<M> {}

#[derive(Debug)]
pub struct Prime7;
impl Modulus for Prime7 {
    fn modulus() -> U256 {
        U256::from_u32(7u32)
    }
}

#[derive(Debug)]
pub struct Prime31;
impl Modulus for Prime31 {
    fn modulus() -> U256 {
        U256::from_u32(31u32)
    }
}

#[cfg(test)]
mod field_element_basic_tests {
    use crate::s256::s256_field::S256Field;
    use super::*;

    #[test]
    fn init_num_valid() {
        type TestField7 = FieldElement<Prime7>;
        type TestField31 = FieldElement<Prime31>;

        let n2: U256 = U256::from(2u32);
        let n4: U256 = U256::from(4u32);
        let fe_7_2 = TestField7::new(n2);
        assert_eq!(fe_7_2.num, n2);

        let fe_31_4= TestField31::new(n4);
        assert_eq!(fe_31_4.num, n4);
    }

    #[test]
    fn init_valid() {
        let gx: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let gy: U256 = U256::from_be_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let fe_gx_p = S256Field::new(gx);
        assert_eq!(fe_gx_p.num, gx);

        let fe_gy_p = S256Field::new(gy);
        assert_eq!(fe_gy_p.num, gy);
    }


    #[test]
    #[should_panic(expected = "not in field range")]
    fn init_invalid() {
        let n: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF2F");
        S256Field::new(n);
    }

    #[test]
    fn eq() {
        let gx: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let fe1 = S256Field::new(gx);
        let fe2 = S256Field::new(gx);
        assert_eq!(fe1, fe2);
        assert!(fe1 == fe2);
    }

    #[test]
    fn neq() {
        let gx: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let gy: U256 = U256::from_be_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let fe1 = S256Field::new(gx);
        let fe2 = S256Field::new(gy);
        assert_ne!(fe1, fe2);
        assert!(fe1 != fe2);
    }
}

#[cfg(test)]
mod field_element_ops_tests {
    use crate::s256::s256_field::S256Field;

    use super::*;

    #[test]
    fn add_valid() {
        let x1: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let x2: U256 = U256::from(10u16);
        let y1: U256 = U256::from_be_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let fe_x1 = S256Field::new(x1);
        let fe_x2 = S256Field::new(x2);
        let fe_y1 = S256Field::new(y1);

        let x3: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F817A2");
        let sum1 = fe_x1 + fe_x2;
        assert_eq!(sum1.num, x3);

        let sum2_num: U256 = U256::from_be_hex("c1f940f620808011b3455e91dc9813afffb3b123d4537cf2f63a51eb1208ec50");
        let sum2 = fe_x1 + fe_y1;
        assert_eq!(sum2.num, sum2_num);
    }

    #[test]
    fn sub_valid() {
        let x1: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let x2: U256 = U256::from(10u16);
        let y1: U256 = U256::from_be_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let fe_x1 = S256Field::new(x1);
        let fe_x2 = S256Field::new(x2);
        let fe_y1 = S256Field::new(y1);

        let x3: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F8178E");
        let sub1 = fe_x1 - fe_x2;
        assert_eq!(sub1.num, x3);

        let sub2_num: U256 = U256::from_be_hex("31838c07d338f746f7fb6699c076025e058448928748d4bfbdaab0cb1be742e0");
        let sub2 = fe_x1 - fe_y1;
        assert_eq!(sub2.num, sub2_num);
    }

    #[test]
    fn mul_valid() {
        let x1: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let x2: U256 = U256::from(10u32);
        let y1: U256 = U256::from_be_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let fe_x1 = S256Field::new(x1);
        let fe_x2 = S256Field::new(x2);
        let fe_y1 = S256Field::new(y1);

        let res1_num: U256 = U256::from_be_hex("c17000f5c29f54bb5843d9da11466e461a17e08fca0d987d83790d92e5b0fb34");
        let res1 = fe_x1 * fe_x2;
        assert_eq!(res1.num, res1_num);

        let res2_num: U256 = U256::from_be_hex("fd3dc529c6eb60fb9d166034cf3c1a5a72324aa9dfd3428a56d7e1ce0179fd9b");
        let res2 = fe_x1 * fe_y1;
        assert_eq!(res2.num, res2_num);
    }

    // Discrete log problem
    /*
    #[test]
    fn div_valid() {
    }
    */

}