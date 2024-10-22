use crate::{
    hit::{Hit, Hittable},
    interval::Interval,
    ray,
    vec3::Point3,
};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    rec: Hit,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Self {
        Sphere {
            center: *center,
            radius: f64::max(0.0, radius),
            rec: Hit::new(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, ray_t: Interval, rec: &mut Hit) -> bool {
        let oc = self.center - *r.origin();

        let a = r.direction().lenght_squared();
        let h = r.direction().dot(&oc);
        let c = oc.lenght_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}
