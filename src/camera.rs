use rand::random;

use crate::{hittable::{HitRecord, Hittable}, interval::Interval, ray::{self, Ray}, util::random_f64, write_color, Color, Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if image_height > 0 {image_height} else {1};

        // camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3 {x: 0.0, y: 0.0, z: 0.0};

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3 {x: viewport_width, y: 0.0, z: 0.0};
        let viewport_v = Vec3 {x: 0.0, y: -viewport_height, z: 0.0};

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center 
            - Vec3 {x: 0.0, y: 0.0, z: focal_length} 
            - 0.5 * (viewport_u + viewport_v);

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: impl Hittable) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0 ,0, 0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, &world);
                }

                write_color(pixel_color, self.samples_per_pixel);
            }
        }
        eprintln!("Done rendering");
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        return Ray {orig: self.center, dir: pixel_sample - self.center}
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random::<f64>();
        let py = -0.5 + random::<f64>();

        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &impl Hittable) -> Color {
        let mut rec = HitRecord::default();

        if depth <= 0 {return Color::new(0,0,0)}
    
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let direction = rec.normal + Vec3::random_on_hemisphere(&rec.normal);
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), depth - 1, world);
        }
    
        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * Color::new(1,1,1) + a * Color::new(0.5, 0.7, 1.0);
    }
}