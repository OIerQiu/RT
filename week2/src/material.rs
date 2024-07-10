use nalgebra::Vector3;
use std::sync::Arc;

use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{reflect,refract,random_unit_vector};
use crate::rtweekend::{near_zero, random_double};
use crate::texture::{Texture, SolidColor};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Vector3<f64>, scattered:&mut Ray) -> bool;
    fn emitted(&self, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64>;
}

pub struct Lambertian {
    pub tex:Arc<dyn Texture>
}

impl Lambertian {
    pub fn initial (tex:Arc<dyn Texture>) -> Self {
        Lambertian {
            tex:tex,
        }
    }
    pub fn new (color:Vector3<f64>) -> Self {
        Lambertian {
            tex:Arc::new(SolidColor::new(color.clone())),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Vector3<f64>, scattered:&mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::initial(rec.p, scatter_direction, r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }
    
    fn emitted(&self, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64> {
        Vector3::new(0.0,0.0,0.0)
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
        *scattered = Ray::initial(rec.p, reflected, r_in.time());
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
    
    fn emitted(&self, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64> {
        Vector3::new(0.0,0.0,0.0)
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
        *scattered = Ray::initial(rec.p, direction, r_in.time());
        true
    }
    
    fn emitted(&self, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64> {
        Vector3::new(0.0,0.0,0.0)
    }
}

pub struct DiffuseLight {
    tex:Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new (tex:Arc<dyn Texture>) -> Self {
        DiffuseLight {
            tex:tex.clone(),
        }
    }

    pub fn initial (emit:Vector3<f64>) -> Self {
        DiffuseLight {
            tex:Arc::new(SolidColor::new(emit.clone())),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Vector3<f64>, scattered:&mut Ray) -> bool {
       false
    }
    
    fn emitted(&self, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64> {
        self.tex.value(u,v,&p)
    }
}

pub struct Isotropic {
    tex:Arc<dyn Texture>,
}

impl Isotropic {
    pub fn initial (albedo:Vector3<f64>) -> Self {
        Isotropic {
            tex:Arc::new(SolidColor::new(albedo.clone()))
        }
    }

    pub fn new (tex:Arc<dyn Texture>) -> Self {
        Isotropic {
            tex:tex.clone(),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Vector3<f64>, scattered:&mut Ray) -> bool {
        *scattered = Ray::initial(rec.p.clone(), random_unit_vector(), r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }

    fn emitted(&self, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64> {
        Vector3::new(0.0,0.0,0.0)
    }
}