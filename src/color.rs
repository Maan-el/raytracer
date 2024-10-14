use std::fmt::Display;

use crate::vec3::Vec3;

pub struct Color(pub Vec3);

impl Display for Color {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.0.x();
        let g = self.0.y();
        let b = self.0.z();

        let r = (r * 255.999) as u8;
        let g = (g * 255.999) as u8;
        let b = (b * 255.999) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}
