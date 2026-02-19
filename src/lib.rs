use std::ops;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct FieldElement {
    pub num: u32,
    pub prime: u32
}

impl FieldElement {
    pub fn new(num: u32, prime: u32) -> Self {
        assert!(num < prime, "Num {} not in field range 0 to {}", num, prime - 1);
        Self {
            num,
            prime
        }
    }

    pub fn pow(self, exponent: i32) -> FieldElement {
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

        FieldElement {
            num,
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
        let num = (self.num + self.prime - other.num) % self.prime;
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
        let num = (self.num * other.num) % self.prime;
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
        let divisor = other.pow((other.prime - 2) as i32);
        let num = (self.num * divisor.num) % self.prime;
        FieldElement {
            num,
            prime: self.prime
        }
    }

}

#[cfg(test)]
mod field_element_tests {
    use super::*;

    #[test]
    fn fe_init_valid() {
        let fe_7_2 = FieldElement::new(2, 7);
        assert_eq!(fe_7_2.num, 2);
        assert_eq!(fe_7_2.prime, 7);

        let fe_31_4 = FieldElement::new(4, 31);
        assert_eq!(fe_31_4.num, 4);
        assert_eq!(fe_31_4.prime, 31);
    }

    #[test]
    #[should_panic(expected = "not in field range")]
    fn fe_init_invalid1() {
        FieldElement::new(7, 7);
    }

    #[test]
    #[should_panic(expected = "not in field range")]
    fn fe_init_invalid2() {
        FieldElement::new(10, 7);
    }

    #[test]
    fn fe_eq() {
        let fe_7_2a = FieldElement::new(2, 7);
        let fe_7_2b = FieldElement::new(2, 7);
        assert_eq!(fe_7_2a, fe_7_2b);
        assert!(fe_7_2a == fe_7_2b);
    }

    #[test]
    fn fe_neq() {
        let fe_7_2 = FieldElement::new(2, 7);
        let fe_7_4 = FieldElement::new(4, 7);
        assert_ne!(fe_7_2, fe_7_4);
        assert!(fe_7_2 != fe_7_4);
    }

    // TODO: Do for bigger fields and numbers
    #[test]
    fn fe_add_valid() {
        let fe_7_2 = FieldElement::new(2, 7);
        let fe_7_3 = FieldElement::new(3, 7);
        let fe_7_5 = FieldElement::new(5, 7);
        let fe_7_6 = FieldElement::new(6, 7);

        let fe_2p3 = fe_7_2 + fe_7_3;
        assert_eq!(fe_2p3.num, 5);
        assert_eq!(fe_2p3.prime, 7);

        let fe_5p6 = fe_7_5 + fe_7_6;
        assert_eq!(fe_5p6.num, 4);
        assert_eq!(fe_5p6.prime, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn fe_add_invalid() {
        let fe_7_2 = FieldElement::new(2, 7);
        let fe_8_2 = FieldElement::new(2, 8);

        let _ = fe_7_2 + fe_8_2;
    }

    // TODO: Do for bigger fields and numbers
    #[test]
    fn fe_mul_valid() {
        let fe_7_2 = FieldElement::new(2, 7);
        let fe_7_3 = FieldElement::new(3, 7);
        let fe_7_5 = FieldElement::new(5, 7);
        let fe_7_6 = FieldElement::new(6, 7);

        let fe_3m2 = fe_7_3 * fe_7_2;
        assert_eq!(fe_3m2.num, 6);
        assert_eq!(fe_3m2.prime, 7);

        let fe_5m6 = fe_7_5 * fe_7_6;
        assert_eq!(fe_5m6.num, 2);
        assert_eq!(fe_5m6.prime, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn fe_mul_invalid() {
        let fe_7_2 = FieldElement::new(2, 7);
        let fe_8_2 = FieldElement::new(2, 8);

        let _ = fe_7_2 * fe_8_2;
    }

    // TODO: Do for bigger fields and numbers
    #[test]
    fn fe_div_valid() {
        let fe_7_2 = FieldElement::new(2, 7);
        let fe_7_3 = FieldElement::new(3, 7);
        let fe_7_5 = FieldElement::new(5, 7);
        let fe_7_6 = FieldElement::new(6, 7);

        let fe_3d2 = fe_7_3 / fe_7_2;
        assert_eq!(fe_3d2.num, 5);
        assert_eq!(fe_3d2.prime, 7);

        let fe_5d6 = fe_7_5 / fe_7_6;
        assert_eq!(fe_5d6.num, 2);
        assert_eq!(fe_5d6.prime, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot operate between different fields")]
    fn fe_div_invalid() {
        let fe_7_2 = FieldElement::new(2, 7);
        let fe_8_2 = FieldElement::new(2, 8);

        let _ = fe_7_2 / fe_8_2;
    }

}
