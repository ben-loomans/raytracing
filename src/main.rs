mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod util;

use std::ops::Range;
use std::rc::Rc;

use camera::Camera;
use hittable::HitRecord;
use hittable::Hittable;
use interval::Interval;
use ray::Ray;
use crate::color::*;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::util::random_f64;
use crate::vec3::*;

fn main() {
    let mut world = HittableList::default();

    world.add(Rc::new(Sphere::new(Point3::new(0,0,-1), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));

    let cam = Camera::new(16.0/9.0, 400, 10);

    cam.render(world);
}