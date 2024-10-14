// My crates
mod color;
mod vec3;

use std::{fmt::Write, fs};

use color::Color;
use vec3::{Point, Vec3};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const MAX_COLOR: u16 = 255;

const PATH_IMG: &str = "out_img/imagem.ppm";

fn main() {
    let met = metadata();
    let mut buf = String::with_capacity((WIDTH * HEIGHT) as usize + met.len() as usize);

    write!(buf, "{}", met).unwrap();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let pixel_color = Color(Vec3 {
                points: [
                    i as Point / MAX_COLOR as Point,
                    j as Point / MAX_COLOR as Point,
                    0.0,
                ],
            });

            writeln!(buf, "{}", pixel_color).unwrap();
        }
    }

    fs::write(PATH_IMG, buf).unwrap();
}

fn metadata() -> String {
    let mut buf = String::new();

    writeln!(buf, "P3").unwrap();
    writeln!(buf, "{} {}", WIDTH, HEIGHT).unwrap();
    writeln!(buf, "{}", MAX_COLOR).unwrap();

    buf
}
