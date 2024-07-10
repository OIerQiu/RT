use nalgebra::Vector3;

use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{reflect,refract,random_unit_vector};
use crate::rtweekend::{near_zero, random_double};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Vector3<f64>, scattered:&mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo:Vector3<f64>
}

impl Lambertian {
    pub fn initial () -> Self {
        Lambertian {
            albedo:Vector3::new(0.0,0.0,0.0),
        }
    }
    pub fn new (color:Vector3<f64>) -> Self {
        Lambertian {
            albedo:color.clone(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Vector3<f64>, scattered:&mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo:Vector3<f64>,
    pub fuzz:f64,
}

impl Metal {
    pub fn new (color:Vector3<f64>,fuz:f64) -> Self {
        let mut fuzzy = fuz;
        if fuzzy > 1.0 {
            fuzzy = 1.0;
        }
        Metal {
            albedo:color,
            fuzz:fuzzy,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Vector3<f64>, scattered:&mut Ray) -> bool {
        let mut reflected = reflect(&r_in.direction(), &rec.normal);
        reflected = reflected.normalize() + self.fuzz * random_unit_vector();
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}

pub struct Dielectric {
    pub refraction_index:f64,
}

impl Dielectric {
    pub fn new(refraction_ind:f64) -> Self{
        Dielectric {
            refraction_index:refraction_ind,
        }
    }
    pub fn reflectance(cosine:f64, refraction_index:f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0)*(1.0-cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Vector3<f64>, scattered:&mut Ray) -> bool {
        *attenuation = Vector3::new(1.0,1.0,1.0);
        let mut ri = self.refraction_index;
        if rec.front_face {
            ri = 1.0 / self.refraction_index;
        }
        let unit_direction = r_in.direction().normalize();
        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let mut direction:Vector3<f64> = Vector3::zeros();
        if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            direction = reflect(&unit_direction, &rec.normal);
        }
        else {
            direction = refract(&unit_direction, &rec.normal, ri);
        }
        *scattered = Ray::new(rec.p, direction);
        true
    }
}