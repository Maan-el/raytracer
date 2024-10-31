use std::f64::consts::PI;

use rand::{distributions::Uniform, prelude::Distribution, thread_rng, Rng};

use crate::{interval::Interval, vec3::Point};

#[allow(dead_code)]
pub fn degrees_to_radians(degrees: Point) -> Point {
    degrees * PI / 180.0
}

#[derive(Default)]
pub struct Generator {}

impl Generator {
    #[inline]
    pub fn random_point() -> Point {
        thread_rng().gen_range(0.0..1.0)
    }

    #[inline]
    pub fn random_points(amount: usize) -> Vec<Point> {
        let interval = Interval::from(0, 1);
        Self::random_points_interval(interval, amount)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn random_point_interval(interval: Interval) -> Point {
        thread_rng().clone().gen_range(interval.min..interval.max)
    }

    #[inline]
    pub fn random_points_interval(interval: Interval, amount: usize) -> Vec<Point> {
        let between = Uniform::from(interval.min..interval.max);

        let mut rng = thread_rng();
        let mut buf: Vec<Point> = Vec::with_capacity(amount);

        for _ in 0..amount {
            buf.push(between.sample(&mut rng));
        }

        buf
    }
}
