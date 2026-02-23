use std::ops;
use crypto_bigint::U256;
use crypto_bigint::I256;
use crypto_bigint::NonZero;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct FieldElement {
    pub num: U256,
    pub prime: U256
}

impl FieldElement {
    pub fn new(num: U256, prime: U256) -> Self {
        assert!(num < prime, "Num {} not in field range 0 to {}", num, prime - U256::ONE);
        Self {
            num,
            prime
        }
    }

    pub fn pow(self, exponent: I256) -> FieldElement {
        let mut num: U256 = U256::ONE;
        let order: U256 = self.prime - U256::ONE;

        let mut exp: U256 = if exponent >= I256::ZERO {
            exponent.as_uint() % order
        } else {
            let neg: U256 = exponent.wrapping_neg().as_uint() % order;
            (order - neg) % order
        };

        loop {
            if exp > U256::ZERO {
                num = (num * self.num) % self.prime;
                exp -= U256::ONE;
            } else {
                break;
            }
        }

        FieldElement {
            num: num,
            prime: self.prime
        }
    }
}

impl ops::Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");
        let num = (self.num + other.num) % self.prime;
        FieldElement {
            num,
            prime: self.prime
        }
    }

}

impl ops::Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");
        let num = if self.num >= other.num {
            self.num - other.num
        } else {
            self.prime - (other.num - self.num) % self.prime
        };
        FieldElement {
            num,
            prime: self.prime
        }
    }

}

impl ops::Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");

        let prime_nz = NonZero::new(self.prime).unwrap();
        let num = self.num.mul_mod(&other.num, &prime_nz);

        FieldElement {
            num,
            prime: self.prime
        }
    }

}

impl ops::Div for FieldElement {
    type Output = FieldElement;

    fn div(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");
        let exp = other.prime.as_int() - I256::from(2);
        let divisor = other.pow(exp);
        let num = (self.num * divisor.num) % self.prime;
        FieldElement {
            num,
            prime: self.prime
        }
    }

}

#[cfg(test)]
mod field_element_basic_tests {
    use super::*;

    #[test]
    fn init_num_valid() {
        let n2: U256 = U256::from(2u32);
        let n4: U256 = U256::from(4u32);
        let n7: U256 = U256::from(7u32);
        let n31: U256 = U256::from(31u32);
        let fe_7_2 = FieldElement::new(n2, n7);
        assert_eq!(fe_7_2.num, n2);
        assert_eq!(fe_7_2.prime, n7);

        let fe_31_4= FieldElement::new(n4, n31);
        assert_eq!(fe_31_4.num, n4);
        assert_eq!(fe_31_4.prime, n31);
    }

    #[test]
    fn init_valid() {
        let p: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let gx: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let gy: U256 = U256::from_be_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let fe_gx_p = FieldElement::new(gx, p);
        assert_eq!(fe_gx_p.num, gx);
        assert_eq!(fe_gx_p.prime, p);

        let fe_gy_p = FieldElement::new(gy, p);
        assert_eq!(fe_gy_p.num, gy);
        assert_eq!(fe_gy_p.prime, p);
    }


    #[test]
    #[should_panic(expected = "not in field range")]
    fn init_invalid() {
        let n: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF2F");
        let p: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        FieldElement::new(n, p);
    }

    #[test]
    fn eq() {
        let gx: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let p: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let fe1 = FieldElement::new(gx, p);
        let fe2 = FieldElement::new(gx, p);
        assert_eq!(fe1, fe2);
        assert!(fe1 == fe2);
    }

    #[test]
    fn neq() {
        let gx: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let gy: U256 = U256::from_be_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let p: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let fe1 = FieldElement::new(gx, p);
        let fe2 = FieldElement::new(gy, p);
        assert_ne!(fe1, fe2);
        assert!(fe1 != fe2);
    }
}

