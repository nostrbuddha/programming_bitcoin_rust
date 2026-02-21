use std::ops;
use crate::primitives::field_element_num::FieldElementNum;
use crate::fen;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct PointFeNum {
    pub x: Option<FieldElementNum>,
    pub y: Option<FieldElementNum>,
    pub a: FieldElementNum,
    pub b: FieldElementNum,
}

impl PointFeNum {
    pub fn new(x: Option<FieldElementNum>, y: Option<FieldElementNum>, a: FieldElementNum, b: FieldElementNum) -> Self {
        if x == None || y == None {
            assert_eq!(x, y, "For Point at Infinity, both points should be None");
            Self { x: None, y: None, a, b}
        } else {
            let _x = x.unwrap();
            let _y = y.unwrap();
            assert_eq!(_y.pow(2), _x.pow(3) + a * _x + b, "({:?}, {:?}) is not on the curve", _x, _y);
            Self {
                x, y, a, b
            }
        }
    }

    pub fn new_concrete(x: FieldElementNum, y: FieldElementNum, a: FieldElementNum, b: FieldElementNum) -> Self {
        Self::new(Some(x), Some(y), a, b)
    }

    pub fn new_inf(a: FieldElementNum, b:FieldElementNum) -> Self {
        Self::new(None, None, a, b)
    }

    pub fn smul(self, coefficient: i32) -> PointFeNum {
        let mut curr = self.clone();
        let mut res = Self::new_inf(self.a, self.b);
        let mut coef = coefficient;
        while coef > 0 {
            if coef & 1 > 0 {
                res = res + curr;
            }
            curr = curr + curr;
            coef >>= 1;
        }
        return res;
    }
}

impl ops::Add for PointFeNum {
    type Output = PointFeNum;

    fn add(self, other: PointFeNum) -> PointFeNum {
        assert!(self.a == other.a && self.b == other.b, "Points are not in the same curve");

        let zero = FieldElementNum::new(0, self.a.prime);

        if self.x == None {
            return other;
        } else if other.x == None {
            return self;
        } else if self.x == other.x && self.y != other.y {
            return PointFeNum::new_inf(self.a, self.b);
        } else if self.x != other.x {
            let x1 = self.x.unwrap();
            let y1 = self.y.unwrap();
            let x2 = other.x.unwrap();
            let y2 = other.y.unwrap();

            let s = (y2 - y1)/(x2 - x1);
            
            let x3 = (s * s) - x1 - x2;
            let y3 =  s * (x1 - x3) - y1;

            PointFeNum::new_concrete(x3, y3, self.a, self.b)
        } else if self == other && self.y == Some(zero) {
            return PointFeNum::new_inf(self.a, self.b);
        } else if self == other && self.y != Some(zero) {
            let x = self.x.unwrap();
            let y = self.y.unwrap();
            let a = self.a;

            let fe_3 = FieldElementNum::new(3, x.prime);
            let fe_2 = FieldElementNum::new(2, x.prime);

            let s = (fe_3 * x * x + a)/(fe_2 * y);
            
            let x3 = (s * s) - (fe_2 * x);
            let y3 = (s * (x - x3)) - y;

            PointFeNum::new_concrete(x3, y3, self.a, self.b)
        } else {
            // Should never come here.
            self
        }
    }
}

#[cfg(test)]
mod point_fe_num_tests_init {
    use super::*;

    #[test]
    #[should_panic(expected = "both points should be None")]
    fn init_inf_invalid_1() {
        PointFeNum::new(None, Some(fen!(1, 31)), fen!(5, 31), fen!(7, 31));
    }

    #[test]
    #[should_panic(expected = "both points should be None")]
    fn init_inf_invalid_2() {
        PointFeNum::new(Some(fen!(1, 31)), None, fen!(5, 31), fen!(7, 31));
    }

    #[test]
    #[should_panic(expected = "is not on the curve")]
    fn init_invald1() {
        let _ = PointFeNum::new_concrete(fen!(2, 31), fen!(4, 31), fen!(5, 31), fen!(7, 31));
    }

    #[test]
    #[should_panic(expected = "is not on the curve")]
    fn init_invald2() {
        let _ = PointFeNum::new_concrete(fen!(5, 31), fen!(7, 31), fen!(5, 31), fen!(7, 31));
    }

    #[test]
    fn init_inf() {
        let p_inf = PointFeNum::new(None, None, fen!(5, 31), fen!(7, 31));
        assert_eq!(p_inf.x, None);
        assert_eq!(p_inf.y, None);
    }

    #[test]
    fn init_valid() {
        let fe0 = fen!(0, 223);
        let fe1 = fen!(1, 223);
        let fe7 = fen!(7, 223);
        let fe17 = fen!(17, 223);
        let fe56 = fen!(56, 223);
        let fe193 = fen!(193, 223);
        let p1 = PointFeNum::new_concrete(fe17, fe56, fe0, fe7);
        assert_eq!(p1.x, Some(fe17));
        assert_eq!(p1.y, Some(fe56));
        let p2 = PointFeNum::new_concrete(fe1, fe193, fe0, fe7);
        assert_eq!(p2.x, Some(fe1));
        assert_eq!(p2.y, Some(fe193));
    }

    #[test]
    fn eq() {
        let fe5 = fen!(5, 223);
        let fe7 = fen!(7, 223);
        let fe18 = fen!(18, 223);
        let fe77 = fen!(77, 223);
        let p1 = PointFeNum::new_concrete(fe18, fe77, fe5, fe7);
        let p2 = PointFeNum::new_concrete(fe18, fe77, fe5, fe7);
        assert_eq!(p1, p2);
        assert!(p1 == p2);
    }

    #[test]
    fn neq() {
        let fe0 = fen!(0, 223);
        let fe1 = fen!(1, 223);
        let fe7 = fen!(7, 223);
        let fe17 = fen!(17, 223);
        let fe56 = fen!(56, 223);
        let fe193 = fen!(193, 223);
        let p1 = PointFeNum::new_concrete(fe17, fe56, fe0, fe7);
        let p2 = PointFeNum::new_concrete(fe1, fe193, fe0, fe7);
        assert_ne!(p1, p2);
        assert!(p1 != p2);
    }
}

#[cfg(test)]
mod point_fe_num_tests_add {
    use super::*;

    // TODO:
}