use rand::random;

use crate::{hittable::HitRecord, ray::Ray, util::random_f64, Color, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self {
            albedo: a,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let atten = self.albedo;
        let scatter = Ray::new(rec.p, scatter_direction);

        Some((atten, scatter))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, fuzz: f64) -> Self {
        assert!(fuzz <= 1.0);
        Self {
            albedo: a,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&r_in.dir.unit_vector(), &rec.normal);
        let direction = reflected + self.fuzz * Vec3::random_unit_vector();
        let atten = self.albedo;
        let scatter = Ray::new(rec.p, direction);

        Some((atten, scatter))
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self {
            ir,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let atten = Color::new(1,1,1);
        let refraction_ratio = if rec.front_face {1.0 / self.ir} else {self.ir};

        let unit_direction = Vec3::unit_vector(r_in.dir);
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random::<f64>() {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scatter = Ray::new(rec.p, direction);
        return Some((atten, scatter));
    }
}

