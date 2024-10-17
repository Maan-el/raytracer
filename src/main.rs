// My crates
mod camera;
mod color;
mod helpers;
mod hit;
mod interval;
mod ray;
mod sphere;
mod vec3;
mod writer;

use std::rc::Rc;

use camera::Camera;
use hit::HittableList;
use sphere::Sphere;
use vec3::Point3;

fn main() {
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        &Point3::from_scalars(0, 0, -1),
        0.50000,
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::from_scalars(0, -100.5, -1),
        100.0,
    )));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1280;

    cam.render(&world);
}
