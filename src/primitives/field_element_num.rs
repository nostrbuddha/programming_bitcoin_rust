use std::ops;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct FieldElementNum {
    pub num: u32,
    pub prime: u32
}

impl FieldElementNum {
    pub fn new(num: u32, prime: u32) -> Self {
        assert!(num < prime, "Num {} not in field range 0 to {}", num, prime - 1);
        Self {
            num,
            prime
        }
    }

    pub fn pow(self, exponent: i32) -> FieldElementNum {
        let mut num = 1;
        let mut multi = if exponent > 0 {
            exponent
        } else {
            (self.prime as i32) + exponent - 1
        };

        loop {
            if multi > 0 {
                num = (num * self.num) % self.prime;
                multi -= 1;
            } else {
                break;
            }
        }

        FieldElementNum {
            num,
            prime: self.prime
        }
    }
}

impl ops::Add for FieldElementNum {
    type Output = FieldElementNum;

    fn add(self, other: FieldElementNum) -> FieldElementNum {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");
        let num = (self.num + other.num) % self.prime;
        FieldElementNum {
            num,
            prime: self.prime
        }
    }

}

impl ops::Sub for FieldElementNum {
    type Output = FieldElementNum;

    fn sub(self, other: FieldElementNum) -> FieldElementNum {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");
        let num = (self.num + self.prime - other.num) % self.prime;
        FieldElementNum {
            num,
            prime: self.prime
        }
    }

}

impl ops::Mul for FieldElementNum {
    type Output = FieldElementNum;

    fn mul(self, other: FieldElementNum) -> FieldElementNum {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");
        let num = (self.num * other.num) % self.prime;
        FieldElementNum {
            num,
            prime: self.prime
        }
    }

}

impl ops::Div for FieldElementNum {
    type Output = FieldElementNum;

    fn div(self, other: FieldElementNum) -> FieldElementNum {
        assert_eq!(self.prime, other.prime, "Cannot operate between different fields");
        let divisor = other.pow((other.prime - 2) as i32);
        let num = (self.num * divisor.num) % self.prime;
        FieldElementNum {
            num,
            prime: self.prime
        }
    }

}

#[cfg(test)]
mod field_element_num_basic_tests {
    use super::*;

    #[test]
    fn init_valid() {
        let fe_7_2 = FieldElementNum::new(2, 7);
        assert_eq!(fe_7_2.num, 2);
        assert_eq!(fe_7_2.prime, 7);

        let fe_31_4 = FieldElementNum::new(4, 31);
        assert_eq!(fe_31_4.num, 4);
        assert_eq!(fe_31_4.prime, 31);
    }

    #[test]
    #[should_panic(expected = "not in field range")]
    fn init_invalid1() {
        FieldElementNum::new(7, 7);
    }

    #[test]
    #[should_panic(expected = "not in field range")]
    fn init_invalid2() {
        FieldElementNum::new(10, 7);
    }

    #[test]
    fn eq() {
        let fe_7_2a = FieldElementNum::new(2, 7);
        let fe_7_2b = FieldElementNum::new(2, 7);
        assert_eq!(fe_7_2a, fe_7_2b);
        assert!(fe_7_2a == fe_7_2b);
    }

    #[test]
    fn neq() {
        let fe_7_2 = FieldElementNum::new(2, 7);
        let fe_7_4 = FieldElementNum::new(4, 7);
        assert_ne!(fe_7_2, fe_7_4);
        assert!(fe_7_2 != fe_7_4);
    }
}

#[cfg(test)]
mod field_element_num_ops_tests {
    use super::*;

    #[test]
    fn add_valid() {
        let fe_7_2 = FieldElementNum::new(2, 7);
        let fe_7_3 = FieldElementNum::new(3, 7);
        let fe_7_5 = FieldElementNum::new(5, 7);
        let fe_7_6 = FieldElementNum::new(6, 7);

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
        let fe_7_2 = FieldElementNum::new(2, 7);
        let fe_8_2 = FieldElementNum::new(2, 8);

        let _ = fe_7_2 + fe_8_2;
    }


    #[test]
    fn sub_valid() {
        let fe_7_2 = FieldElementNum::new(2, 7);
        let fe_7_3 = FieldElementNum::new(3, 7);
        let fe_7_5 = FieldElementNum::new(5, 7);
        let fe_7_6 = FieldElementNum::new(6, 7);

        let fe_2p3 = fe_7_2 - fe_7_3;
        assert_eq!(fe_2p3.num, 6);
        assert_eq!(fe_2p3.prime, 7);

        let fe_5p6 = fe_7_5 - fe_7_6;
        assert_eq!(fe_5p6.num, 6);
        assert_eq!(fe_5p6.prime, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn sub_invalid() {
        let fe_7_2 = FieldElementNum::new(2, 7);
        let fe_8_2 = FieldElementNum::new(2, 8);

        let _ = fe_7_2 - fe_8_2;
    }

    #[test]
    fn mul_valid() {
        let fe_7_2 = FieldElementNum::new(2, 7);
        let fe_7_3 = FieldElementNum::new(3, 7);
        let fe_7_5 = FieldElementNum::new(5, 7);
        let fe_7_6 = FieldElementNum::new(6, 7);

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
        let fe_7_2 = FieldElementNum::new(2, 7);
        let fe_8_2 = FieldElementNum::new(2, 8);

        let _ = fe_7_2 * fe_8_2;
    }

    #[test]
    fn div_valid() {
        let fe_7_2 = FieldElementNum::new(2, 7);
        let fe_7_3 = FieldElementNum::new(3, 7);
        let fe_7_5 = FieldElementNum::new(5, 7);
        let fe_7_6 = FieldElementNum::new(6, 7);

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
        let fe_7_2 = FieldElementNum::new(2, 7);
        let fe_8_2 = FieldElementNum::new(2, 8);

        let _ = fe_7_2 / fe_8_2;
    }

}