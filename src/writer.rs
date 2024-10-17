use std::fmt::Write;
use std::fs;

use crate::color::Color;

pub struct Writer<'a> {
    data: String,
    path: &'a str,
}

const MAX_COLOR: u8 = 255;

impl<'a> Writer<'a> {
    #[inline(always)]
    pub fn new(path: &'a str, width: u32, height: u32) -> Self {
        let mut temp = String::with_capacity(width as usize * height as usize * 12 + 15);
        temp.push_str(&Self::metadata(width, height));

        Writer { data: temp, path }
    }

    #[inline(always)]
    pub fn add(&mut self, color: Color) {
        writeln!(self.data, "{}", color).unwrap()
    }

    #[inline(always)]
    pub fn write(self) {
        fs::write(self.path, self.data).unwrap();
    }

    #[inline(always)]
    fn metadata(width: u32, height: u32) -> String {
        let mut buf = String::new();

        writeln!(buf, "P3").unwrap();
        writeln!(buf, "{} {}", width, height).unwrap();
        writeln!(buf, "{}", MAX_COLOR).unwrap();

        buf
    }
}
