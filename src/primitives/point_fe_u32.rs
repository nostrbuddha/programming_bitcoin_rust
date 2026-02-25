use std::ops;
use crate::primitives::field_element_u32::FieldElementU32;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct PointFEU32 {
    pub x: Option<FieldElementU32>,
    pub y: Option<FieldElementU32>,
    pub a: FieldElementU32,
    pub b: FieldElementU32,
}

impl PointFEU32 {
    pub fn new(x: Option<FieldElementU32>, y: Option<FieldElementU32>, a: FieldElementU32, b: FieldElementU32) -> Self {
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

    pub fn new_concrete(x: FieldElementU32, y: FieldElementU32, a: FieldElementU32, b: FieldElementU32) -> Self {
        Self::new(Some(x), Some(y), a, b)
    }

    pub fn new_inf(a: FieldElementU32, b:FieldElementU32) -> Self {
        Self::new(None, None, a, b)
    }

    // TODO: To handle negative multipliers
    pub fn smul(self, mut s: i32) -> PointFEU32 {
        let mut curr = self.clone();
        let mut res = Self::new_inf(self.a, self.b);
        while s > 0 {
            if s & 1 > 0 {
                res = res + curr;
            }
            curr = curr + curr;
            s >>= 1;
        }
        return res;
    }
}

impl ops::Add for PointFEU32 {
    type Output = PointFEU32;

    fn add(self, other: PointFEU32) -> PointFEU32 {
        assert!(self.a == other.a && self.b == other.b, "Points are not in the same curve");

        let zero = FieldElementU32::new(0, self.a.prime);

        if self.x == None {
            return other;
        } else if other.x == None {
            return self;
        } else if self.x == other.x && self.y != other.y {
            return PointFEU32::new_inf(self.a, self.b);
        } else if self.x != other.x {
            let x1 = self.x.unwrap();
            let y1 = self.y.unwrap();
            let x2 = other.x.unwrap();
            let y2 = other.y.unwrap();

            let s = (y2 - y1)/(x2 - x1);
            
            let x3 = (s * s) - x1 - x2;
            let y3 =  s * (x1 - x3) - y1;

            PointFEU32::new_concrete(x3, y3, self.a, self.b)
        } else if self == other && self.y == Some(zero) {
            return PointFEU32::new_inf(self.a, self.b);
        } else if self == other && self.y != Some(zero) {
            let x = self.x.unwrap();
            let y = self.y.unwrap();
            let a = self.a;

            let fe_3 = FieldElementU32::new(3, x.prime);
            let fe_2 = FieldElementU32::new(2, x.prime);

            let s = (fe_3 * x * x + a)/(fe_2 * y);
            
            let x3 = (s * s) - (fe_2 * x);
            let y3 = (s * (x - x3)) - y;

            PointFEU32::new_concrete(x3, y3, self.a, self.b)
        } else {
            // Should never come here.
            self
        }
    }
}

#[cfg(test)]
mod point_fe_u32_tests_init {
    use crate::p_fe32;
    use crate::fe32;
    use super::*;

    #[test]
    #[should_panic(expected = "both points should be None")]
    fn init_inf_invalid_1() {
        PointFEU32::new(None, Some(fe32!(1, 31)), fe32!(5, 31), fe32!(7, 31));
    }

    #[test]
    #[should_panic(expected = "both points should be None")]
    fn init_inf_invalid_2() {
        PointFEU32::new(Some(fe32!(1, 31)), None, fe32!(5, 31), fe32!(7, 31));
    }

    #[test]
    #[should_panic(expected = "is not on the curve")]
    fn init_invald1() {
        let _ = p_fe32!(fe32!(2, 31), fe32!(4, 31), fe32!(5, 31), fe32!(7, 31));
    }

    #[test]
    #[should_panic(expected = "is not on the curve")]
    fn init_invald2() {
        let _ = p_fe32!(fe32!(5, 31), fe32!(7, 31), fe32!(5, 31), fe32!(7, 31));
    }

    #[test]
    fn init_inf() {
        let p_inf = PointFEU32::new(None, None, fe32!(5, 31), fe32!(7, 31));
        assert_eq!(p_inf.x, None);
        assert_eq!(p_inf.y, None);
    }

    #[test]
    fn init_valid() {
        let fe0 = fe32!(0, 223);
        let fe1 = fe32!(1, 223);
        let fe7 = fe32!(7, 223);
        let fe17 = fe32!(17, 223);
        let fe56 = fe32!(56, 223);
        let fe193 = fe32!(193, 223);
        let p1 = p_fe32!(fe17, fe56, fe0, fe7);
        assert_eq!(p1.x, Some(fe17));
        assert_eq!(p1.y, Some(fe56));
        let p2 = p_fe32!(fe1, fe193, fe0, fe7);
        assert_eq!(p2.x, Some(fe1));
        assert_eq!(p2.y, Some(fe193));
    }

