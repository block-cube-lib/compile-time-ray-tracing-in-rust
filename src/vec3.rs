use crate::util::*;
use std::ops::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl const std::default::Default for Vec3 {
    fn default() -> Self {
        Vec3::ZERO
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub const fn length(&self) -> f64 {
        sqrt(self.length_squared())
    }
}

impl const Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl const Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of range"),
        }
    }
}

impl const IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("out of range"),
        }
    }
}

impl const Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl const Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl const Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl const MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl const Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl const DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl const Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl const Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

pub const fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub const fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub const fn unit_vector(v: &Vec3) -> Vec3 {
    let len = v.length();
    if std::f64::EPSILON < abs(len) {
        *v / len
    } else {
        Vec3::ZERO
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        const V: Vec3 = Vec3::default();
        assert_eq!(V, Vec3::ZERO);
    }

    #[test]
    fn length_squared() {
        for x in -100..100 {
            for y in -100..100 {
                for z in -100..100 {
                    let (x, y, z) = (x as f64, y as f64, z as f64);
                    let v: Vec3 = Vec3::new(x, y, z);
                    let length_squared: f64 = v.length_squared();
                    assert_eq!(length_squared, x * x + y * y + z * z);
                }
            }
        }
    }

    #[test]
    fn length() {
        for x in -100..100 {
            for y in -100..100 {
                for z in -100..100 {
                    let (x, y, z) = (x as f64, y as f64, z as f64);
                    let v: Vec3 = Vec3::new(x, y, z);
                    let length: f64 = v.length();
                    let diff = (length - (x * x + y * y + z * z).sqrt()).abs();
                    assert!(diff < 1e-10);
                }
            }
        }
    }

    #[test]
    fn neg() {
        let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-v, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn index() {
        let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!((v[0], v[1], v[2]), (1.0, 2.0, 3.0));
    }

    #[test]
    fn index_mut() {
        let mut v: Vec3 = Vec3::ZERO;
        v[0] = 1.0;
        v[1] = 2.0;
        v[2] = 3.0;
        assert_eq!((v[0], v[1], v[2]), (1.0, 2.0, 3.0));
    }
    #[test]
    fn add() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(2.1, 3.2, 4.3);
        let v3 = v1 + v2;
        assert_eq!((v3.x, v3.y, v3.z), (3.1, 5.2, 7.3));
    }

    #[test]
    fn sub() {
        let v1: Vec3 = Vec3::new(2.1, 3.2, 4.3);
        let v2: Vec3 = Vec3::new(1.0, 2.1, 3.3);
        let v3 = v1 - v2;
        assert_eq!((v3.x, v3.y, v3.z), (1.1, 1.1, 1.0));
    }

    #[test]
    fn mul_vec3_scalar() {
        let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn mul_assign_vec3_scalar() {
        let mut v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn mul_scalar_vec3() {
        let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(2.0 * v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn div_vec3_scalar() {
        let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn div_assign_vec3_scalar() {
        let mut v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        v /= 2.0;
        assert_eq!(v, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn dot() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(2.0, 4.0, 6.0);
        let dot_value = super::dot(&v1, &v2);
        assert_eq!(dot_value, v1.x * v2.x + v1.y * v2.y + v1.z * v2.z);
    }

    #[test]
    fn corss() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(2.0, 4.0, 6.0);
        let cross_v = super::cross(&v1, &v2);
        assert_eq!(
            cross_v,
            Vec3::new(
                v1.y * v2.z - v1.z * v2.y,
                v1.z * v2.x - v1.x * v2.z,
                v1.x * v2.y - v1.y * v2.x,
            ),
        );
    }

    #[test]
    fn unit_vector() {
        let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v = super::unit_vector(&v);
        assert!((v.length() - 1.0).abs() < 1e-10, "length = {}", v.length());
    }
}
