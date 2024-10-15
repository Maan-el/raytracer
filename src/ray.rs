use crate::vec3::{Point, Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub const fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub const fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: Point) -> Point3 {
        self.origin + t * self.direction
    }
}
