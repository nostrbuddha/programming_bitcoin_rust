use std::ops;

// Chapter 1
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct FieldElementU32 {
    pub num: u32,
    pub prime: u32
}

impl FieldElementU32 {
    pub fn new(num: u32, prime: u32) -> Self {
        assert!(num < prime, "Num {} not in field range 0 to {}", num, prime - 1);
        Self { num, prime }
    }

    pub fn pow(self, mut exp: i32) -> FieldElementU32 {
        let p = self.prime as i32;
        exp %= p - 1;

        if exp < 0 {
            exp += p - 1;
        }

        let mut result = 1u32;
        let mut base = self.num;
        let mut e = exp as u32;

        while e > 0 {
            if e & 1 == 1 {
                result = (result * base) % self.prime;
            }
            base = (base * base) % self.prime;
            e >>= 1;
        }

        FieldElementU32 { num: result, prime: self.prime }
    }

    pub fn inv(self) -> Self {
        assert!(self.num != 0, "Cannot invert zero");
        self.pow((self.prime - 2) as i32)
    }
}

impl ops::Add for FieldElementU32 {
    type Output = FieldElementU32;

    fn add(self, other: FieldElementU32) -> FieldElementU32 {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");
        let num = (self.num + other.num) % self.prime;
        FieldElementU32 {
            num,
            prime: self.prime
        }
    }

}

impl ops::Sub for FieldElementU32 {
    type Output = FieldElementU32;

    fn sub(self, other: FieldElementU32) -> FieldElementU32 {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");
        let num = (self.num + self.prime - other.num) % self.prime;
        FieldElementU32 {
            num,
            prime: self.prime
        }
    }

}

impl ops::Mul for FieldElementU32 {
    type Output = FieldElementU32;

    fn mul(self, other: FieldElementU32) -> FieldElementU32 {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");
        let num = (self.num * other.num) % self.prime;
        FieldElementU32 {
            num,
            prime: self.prime
        }
    }

}

impl ops::Div for FieldElementU32 {
    type Output = FieldElementU32;

    fn div(self, other: FieldElementU32) -> FieldElementU32 {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");
        self * other.inv()
    }

}

#[cfg(test)]
mod field_element_u32_basic_tests {
    use crate::fe32;

    #[test]
    fn init_valid() {
        let fe_7_2 = fe32!(2, 7);
        assert_eq!(fe_7_2.num, 2);
        assert_eq!(fe_7_2.prime, 7);

        let fe_31_4 = fe32!(4, 31);
        assert_eq!(fe_31_4.num, 4);
        assert_eq!(fe_31_4.prime, 31);
    }

    #[test]
    #[should_panic(expected = "not in field range")]
    fn init_invalid1() {
        fe32!(7, 7);
    }

    #[test]
    #[should_panic(expected = "not in field range")]
    fn init_invalid2() {
        fe32!(10, 7);
    }

    #[test]
    fn eq() {
        let fe_7_2a = fe32!(2, 7);
        let fe_7_2b = fe32!(2, 7);
        assert_eq!(fe_7_2a, fe_7_2b);
        assert!(fe_7_2a == fe_7_2b);
    }

    #[test]
    fn neq() {
        let fe_7_2 = fe32!(2, 7);
        let fe_7_4 = fe32!(4, 7);
        assert_ne!(fe_7_2, fe_7_4);
        assert!(fe_7_2 != fe_7_4);
    }
}

#[cfg(test)]
mod field_element_u32_ops_tests {
    use crate::fe32;

