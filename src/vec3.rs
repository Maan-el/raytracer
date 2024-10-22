use std::{
    ops::{Add, AddAssign, Div, Index, Mul, MulAssign, Neg, Sub, SubAssign},
    slice::SliceIndex,
};

use crate::{helpers::Generator, interval::Interval};

pub type Point = f64;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub points: [Point; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    /// Generates a Vec3 with all values initialized to zero
    pub const fn new() -> Vec3 {
        Vec3 {
            points: [0.0, 0.0, 0.0],
        }
    }

    pub fn from_scalars<T, U, V>(s1: T, s2: U, s3: V) -> Vec3
    where
        T: Into<Point> + Copy,
        U: Into<Point> + Copy,
        V: Into<Point> + Copy,
    {
        Vec3 {
            points: [s1.into(), s2.into(), s3.into()],
        }
    }

    /// .Recieves a slice and returns a Vec3
    pub fn from_slice<T: Into<Point> + Copy>(values: [T; 3]) -> Vec3 {
        Vec3 {
            points: [values[0].into(), values[1].into(), values[2].into()],
        }
    }

    pub const fn x(&self) -> Point {
        self.points[0]
    }

    pub const fn y(&self) -> Point {
        self.points[1]
    }

    pub const fn z(&self) -> Point {
        self.points[2]
    }

    pub fn lenght(&self) -> f64 {
        self.lenght_squared().sqrt()
    }

    pub const fn lenght_squared(&self) -> f64 {
        self.points[0] * self.points[0]
            + self.points[1] * self.points[1]
            + self.points[2] * self.points[2]
    }

    pub const fn dot(&self, other: &Vec3) -> Point {
        self.points[0] * other.points[0]
            + self.points[1] * other.points[1]
            + self.points[2] * other.points[2]
    }

    pub const fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            points: [
                self.points[1] * other.points[2] - self.points[2] * other.points[1],
                self.points[2] * other.points[0] - self.points[0] * other.points[2],
                self.points[0] * other.points[1] - self.points[1] * other.points[0],
            ],
        }
    }

    #[inline]
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.lenght()
    }

    #[inline]
    pub fn random() -> Vec3 {
        let gen = Generator::random_points(3);
        Vec3::from_scalars(gen[0], gen[1], gen[2])
    }

    #[inline]
    pub fn random_interval(interval: Interval) -> Vec3 {
        let gen = Generator::random_points_interval(interval, 3);
        Vec3::from_scalars(gen[0], gen[1], gen[2])
    }

    #[allow(dead_code)]
    #[inline]
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Self::random_interval(Interval::from(-1, 1));
            let lensq = p.lenght_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    #[inline]
    pub fn random_on_hemisphere(&self) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if self.dot(&on_unit_sphere) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

impl<T> From<T> for Vec3
where
    T: Into<Point> + Copy,
{
    /// Takes a single scalar value and creates a new Vec3 with repeating values.
    #[inline]
    fn from(value: T) -> Self {
        Vec3 {
            points: [value.into(), value.into(), value.into()],
        }
    }
}

impl<Idx> Index<Idx> for Vec3
where
    Idx: SliceIndex<[Point], Output = Point>,
{
    type Output = Point;

    #[inline]
    fn index(&self, index: Idx) -> &Self::Output {
        &self.points[index]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3 {
            points: [-self[0], -self[1], -self[2]],
        }
    }
}

impl<T> Add<T> for Vec3
where
    T: Into<Vec3> + Copy,
{
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec3 {
            points: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl<T> Sub<T> for Vec3
where
    T: Into<Vec3> + Copy,
{
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec3 {
            points: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl<T> Mul<T> for Vec3
where
    T: Into<Vec3> + Copy,
{
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec3 {
            points: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

impl<T> Div<T> for Vec3
where
    T: Into<Point> + Copy,
{
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        self * (1.0 / rhs.into())
    }
}

impl Add<Vec3> for Point {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        let s: Vec3 = self.into();
        Vec3 {
            points: [s[0] + rhs[0], s[1] + rhs[1], s[2] + rhs[2]],
        }
    }
}

impl Sub<Vec3> for Point {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        let s: Vec3 = self.into();
        Vec3 {
            points: [s[0] - rhs[0], s[1] - rhs[1], s[2] - rhs[2]],
        }
    }
}

impl Mul<Vec3> for Point {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        let s: Vec3 = self.into();
        Vec3 {
            points: [s[0] * rhs[0], s[1] * rhs[1], s[2] * rhs[2]],
        }
    }
}

impl<T> AddAssign<T> for Vec3
where
    T: Into<Vec3> + Copy,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T> SubAssign<T> for Vec3
where
    T: Into<Vec3> + Copy,
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T> MulAssign<T> for Vec3
where
    T: Into<Vec3> + Copy,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

#[cfg(test)]
mod test {

    use super::Vec3;

    #[test]
    fn basics() {
        let v1 = Vec3 {
            points: [0.0, 0.5, 0.8],
        };

        assert_eq!(v1.x(), 0.0);
        assert_eq!(v1.y(), 0.5);
        assert_eq!(v1.z(), 0.8);

        let v2 = Vec3::new();
        let v3 = Vec3::from(2.0);
        let v4 = Vec3::from_scalars(0.0, 1, 23);
        let v5 = Vec3::from_slice([1.0, 1.0, 1.0]);

        assert_eq!(v2.points, [0.0, 0.0, 0.0]);
        assert_eq!(v3.points, [2.0, 2.0, 2.0]);
        assert_eq!(v4.points, [0.0, 1.0, 23.0]);
        assert_eq!(v5.points, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn lenght_squared() {
        let v1 = Vec3 {
            points: [1.0, 1.0, 1.0],
        };

        assert_eq!(v1.lenght_squared(), 3.0);
    }

    #[test]
    fn lenght() {
        let v1: Vec3 = Vec3 {
            points: [1.0, 1.0, 1.0],
        };

        assert!(1.732050 <= v1.lenght() && v1.lenght() <= 1.732051);
    }

    #[test]
    fn division() {
        let v1 = Vec3::from(4);

        let res = v1 / 2;

        assert_eq!(res.points, [2.0, 2.0, 2.0]);
    }

    #[test]
    fn unit_vector() {
        let v1 = Vec3::from(4);

        let res = v1.unit_vector();

        assert!(0.57735 <= res.points[0] && res.points[0] <= 0.57736);
        assert!(0.57735 <= res.points[1] && res.points[1] <= 0.57736);
        assert!(0.57735 <= res.points[2] && res.points[2] <= 0.57736);
    }
}
