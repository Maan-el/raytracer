use crate::{
    color::Color,
    helpers::Generator,
    hit::{Hit, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::{Point, Point3, Vec3},
    writer::Writer,
};

const PATH_IMG: &str = "out_img/imagem.ppm";

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u8,
    image_height: u32,
    pixel_sample_scale: Point,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    /// Creates a new [`Camera`].
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            pixel_sample_scale: 0.0,
            center: Point3::new(),
            pixel00_loc: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
        }
    }

    pub fn render(mut self, world: &impl Hittable) {
        self.initialize();

        let mut writer = Writer::new(PATH_IMG, self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }

                writer.add(pixel_color * self.pixel_sample_scale);
            }
        }

        writer.write();
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn initialize(&mut self) {
        // Calculate height, and ensure it is at least 1
        self.image_height = (f64::from(self.image_width) / self.aspect_ratio)
            .abs()
            .trunc() as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_sample_scale = 1.0 / Point::from(self.samples_per_pixel);

        self.center = Point3::new();

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (f64::from(self.image_width) / f64::from(self.image_height));

        // Calculate vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::from_scalars(viewport_width, 0, 0);
        let viewport_v = Vec3::from_scalars(0, -viewport_height, 0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        // Calculate the location of the upper_left pixel
        let vec_focal_lenght = Vec3::from_scalars(0, 0, focal_length);
        let viewport_upper_left = self.center - vec_focal_lenght - viewport_u / 2 - viewport_v / 2;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(r: &Ray, dept: u8, world: &impl Hittable) -> Color {
        if dept == 0 {
            return Color::new();
        }

        let mut rec = Hit::new();

        if world.hit(r, Interval::from(0.001, Point::INFINITY), &mut rec) {
            let direction = rec.normal + Vec3::random_unit_vector();
            0.5 * (Self::ray_color(&Ray::new(rec.p, direction), dept - 1, world))
        } else {
            let unit_direction = r.direction().unit_vector();
            let a = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - a) * Color::WHITE + a * Color::BLEND
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();

        let pixel_sample = self.pixel00_loc
            + ((Point::from(i) + offset.x()) * self.pixel_delta_u)
            + ((Point::from(j) + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::from_slice([
            Generator::random_point() - 0.5,
            Generator::random_point() - 0.5,
            0.0,
        ])
    }
}
