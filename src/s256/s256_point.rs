use std::ops;
use crypto_bigint::{I256, U256};

use crate::s256::{s256_field::S256Field, signature::Signature, scalar::Scalar};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct S256Point(Option<S256Field>, Option<S256Field>);

impl S256Point {
    pub fn new(x: Option<S256Field>, y: Option<S256Field>) -> Self {
        if x == None || y == None {
            assert_eq!(x, y, "For Point at Infinity, both points should be None");
            Self(None, None)
        } else {
            let _x = x.unwrap();
            let _y = y.unwrap();
            let two = I256::from(2);
            let three = I256::from(3);
            assert_eq!(_y.pow(two), _x.pow(three) + Self::a() * _x + Self::b(), "({:?}, {:?}) is not on the curve", _x, _y);
            Self(x, y)
        }
    }

    pub fn new_concrete(x: S256Field, y: S256Field) -> Self {
        Self::new(Some(x), Some(y))
    }

    pub fn x(self) -> Option<S256Field> {
        self.0
    }

    pub fn y(self) -> Option<S256Field> {
        self.1
    }

    pub fn rmul(self, coefficeint: U256) -> Self {
        let mut coef = coefficeint % Self::n();

        let mut result = S256Point::new(None, None);
        let mut current = self;

        while coef > U256::ZERO {
            if coef.bit(0).into() {
                result = result + current;
            }
            current = current + current;
            coef >>= 1;
        }

        result
    }

    pub fn g() -> Self {
        S256Point(
            Some(S256Field::new(U256::from_be_hex("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"))),
            Some(S256Field::new(U256::from_be_hex("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8")))
        )
    }

    pub fn a() -> S256Field {
        S256Field::new(U256::ZERO)
    }

    pub fn b() -> S256Field {
        S256Field::new(U256::from_be_hex("0000000000000000000000000000000000000000000000000000000000000007"))
    }

    pub fn n() -> U256 {
        U256::from_be_hex(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141"
        )
    }

    pub fn verify(self, z: S256Field, sig: Signature) -> bool {
        let s_inv = Scalar::new(sig.s.value()).inv();
        let u = Scalar::new(z.value())* s_inv;
        let v = Scalar::new(sig.r.value()) * s_inv;

        let total = Self::g().rmul(u.value()) + self.rmul(v.value());

        let x = total.0.unwrap().value();
        let x_mod_n = x % Self::n();

        x_mod_n == sig.r.value()
    }
}


impl ops::Add for S256Point {
    type Output = S256Point;

    fn add(self, other: S256Point) -> S256Point {
        let zero = S256Field::new_zero();
        if self.0 == None {
            return other;
        } else if other.0 == None {
            return self;
        } else if self.0 == other.0 && self.1 != other.1 {
            return S256Point::new(None, None);
        } else if self.0 != other.0 {
            let x1 = self.0.unwrap();
            let y1 = self.1.unwrap();
            let x2 = other.0.unwrap();
            let y2 = other.1.unwrap();

            let s = (y2 - y1) * (x2 - x1).inv();
            
            let x3 = (s * s) - x1 - x2;
            let y3 =  s * (x1 - x3) - y1;

            S256Point::new(Some(x3), Some(y3))
        } else if self == other && self.1 == Some(zero) {
            return S256Point::new(None, None);
        } else if self == other && self.1 != Some(zero) {
            let x = self.0.unwrap();
            let y = self.1.unwrap();
            let a = Self::a();

            let two= S256Field::new_two();
            let three= S256Field::new_three();

            let s = (three * x * x + a) * (two * y).inv();
            
            let x3 = (s * s) - (two * x);
            let y3 = (s * (x - x3)) - y;

            S256Point::new(Some(x3), Some(y3))
        } else {
            // Should never come here.
            self
        }
    }

}

#[cfg(test)]
mod s256_point_tests_temp {
    use super::*;

    #[test]
    fn g_order() {
        let G = S256Point(
            Some(S256Field::new(U256::from_be_hex("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"))),
            Some(S256Field::new(U256::from_be_hex("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8")))
        );

        let res = G.rmul(S256Point::n());
        assert_eq!(res.0, None);
        assert_eq!(res.1, None);
    }

    #[test]
    fn ex06() {
        let p = S256Point::new_concrete(
            S256Field::new(U256::from_be_hex("887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c")),
            S256Field::new(U256::from_be_hex("61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34")),
        );

        let z = S256Field::new(U256::from_be_hex("ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60"));
        
        let sig: Signature = Signature {
            r:S256Field::new(U256::from_be_hex("ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395")),
            s:S256Field::new(U256::from_be_hex("068342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4")),
        };

        let res = p.verify(z, sig);

        println!("res: {res:?}");

        assert!(res);

    }
}