    #[test]
    fn eq() {
        let fe5 = fe32!(5, 223);
        let fe7 = fe32!(7, 223);
        let fe18 = fe32!(18, 223);
        let fe77 = fe32!(77, 223);
        let p1 = p_fe32!(fe18, fe77, fe5, fe7);
        let p2 = p_fe32!(fe18, fe77, fe5, fe7);
        assert_eq!(p1, p2);
        assert!(p1 == p2);
    }

    #[test]
    fn neq() {
        let fe0 = fe32!(0, 223);
        let fe1 = fe32!(1, 223);
        let fe7 = fe32!(7, 223);
        let fe17 = fe32!(17, 223);
        let fe56 = fe32!(56, 223);
        let fe193 = fe32!(193, 223);
        let p1 = p_fe32!(fe17, fe56, fe0, fe7);
        let p2 = p_fe32!(fe1, fe193, fe0, fe7);
        assert_ne!(p1, p2);
        assert!(p1 != p2);
    }
}

#[cfg(test)]
mod point_fe_u32_tests_ops {
    use crate::p_fe32;
    use crate::fe32;

    #[test]
    fn smul2() {
        let fe0 = fe32!(0, 223);
        let fe7 = fe32!(7, 223);
        let fe47 = fe32!(47, 223);
        let fe71 = fe32!(71, 223);
        let p1 = p_fe32!(fe47, fe71, fe0, fe7);
        let res = p1.smul(2);
        assert_eq!(res.x, Some(fe32!(36, 223)));
        assert_eq!(res.y, Some(fe32!(111, 223)));
    }

    #[test]
    fn smul5() {
        let fe0 = fe32!(0, 223);
        let fe7 = fe32!(7, 223);
        let fe47 = fe32!(47, 223);
        let fe71 = fe32!(71, 223);
        let p1 = p_fe32!(fe47, fe71, fe0, fe7);
        let res = p1.smul(5);
        assert_eq!(res.x, Some(fe32!(126, 223)));
        assert_eq!(res.y, Some(fe32!(96, 223)));
    }


}

#[cfg(test)]
mod point_fe_u32_tests_add {
    use crate::fe32;
    use crate::p_fe32;
    use crate::primitives::point_fe_u32::PointFEU32;

    #[test]
    fn add_identity() {
        let fe0 = fe32!(0, 223);
        let fe7 = fe32!(7, 223);
        let fe47 = fe32!(47, 223);
        let fe71 = fe32!(71, 223);
        let p1 = p_fe32!(fe47, fe71, fe0, fe7);
        let p2 = PointFEU32::new(None, None, fe0, fe7);

        let sum1 = p1 + p2;
        assert_eq!(sum1.x, Some(fe47));
        assert_eq!(sum1.y, Some(fe71));

        let sum2 = p2 + p1;
        assert_eq!(sum2.x, Some(fe47));
        assert_eq!(sum2.y, Some(fe71));
    }

    #[test]
    fn add_inverse() {
        let fe0 = fe32!(0, 223);
        let fe7 = fe32!(7, 223);
        let fe36 = fe32!(36, 223);
        let fe111 = fe32!(111, 223);
        let fe112 = fe32!(112, 223);
        let p1 = p_fe32!(fe36, fe111, fe0, fe7);
        let p2 = p_fe32!(fe36, fe112, fe0, fe7);

        let sum1 = p1 + p2;
        assert_eq!(sum1.x, None);
        assert_eq!(sum1.y, None);

        let sum2 = p2 + p1;
        assert_eq!(sum2.x, None);
        assert_eq!(sum2.y, None);
    }

    #[test]
    fn add_different_points() {
        let fe0 = fe32!(0, 223);
        let fe7 = fe32!(7, 223);
        let fe47 = fe32!(47, 223);
        let fe71 = fe32!(71, 223);
        let fe36 = fe32!(36, 223);
        let fe111 = fe32!(111, 223);
        let p1 = p_fe32!(fe47, fe71, fe0, fe7);
        let p2 = p_fe32!(fe36, fe111, fe0, fe7);

        let sum1 = p1 + p2;
        assert_eq!(sum1.x, Some(fe32!(15, 223)));
        assert_eq!(sum1.y, Some(fe32!(137, 223)));

        let sum2 = p2 + p1;
        assert_eq!(sum2.x, Some(fe32!(15, 223)));
        assert_eq!(sum2.y, Some(fe32!(137, 223)));
    }
}