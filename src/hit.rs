use std::rc::Rc;

use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point, Point3, Vec3},
};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Hit {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Point,
    pub front_face: bool,
}

impl Hit {
    pub const fn new() -> Self {
        Hit {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub trait Hittable {
    fn hit(&self, _r: &Ray, _ray_t: Interval, _rec: &mut Hit) -> bool {
        false
    }
}

impl Hittable for Hit {}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct HitList<T: Hittable> {
    pub rec: Hit,
    pub objects: Vec<Rc<T>>,
}

impl<T: Hittable> HitList<T> {
    pub const fn new() -> Self {
        HitList {
            rec: Hit::new(),
            objects: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from(object: Rc<T>) -> Self {
        HitList {
            rec: Hit::new(),
            objects: vec![object],
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<T>) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for HitList<T> {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Hit) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut temp_rec = Hit::new();

        for obj in self.objects.clone() {
            if obj.hit(r, Interval::from(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
