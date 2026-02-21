use std::ops;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct PointNum {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub a: i32,
    pub b: i32,
}

impl PointNum {
    pub fn new(x: Option<i32>, y: Option<i32>, a: i32, b: i32) -> Self {
        if x == None || y == None {
            assert_eq!(x, y, "For Point at Infinity, both points should be None");
            Self { x: None, y: None, a, b}
        } else {
            let _x = x.unwrap();
            let _y = y.unwrap();
            assert_eq!(_y.pow(2), _x.pow(3) + a * _x + b, "({}, {}) is not on the curve", _x, _y);
            Self {
                x, y, a, b
            }
        }
    }

    pub fn new_concrete(x: i32, y: i32, a: i32, b: i32) -> Self {
        Self::new(Some(x), Some(y), a, b)
    }

    pub fn new_inf(a: i32, b:i32) -> Self {
        Self::new(None, None, a, b)
    }
}

impl ops::Add for PointNum {
    type Output = PointNum;

    fn add(self, other: PointNum) -> PointNum {
        assert!(self.a == other.a && self.b == other.b, "Points are not in the same curve");

        if self.x == None {
            return other;
        } else if other.x == None {
            return self;
        } else if self.x == other.x && self.y != other.y {
            return PointNum::new_inf(self.a, self.b);
        } else if self.x != other.x {
            let x1 = self.x.unwrap() as f64;
            let y1 = self.y.unwrap() as f64;
            let x2 = other.x.unwrap() as f64;
            let y2 = other.y.unwrap() as f64;

            let s = (y2 - y1)/(x2 - x1);
            
            let x3 = (s * s) - x1 - x2;
            let y3 =  s * (x1 - x3) - y1;

            PointNum::new_concrete(x3 as i32, y3 as i32, self.a, self.b)
        } else if self == other && self.y == Some(0) {
            return PointNum::new_inf(self.a, self.b);
        } else if self == other && self.y != Some(0) {
            let x = self.x.unwrap() as f64;
            let y = self.y.unwrap() as f64;
            let a = self.a as f64;

            let s = (3.0 * x * x + a)/(2.0 * y);
            
            let x3 = (s * s) - (2.0 * x);
            let y3 = (s * (x - x3)) - y;

            PointNum::new_concrete(x3 as i32, y3 as i32, self.a, self.b)
        } else {
            // Should never come here.
            self
        }
    }

}

#[cfg(test)]
mod point_num_tests_init {
    use super::*;

    #[test]
    #[should_panic(expected = "both points should be None")]
    fn init_inf_invalid_1() {
        PointNum::new(None, Some(1), 5, 7);
    }

    #[test]
    #[should_panic(expected = "both points should be None")]
    fn init_inf_invalid_2() {
        PointNum::new(Some(1), None, 5, 7);
    }

    #[test]
    #[should_panic(expected = "is not on the curve")]
    fn init_invald1() {
        let _ = PointNum::new_concrete(2, 4, 5, 7);
    }

    #[test]
    #[should_panic(expected = "is not on the curve")]
    fn init_invald2() {
        let _ = PointNum::new_concrete(5, 7, 5, 7);
    }

    #[test]
    fn init_inf() {
        let p_inf = PointNum::new(None, None, 5, 7);
        assert_eq!(p_inf.x, None);
        assert_eq!(p_inf.y, None);
    }

    #[test]
    fn init_valid() {
        let p_m1_m1 = PointNum::new_concrete(-1, -1, 5, 7);
        assert_eq!(p_m1_m1.x, Some(-1));
        assert_eq!(p_m1_m1.y, Some(-1));
        let p_18_77 = PointNum::new_concrete(18, 77, 5, 7);
        assert_eq!(p_18_77.x, Some(18));
        assert_eq!(p_18_77.y, Some(77));
    }

    #[test]
    fn eq() {
        let point_18_77a = PointNum::new_concrete(18, 77, 5, 7);
        let point_18_77b = PointNum::new_concrete(18, 77, 5, 7);
        assert_eq!(point_18_77a, point_18_77b);
        assert!(point_18_77a == point_18_77b);
    }

    #[test]
    fn neq() {
        let point_18_77a = PointNum::new_concrete(18, 77, 5, 7);
        let point_18_77b = PointNum::new_concrete(-1, -1, 5, 7);
        assert_ne!(point_18_77a, point_18_77b);
        assert!(point_18_77a != point_18_77b);
    }
}

#[cfg(test)]
mod point_num_tests_add {
    use super::*;

    #[test]
    #[should_panic(expected = "Points are not in the same curve")]
    fn add_not_same_curve() {
        let p1 = PointNum::new_concrete(18, 77, 5, 7);
        let p2 = PointNum::new_concrete(0, 1, -1, 1);
        let _ = p1 + p2;
    }

    #[test]
    fn add_identity() {
        let p1 = PointNum::new_concrete(18, 77, 5, 7);
        let p2 = PointNum::new_inf(5, 7);

        let res1 = p1 + p2;
        assert_eq!(res1.x, Some(18));
        assert_eq!(res1.y, Some(77));

        let res2 = p2 + p1;
        assert_eq!(res2.x, Some(18));
        assert_eq!(res2.y, Some(77));
    }

    #[test]
    fn add_inverse() {
        let p1 = PointNum::new_concrete(18, 77, 5, 7);
        let p2 = PointNum::new_concrete(18, -77, 5, 7);
        let res = p1 + p2;
        assert_eq!(res.x, None);
        assert_eq!(res.y, None);
    }

    #[test]
    fn add_commute() {
        let p1 = PointNum::new_concrete(-1, -1, 5, 7);
        let p2 = PointNum::new_concrete(2, 5, 5, 7);
        let res1 = p1 + p2;
        let res2 = p2 + p1;
        assert_eq!(res1.x, res2.x);
        assert_eq!(res1.y, res2.y);
    }

    #[test]
    fn add_different_points() {
        let p1 = PointNum::new_concrete(-1, -1, 5, 7);
        let p2 = PointNum::new_concrete(2, 5, 5, 7);
        let res = p1 + p2;
        assert_eq!(res.x, Some(3));
        assert_eq!(res.y, Some(-7));
    }

    #[test]
    fn add_same_points_y_0() {
        let p1 = PointNum::new_concrete(1, 0, -1, 0);
        let p2 = PointNum::new_concrete(1, 0, -1, 0);
        let res = p1 + p2;
        assert_eq!(res.x, None);
        assert_eq!(res.y, None);
    }

    #[test]
    fn add_same_points_y_non_0() {
        let p1 = PointNum::new_concrete(-1, -1, 5, 7);
        let p2 = PointNum::new_concrete(-1, -1, 5, 7);
        let res = p1 + p2;
        assert_eq!(res.x, Some(18));
        assert_eq!(res.y, Some(77));
    }





}