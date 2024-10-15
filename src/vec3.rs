use std::{
    fmt::Display,
    ops::{Add, Div, Index, Mul, Neg, Sub},
    slice::SliceIndex,
};

pub type Point = f64;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub points: [Point; 3],
}

impl Vec3 {
    #[inline(always)]
    pub const fn new() -> Vec3 {
        Vec3 {
            points: [0.0, 0.0, 0.0],
        }
    }

    #[inline(always)]
    pub const fn x(&self) -> Point {
        self.points[0]
    }

    #[inline(always)]
    pub const fn y(&self) -> Point {
        self.points[1]
    }

    #[inline(always)]
    pub const fn z(&self) -> Point {
        self.points[2]
    }

    #[inline(always)]
    pub fn lenght(&self) -> f64 {
        self.lenght_squared().sqrt()
    }

    #[inline(always)]
    pub fn lenght_squared(&self) -> f64 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
    }

    #[inline(always)]
    pub fn dot(&self, other: &Vec3) -> Point {
        self[0] * other[0] + self[1] * other[1] + self[2] + other[2]
    }

    #[inline(always)]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            points: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0],
            ],
        }
    }

    #[inline(always)]
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.lenght()
    }
}

impl Display for Vec3 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl<Idx> Index<Idx> for Vec3
where
    Idx: SliceIndex<[Point], Output = Point>,
{
    type Output = Point;

    #[inline(always)]
    fn index(&self, index: Idx) -> &Self::Output {
        &self.points[index]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec3 {
            points: [-self[0], -self[1], -self[2]],
        }
    }
}
impl<T> Add<T> for Vec3
where
    T: Into<Point> + Copy,
{
    type Output = Vec3;

    #[inline(always)]
    fn add(self, rhs: T) -> Self::Output {
        Vec3 {
            points: [
                self[0] + rhs.into(),
                self[1] + rhs.into(),
                self[2] + rhs.into(),
            ],
        }
    }
}

impl<T> Sub<T> for Vec3
where
    T: Into<Point> + Copy,
{
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, rhs: T) -> Self::Output {
        Vec3 {
            points: [
                self[0] - rhs.into(),
                self[1] - rhs.into(),
                self[2] - rhs.into(),
            ],
        }
    }
}

impl<T> Mul<T> for Vec3
where
    T: Into<Point> + Copy,
{
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            points: [
                self[0] * rhs.into(),
                self[1] * rhs.into(),
                self[2] * rhs.into(),
            ],
        }
    }
}

impl Div<Point> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, rhs: Point) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            points: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            points: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
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
