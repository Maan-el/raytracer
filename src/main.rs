use std::{fmt::Write, fs};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const MAX_COLOR: u16 = 255;

const PATH_IMG: &str = "out_img/imagem.ppm";

fn main() {
    let mut buf = String::with_capacity((WIDTH * HEIGHT) as usize);

    write!(buf, "{}", metadata()).unwrap();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let r = i as f64 / MAX_COLOR as f64;
            let g = j as f64 / MAX_COLOR as f64;
            let b = 0.0;

            let r = (255.999 * r) as u8;
            let g = (255.999 * g) as u8;
            let b = (255.999 * b) as u8;

            writeln!(buf, "{} {} {}", r, g, b).unwrap();
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
