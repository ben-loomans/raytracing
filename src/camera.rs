use std::sync::Arc;

use rayon;
use rayon::prelude::*;

use rand::random;

use crate::{hittable::{HitRecord, Hittable}, interval::Interval, ray::{self, Ray}, util::random_f64, write_color, Color, Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3, 
    v: Vec3, 
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,

        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        /* 
        let aspect_ratio = 1.0;
        let image_width = 100; 
        let samples_per_pixel = 10;
        let max_depth = 10;
        let vfov: f64 = 90.0;
        */

        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if image_height > 0 {image_height} else {1};

        let center = lookfrom;

        // determine viewport dimensions
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // calculate u,v,w from camera coordinate frame
        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(vup.cross(&w));
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            lookfrom,
            lookat,
            vup,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: Arc<dyn Hittable>) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_color = (0..self.samples_per_pixel).into_par_iter().map(|_| {
                    let r = self.get_ray(i, j);
                    let new_world = world.clone();
                    self.ray_color(&r, self.max_depth, new_world)
                }).sum();

                write_color(pixel_color, self.samples_per_pixel);
            }
        }
        eprintln!("Done rendering");
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // get a randomly-sampled camera ray for the pixel at locationi,j originating from the camera defocus disk.

        let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
        let ray_direction = pixel_sample - ray_origin;

        return Ray {orig: ray_origin, dir: ray_direction}
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random::<f64>();
        let py = -0.5 + random::<f64>();

        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // returns a random point in the camera defocus disk
        let p = Vec3::random_in_unit_disk();

        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: Arc<dyn Hittable>) -> Color {
        if depth <= 0 {return Color::new(0,0,0)}
    
        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some((atten, scatter)) = rec.mat.scatter(r, &rec) {
                return atten * self.ray_color(&scatter, depth - 1, world);
            }
            
            return Color::new(0,0,0);
        }
    
        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * Color::new(1,1,1) + a * Color::new(0.5, 0.7, 1.0);
    }
}