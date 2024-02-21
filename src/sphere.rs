use std::sync::Arc;

use crate::hittable::*;
use crate::interval::Interval;
use crate::material::Material;
use crate::vec3::*;
use crate::ray::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {return None;}
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let outward_normal = (r.at(root) - self.center) / self.radius;

        let mut rec = HitRecord {
            p: r.at(root),
            normal: outward_normal,
            mat: Arc::clone(&self.mat),
            t: root,
            front_face: false,
        };
        
        rec.set_face_normal(r, &outward_normal);

        return Some(rec);
    }
}