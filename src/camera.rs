use crate::{hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray, write_color, Color, Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
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
                let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;

                let r = Ray {orig: self.center, dir: ray_direction};

                let pixel_color = Self::ray_color(&r, &world);
                write_color(pixel_color);
            }
        }
        eprintln!("Done rendering");
    }

    fn initialize() {

    }

    fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
        let mut rec = HitRecord::default();
    
        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1,1,1));
        }
    
        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color {x: 1.0, y: 1.0, z: 1.0} + a * Color {x: 0.5, y: 0.7, z: 1.0}
    }
}