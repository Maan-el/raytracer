use std::fmt::Display;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

impl Color {
    pub const WHITE: Color = Color {
        points: [1.0, 1.0, 1.0],
    };

    pub const BLACK: Color = Color {
        points: [0.0, 0.0, 0.0],
    };

    pub const BLEND: Color = Color {
        points: [0.5, 0.7, 1.0],
    };
}

impl Display for Color {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn linear_to_gama(linear_component: f64) -> f64 {
            if linear_component > 0.0 {
                linear_component.sqrt()
            } else {
                0.0
            }
        }

        let r = self.x();
        let g = self.y();
        let b = self.z();

        let r = linear_to_gama(r);
        let g = linear_to_gama(g);
        let b = linear_to_gama(b);

        let intensity = Interval::from(0.000, 0.999);
        let r_byte = (256.000 * intensity.clamp(r)).trunc().abs() as u8;
        let g_byte = (256.000 * intensity.clamp(g)).trunc().abs() as u8;
        let b_byte = (256.000 * intensity.clamp(b)).trunc().abs() as u8;

        write!(f, "{r_byte} {g_byte} {b_byte}")
    }
}
