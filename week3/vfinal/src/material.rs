use nalgebra::Vector3;
use std::sync::Arc;

use crate::onb::Onb;
use crate::pdf::{CosinePdf, Pdf, SpherePdf};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{random_cosine_direction, random_on_hemisphere, random_unit_vector, reflect, refract};
use crate::rtweekend::{near_zero, random_double};
use crate::texture::{Texture, SolidColor};

#[derive(Clone)]
pub struct ScatterRecord {
    pub attenuation:Vector3<f64>,
    pub pdf_ptr:Arc<dyn Pdf>,
    pub skip_pdf:bool,
    pub skip_pdf_ray:Ray,
}

impl ScatterRecord {
    pub fn new () -> Self {
        ScatterRecord {
            attenuation:Vector3::zeros(),
            pdf_ptr:Arc::new(SpherePdf::new()),
            skip_pdf:false,
            skip_pdf_ray:Ray::new_zero(),
        }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, srec:&mut ScatterRecord) -> bool {
        false
    }
    
    fn emitted(&self, r_in:&Ray, rec:&HitRecord, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64> {
        Vector3::new(0.0,0.0,0.0)
    }
    
    fn scattering_pdf(&self, r_in:&Ray, rec:&HitRecord, scattered:&Ray) -> f64 {
        0.0
    }
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
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, srec:&mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Arc::new(CosinePdf::new(&rec.normal));
        srec.skip_pdf = false;
        true
    }
    
    fn scattering_pdf(&self, r_in:&Ray, rec:&HitRecord, scattered:&Ray) -> f64 {
        let cosine = rec.normal.dot(&scattered.direction().normalize());
        if cosine < 0.0 {0.0} else {cosine / std::f64::consts::PI}
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
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, srec:&mut ScatterRecord) -> bool {
        let mut reflected = reflect(&r_in.direction(), &rec.normal);
        reflected = reflected.normalize() + self.fuzz * random_unit_vector();
        
        srec.attenuation = self.albedo;
        srec.pdf_ptr = Arc::new(SpherePdf::new());
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::initial(rec.p.clone(), reflected.clone(), r_in.time());

        true
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
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, srec:&mut ScatterRecord) -> bool {
        srec.attenuation = Vector3::new(1.0,1.0,1.0);
        srec.pdf_ptr = Arc::new(SpherePdf::new());
        srec.skip_pdf = true;

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
        srec.skip_pdf_ray = Ray::initial(rec.p, direction, r_in.time());
        true
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
    fn emitted(&self, r_in:&Ray, rec:&HitRecord, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64> {
        if !rec.front_face {
            return Vector3::new(0.0,0.0,0.0);
        }
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
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, srec:&mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Arc::new(SpherePdf::new());
        srec.skip_pdf = false;
        true
    }
    
    fn scattering_pdf(&self, r_in:&Ray, rec:&HitRecord, scattered:&Ray) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }
}