    #[test]
    fn add_valid() {
        let fe_7_2 = fe32!(2, 7);
        let fe_7_3 = fe32!(3, 7);
        let fe_7_5 = fe32!(5, 7);
        let fe_7_6 = fe32!(6, 7);

        let fe_2p3 = fe_7_2 + fe_7_3;
        assert_eq!(fe_2p3.num, 5);
        assert_eq!(fe_2p3.prime, 7);

        let fe_5p6 = fe_7_5 + fe_7_6;
        assert_eq!(fe_5p6.num, 4);
        assert_eq!(fe_5p6.prime, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn add_invalid() {
        let fe_7_2 = fe32!(2, 7);
        let fe_8_2 = fe32!(2, 8);

        let _ = fe_7_2 + fe_8_2;
    }


    #[test]
    fn sub_valid() {
        let fe_7_2 = fe32!(2, 7);
        let fe_7_3 = fe32!(3, 7);
        let fe_7_5 = fe32!(5, 7);
        let fe_7_6 = fe32!(6, 7);

        let fe_2s3 = fe_7_2 - fe_7_3;
        assert_eq!(fe_2s3.num, 6);
        assert_eq!(fe_2s3.prime, 7);

        let fe_5s6 = fe_7_5 - fe_7_6;
        assert_eq!(fe_5s6.num, 6);
        assert_eq!(fe_5s6.prime, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn sub_invalid() {
        let fe_7_2 = fe32!(2, 7);
        let fe_8_2 = fe32!(2, 8);

        let _ = fe_7_2 - fe_8_2;
    }

    #[test]
    fn mul_valid() {
        let fe_7_2 = fe32!(2, 7);
        let fe_7_3 = fe32!(3, 7);
        let fe_7_5 = fe32!(5, 7);
        let fe_7_6 = fe32!(6, 7);

        let fe_3m2 = fe_7_3 * fe_7_2;
        assert_eq!(fe_3m2.num, 6);
        assert_eq!(fe_3m2.prime, 7);

        let fe_5m6 = fe_7_5 * fe_7_6;
        assert_eq!(fe_5m6.num, 2);
        assert_eq!(fe_5m6.prime, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn mul_invalid() {
        let fe_7_2 = fe32!(2, 7);
        let fe_8_2 = fe32!(2, 8);

        let _ = fe_7_2 * fe_8_2;
    }

    #[test]
    fn div_valid() {
        let fe_7_2 = fe32!(2, 7);
        let fe_7_3 = fe32!(3, 7);
        let fe_7_5 = fe32!(5, 7);
        let fe_7_6 = fe32!(6, 7);

        let fe_3d2 = fe_7_3 / fe_7_2;
        assert_eq!(fe_3d2.num, 5);
        assert_eq!(fe_3d2.prime, 7);

        let fe_5d6 = fe_7_5 / fe_7_6;
        assert_eq!(fe_5d6.num, 2);
        assert_eq!(fe_5d6.prime, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn div_invalid() {
        let fe_7_2 = fe32!(2, 7);
        let fe_8_2 = fe32!(2, 8);

        let _ = fe_7_2 / fe_8_2;
    }

    #[test]
    fn pow_valid() {
        let fe_7_2 = fe32!(2, 7);
        let fe_7_6 = fe32!(6, 7);

        let fe1 = fe_7_2.pow(3);
        assert_eq!(fe1.num, 1);
        assert_eq!(fe1.prime, 7);

        let fe2 = fe_7_2.pow(4);
        assert_eq!(fe2.num, 2);
        assert_eq!(fe2.prime, 7);

        let fe3 = fe_7_6.pow(3);
        assert_eq!(fe3.num, 6);
        assert_eq!(fe3.prime, 7);

        let fe4 = fe_7_6.pow(4);
        assert_eq!(fe4.num, 1);
        assert_eq!(fe4.prime, 7);
    }

    #[test]
    fn pow_valid_negative() {
        let fe_7_2 = fe32!(2, 7);
        let fe_7_6 = fe32!(6, 7);

        let fe1 = fe_7_2.pow(-3);
        assert_eq!(fe1.num, 1);
        assert_eq!(fe1.prime, 7);

        let fe2 = fe_7_2.pow(-4);
        assert_eq!(fe2.num, 4);
        assert_eq!(fe2.prime, 7);

        let fe3 = fe_7_6.pow(-3);
        assert_eq!(fe3.num, 6);
        assert_eq!(fe3.prime, 7);

        let fe4 = fe_7_6.pow(-4);
        assert_eq!(fe4.num, 1);
        assert_eq!(fe4.prime, 7);
    }

}