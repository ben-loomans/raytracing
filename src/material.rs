use crate::{hittable::HitRecord, ray::Ray, Color, Vec3};

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
}

impl Metal {
    pub fn new(a: Color) -> Self {
        Self {
            albedo: a,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.dir.unit_vector().reflect(&rec.normal);
        
        let atten = self.albedo;
        let scatter = Ray::new(rec.p, reflected);

        Some((atten, scatter))
    }
}