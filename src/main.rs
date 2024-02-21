mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod util;
mod material;

use std::f64::consts::PI;
use std::ops::Range;
use std::rc::Rc;

use camera::Camera;
use hittable::HitRecord;
use hittable::Hittable;
use interval::Interval;
use material::Dielectric;
use material::Lambertian;
use material::Material;
use material::Metal;
use ray::Ray;
use crate::color::*;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::util::random_f64;
use crate::vec3::*;

fn main() {
    let mut world = HittableList::default();

    let R = (PI / 4.0).cos();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left   = Rc::new(Dielectric::new(1.5));
    let material_right  = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, material_left.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),  -0.4, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, material_right)));

    let mut cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Point3::new(-2,2,1),
        Point3::new(0,0,-1),
        Vec3::new(0,1,0),
        10.0,
        3.4
    );

    cam.render(world);
}