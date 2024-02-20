mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;

use std::rc::Rc;

use hittable::HitRecord;
use hittable::Hittable;
use ray::Ray;
use crate::color::*;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::*;

fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    let mut rec = HitRecord::default();

    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1,1,1));
    }

    let unit_direction = r.dir.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color {x: 1.0, y: 1.0, z: 1.0} + a * Color {x: 0.5, y: 0.7, z: 1.0}
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let image_height = if image_height > 0 {image_height} else {1};

    // world
    let mut world = HittableList::default();

    world.add(Rc::new(Sphere::new(Point3::new(0,0,-1), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3 {x: 0.0, y: 0.0, z: 0.0};

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3 {x: viewport_width, y: 0.0, z: 0.0};
    let viewport_v = Vec3 {x: 0.0, y: -viewport_height, z: 0.0};

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center 
        - Vec3 {x: 0.0, y: 0.0, z: focal_length} 
        - 0.5 * (viewport_u + viewport_v);

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    
    // render
    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprintln!("scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let r = Ray {orig: camera_center, dir: ray_direction};

            let pixel_color = ray_color(&r, &world);
            write_color(pixel_color);
        }
    }
    eprintln!("Done rendering");
}