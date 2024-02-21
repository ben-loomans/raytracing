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

pub struct CameraBuilder {
    aspect_ratio: Option<f64>,
    image_width: Option<u32>,
    samples_per_pixel: Option<u32>,
    max_depth: Option<u32>,
    vfov: Option<f64>,
    lookfrom: Option<Point3>,
    lookat: Option<Point3>,
    vup: Option<Vec3>,
    defocus_angle: Option<f64>,
    focus_dist: Option<f64>,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            aspect_ratio: None,
            image_width: None,
            samples_per_pixel: None,
            max_depth: None,
            vfov: None,
            lookfrom: None,
            lookat: None,
            vup: None,
            defocus_angle: None,
            focus_dist: None,
        }
    }

    pub fn build(self) -> Camera {
        let aspect_ratio = self.aspect_ratio.unwrap_or(16.0 / 9.0);
        let image_width = self.image_width.unwrap_or(400);
        let samples_per_pixel = self.samples_per_pixel.unwrap_or(10);
        let max_depth = self.max_depth.unwrap_or(10);

        let vfov = self.vfov.unwrap_or(90.0);
        let lookfrom = self.lookfrom.unwrap_or(Point3::new(0,0,-1));
        let lookat = self.lookat.unwrap_or(Point3::new(0,0,0));
        let vup = self.vup.unwrap_or(Vec3::new(0,1,0));
        let defocus_angle = self.defocus_angle.unwrap_or(0.0);
        let focus_dist = self.focus_dist.unwrap_or(0.0);

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

        Camera {
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

    pub fn aspect_ratio(&mut self, aspect_ratio: f64) -> &mut Self {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }

    pub fn width(&mut self, width: u32) -> &mut Self {
        self.image_width = Some(width);
        self
    }

    pub fn samples_per_pixel(&mut self, samples: u32) -> &mut Self {
        self.samples_per_pixel = Some(samples);
        self
    }

    pub fn max_depth(&mut self, depth: u32) -> &mut Self {
        self.max_depth = Some(depth);
        self
    }

    pub fn field_of_view(&mut self, vfov: f64) -> &mut Self {
        self.vfov = Some(vfov);
        self
    }

    pub fn set_view(&mut self, lookfrom: Point3, lookat: Point3, vup:Vec3) -> &mut Self {
        self.lookfrom = Some(lookfrom);
        self.lookat = Some(lookat);
        self.vup = Some(vup);
        self
    }

    pub fn focus(&mut self, defocus_angle: f64, focus_dist: f64) -> &mut Self {
        self.defocus_angle = Some(defocus_angle);
        self.focus_dist = Some(focus_dist);
        self
    }
}