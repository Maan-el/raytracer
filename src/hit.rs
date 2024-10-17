use std::rc::Rc;

use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point, Point3, Vec3},
};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Point,
    pub front_face: bool,
}

impl HitRecord {
    pub const fn new() -> Self {
        HitRecord {
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
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        false
    }
}

impl Hittable for HitRecord {}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct HittableList<T: Hittable> {
    pub rec: HitRecord,
    pub objects: Vec<Rc<T>>,
}

impl<T: Hittable> HittableList<T> {
    pub const fn new() -> Self {
        HittableList {
            rec: HitRecord::new(),
            objects: Vec::new(),
        }
    }

    pub fn from(object: Rc<T>) -> Self {
        HittableList {
            rec: HitRecord::new(),
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<T>) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut temp_rec = HitRecord::new();

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
