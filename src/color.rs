use std::fmt::Display;

use crate::vec3::Vec3;

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
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let r = (r * 255.999) as u8;
        let g = (g * 255.999) as u8;
        let b = (b * 255.999) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}
