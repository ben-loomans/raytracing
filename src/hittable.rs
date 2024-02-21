use std::rc::Rc;

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-*outward_normal};
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray: Interval) -> Option<HitRecord>;
}