#[cfg(test)]
mod field_element_ops_tests {
    use super::*;
    #[test]
    fn add_valid() {
        let x1: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let x2: U256 = U256::from(10u16);
        let y1: U256 = U256::from_be_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let p: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let fe_x1 = FieldElement::new(x1, p);
        let fe_x2 = FieldElement::new(x2, p);
        let fe_y1 = FieldElement::new(y1, p);

        let x3: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F817A2");
        let sum1 = fe_x1 + fe_x2;
        assert_eq!(sum1.num, x3);
        assert_eq!(sum1.prime, p);

        let sum2_num: U256 = U256::from_be_hex("c1f940f620808011b3455e91dc9813afffb3b123d4537cf2f63a51eb1208ec50");
        let sum2 = fe_x1 + fe_y1;
        assert_eq!(sum2.num, sum2_num);
        assert_eq!(sum2.prime, p);

    }

    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn add_invalid() {
        let x1: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let p1: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let p2: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2A");
        let fe_x1_p1 = FieldElement::new(x1, p1);
        let fe_x1_p2 = FieldElement::new(x1, p2);

        let _ = fe_x1_p1 + fe_x1_p2;
    }


    #[test]
    fn sub_valid() {
        let x1: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let x2: U256 = U256::from(10u16);
        let y1: U256 = U256::from_be_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let p: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let fe_x1 = FieldElement::new(x1, p);
        let fe_x2 = FieldElement::new(x2, p);
        let fe_y1 = FieldElement::new(y1, p);

        let x3: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F8178E");
        let sub1 = fe_x1 - fe_x2;
        assert_eq!(sub1.num, x3);
        assert_eq!(sub1.prime, p);

        let sub2_num: U256 = U256::from_be_hex("31838c07d338f746f7fb6699c076025e058448928748d4bfbdaab0cb1be742e0");
        let sub2 = fe_x1 - fe_y1;
        assert_eq!(sub2.num, sub2_num);
        assert_eq!(sub2.prime, p);

    }

    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn sub_invalid() {
        let x1: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let p1: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let p2: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2A");
        let fe_x1_p1 = FieldElement::new(x1, p1);
        let fe_x1_p2 = FieldElement::new(x1, p2);

        let _ = fe_x1_p1 - fe_x1_p2;
    }

    #[test]
    fn mul_valid() {
        let x1: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let x2: U256 = U256::from(10u32);
        let y1: U256 = U256::from_be_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let p: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let fe_x1 = FieldElement::new(x1, p);
        let fe_x2 = FieldElement::new(x2, p);
        let fe_y1 = FieldElement::new(y1, p);

        let res1_num: U256 = U256::from_be_hex("c17000f5c29f54bb5843d9da11466e461a17e08fca0d987d83790d92e5b0fb34");
        let res1 = fe_x1 * fe_x2;
        assert_eq!(res1.num, res1_num);
        assert_eq!(res1.prime, p);

        let res2_num: U256 = U256::from_be_hex("fd3dc529c6eb60fb9d166034cf3c1a5a72324aa9dfd3428a56d7e1ce0179fd9b");
        let res2 = fe_x1 * fe_y1;
        assert_eq!(res2.num, res2_num);
        assert_eq!(res2.prime, p);
    }

    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn mul_invalid() {
        let x1: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let p1: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let p2: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2A");
        let fe_x1_p1 = FieldElement::new(x1, p1);
        let fe_x1_p2 = FieldElement::new(x1, p2);

        let _ = fe_x1_p1 * fe_x1_p2;
    }

    // Discrete log problem
    /*
    #[test]
    fn div_valid() {
        let x1: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let x2: U256 = U256::from(10u32);
        let y1: U256 = U256::from_be_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let p: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let fe_x1 = FieldElement::new(x1, p);
        let fe_x2 = FieldElement::new(x2, p);
        let fe_y1 = FieldElement::new(y1, p);

        let res1_num: U256 = U256::from_be_hex("0c2ca3d97f62df913bc33d0efb0d811a4d0f99491e2e3748ef650cef824c025c");
        let res1 = fe_x1 / fe_x2;
        assert_eq!(res1.num, res1_num);
        assert_eq!(res1.prime, p);

        let res2_num: U256 = U256::from_be_hex("2db7da16ef4bd6e01dfaad38c11521cbc90dda6ded1975fc41895c5d541f5127");
        let res2 = fe_x1 / fe_y1;
        assert_eq!(res2.num, res2_num);
        assert_eq!(res2.prime, p);
    }
    */

    /*
    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn div_invalid() {
        let x1: U256 = U256::from_be_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let p1: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let p2: U256 = U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2A");
        let fe_x1_p1 = FieldElement::new(x1, p1);
        let fe_x1_p2 = FieldElement::new(x1, p2);

        let _ = fe_x1_p1 / fe_x1_p2;
    }
    */

}