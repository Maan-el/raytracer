use std::f64::consts::PI;

use crate::vec3::Point;

pub fn degrees_to_radians(degrees: Point) -> Point {
    degrees * PI / 180.0
}
