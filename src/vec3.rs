extern crate num;

use num_traits::AsPrimitive;

use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: (self.x * other.y - self.y * other.x),
        }
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn make_unit_vec(&self) -> Vec3 {
        let k = 1.0 / self.length();
        self * k
    }

    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::new(rand::random(), rand::random(), rand::random()) * 2.0
                - Vec3::new(1.0, 1.0, 1.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }
}

macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a> $imp<&'a $u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }
        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

forward_ref_binop!(impl Add, add for Vec3, Vec3);

impl<S: AsPrimitive<f64>> Add<S> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: S) -> Vec3 {
        Vec3 {
            x: self.x + rhs.as_(),
            y: self.y + rhs.as_(),
            z: self.z + rhs.as_(),
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

forward_ref_binop!(impl Sub, sub for Vec3, Vec3);

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

forward_ref_binop!(impl Div, div for Vec3, Vec3);

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

forward_ref_binop!(impl Mul, mul for Vec3, Vec3);

impl<S: AsPrimitive<f64>> DivAssign<S> for Vec3 {
    fn div_assign(&mut self, rhs: S) {
        self.x /= rhs.as_();
        self.y /= rhs.as_();
        self.z /= rhs.as_();
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> <Self as Div<Vec3>>::Output {
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl<S: AsPrimitive<f64>> Div<S> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: S) -> Vec3 {
        Vec3 {
            x: self.x / rhs.as_(),
            y: self.y / rhs.as_(),
            z: self.z / rhs.as_(),
        }
    }
}

//impl<'a, S: AsPrimitive<f64>> Mul<S> for Vec3 {
//    type Output = Vec3;
//
//    fn mul(self, rhs: S) -> Vec3 {
//        Vec3 {
//            x: self.x * rhs.as_(),
//            y: self.y * rhs.as_(),
//            z: self.z * rhs.as_(),
//        }
//    }
//}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

forward_ref_binop!(impl Mul, mul for f64, Vec3);
forward_ref_binop!(impl Mul, mul for Vec3, f64);

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn operations_dont_move_vec() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let b = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let u = &a + &b;
        let z = &a + &b;

        assert_eq!(u, z);
    }

    #[test]
    fn multiply_by_scalar() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(
            a * 3,
            Vec3 {
                x: 3.0,
                y: 6.0,
                z: 9.0,
            }
        )
    }

    #[test]
    fn multiply_by_vec() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let b = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(
            a * &b,
            Vec3 {
                x: 1.0,
                y: 4.0,
                z: 9.0,
            }
        )
    }

    #[test]
    fn unit_vec() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(
            a.make_unit_vec(),
            Vec3 {
                x: 0.2672612419124244,
                y: 0.5345224838248488,
                z: 0.8017837257372732,
            }
        )
    }

}
