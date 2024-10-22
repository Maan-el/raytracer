use crate::vec3::Point;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Interval {
    pub min: Point,
    pub max: Point,
}

impl Interval {
    pub const EMPTY: Interval = Interval::new();
    pub const UNIVERSE: Interval = Interval {
        min: Point::NEG_INFINITY,
        max: Point::INFINITY,
    };

    /// Creates a new [`Interval`] with an empty range.
    pub const fn new() -> Self {
        Interval {
            min: Point::INFINITY,
            max: Point::NEG_INFINITY,
        }
    }

    /// .Creates a new [`Interval`] with the range provided.
    pub fn from<T>(min: T, max: T) -> Self
    where
        T: Into<Point> + Copy,
    {
        Interval {
            min: min.into(),
            max: max.into(),
        }
    }

    pub const fn size(&self) -> Point {
        self.max - self.min
    }

    pub const fn contains(&self, x: Point) -> bool {
        self.min <= x && x <= self.max
    }

    pub const fn surrounds(&self, x: Point) -> bool {
        self.min < x && x < self.max
    }

    pub const fn clamp(&self, x: Point) -> Point {
        if x < self.min {
            self.min
        } else if self.max < x {
            self.max
        } else {
            x
        }
    }
}
