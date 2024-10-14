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
            points: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}
