// My crates
mod color;
mod ray;
mod vec3;

use std::{fmt::Write, fs};

use color::Color;
use ray::Ray;
use vec3::{Point, Point3, Vec3};

const MAX_COLOR: u16 = 255;

const PATH_IMG: &str = "out_img/imagem.ppm";

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 1280;

    // Calculate height, and ensure it is at least 1
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::from(0);

    // Calculate vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::from_scalars(viewport_width, 0, 0);
    let viewport_v = Vec3::from_scalars(0, -viewport_height, 0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper_left pixel
    let vec_focal_lenght = Vec3::from_scalars(0, 0, focal_length);
    let viewport_upper_left = camera_center - vec_focal_lenght - viewport_u / 2 - viewport_v / 2;

    let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let met = metadata(image_width, image_height);
    let mut buf = String::with_capacity((image_width * image_height) as usize + met.len() as usize);
    write!(buf, "{}", met).unwrap();

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel100_loc + (i as Point * pixel_delta_u) + (j as Point * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);

            writeln!(buf, "{}", pixel_color).unwrap();
        }
    }

    fs::write(PATH_IMG, buf).unwrap();
}

fn metadata(width: u32, height: u32) -> String {
    let mut buf = String::new();

    writeln!(buf, "P3").unwrap();
    writeln!(buf, "{} {}", width, height).unwrap();
    writeln!(buf, "{}", MAX_COLOR).unwrap();

    buf
}

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::WHITE + a * Color::BLEND
}
