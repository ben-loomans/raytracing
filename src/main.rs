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
use rand::random;
use ray::Ray;
use crate::color::*;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::util::random_f64;
use crate::vec3::*;

fn main() {
    let mut world = HittableList::default();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5,0.5,0.5)));
    world.add(Rc::new(Sphere::new(Point3::new( 0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let center = Point3::new(a as f64 + 0.9 * random::<f64>(), 0.2, b as f64 + 0.9 * random::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;
                
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat > 0.95 {
                    // metal
                    let albedo = Color::random_bounded(&Interval::new(0.5, 1.0));
                    let fuzz = random_f64(&Interval::new(0.0, 0.5));
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Point3::new(0,1,0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point3::new(-4,1,0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Point3::new(4,1,0), 1.0, material3)));

    let cam = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20.0,
        Point3::new(13,2,3),
        Point3::new(0,0,0),
        Vec3::new(0,1,0),
        0.6,
        10.0,
    );

    cam.render(world);

    /* 
    let ground_material = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
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

    */
}