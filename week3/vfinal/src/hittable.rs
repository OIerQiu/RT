use nalgebra::Vector3;
use std::sync::Arc;

use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::{Material,Lambertian};
use crate::aabb::Aabb;
use crate::texture::Texture;

#[derive(Clone)]
pub struct HitRecord {
    pub p:Vector3<f64>,
    pub normal:Vector3<f64>,
    pub mat:Arc<dyn Material>,
    pub t:f64,
    pub u:f64,
    pub v:f64,
    pub front_face:bool,
}

impl HitRecord { 
    pub fn new() -> Self {
        HitRecord {
            p:Vector3::new(0.0,0.0,0.0),
            normal:Vector3::new(0.0,0.0,0.0),
            mat:Arc::new(Lambertian::new(Vector3::new(0.0,0.0,0.0))),
            t:0.0,
            u:0.0,
            v:0.0,
            front_face:false,
        }
    }
    pub fn set_face_normal(&mut self, r:&Ray, outward_normal:&Vector3<f64>) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        }
        else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit (&self, r: &Ray, ray_t:&Interval, rec:&mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;

    fn pdf_value(&self, origin:&Vector3<f64>, direction:&Vector3<f64>) -> f64 {
        0.0
    }
    
    fn random (&self, origin:&Vector3<f64>) -> Vector3<f64> {
        Vector3::new(1.0,0.0,0.0)
    }
}

pub struct Translate {
    object:Arc<dyn Hittable>,
    offset:Vector3<f64>,
    bbox:Aabb,
}

impl Translate {
    pub fn new (object:Arc<dyn Hittable>, offset:Vector3<f64>) -> Self {
        let bbox = object.bounding_box().add(&offset);
        Translate{
            object:object.clone(),
            offset:offset.clone(),
            bbox:bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit (&self, r: &Ray, ray_t:&Interval, rec:&mut HitRecord) -> bool {
        let offset_r = Ray::initial(r.origin() - self.offset, r.direction(), r.time());
        if !self.object.hit(&offset_r, &ray_t, rec) {
            return false;
        }
        rec.p += self.offset;
        true
    }
    
    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

pub struct RotateY {
    object:Arc<dyn Hittable>,
    sin_theta:f64,
    cos_theta:f64,
    bbox:Aabb,
}

impl RotateY {
    pub fn new(object:Arc<dyn Hittable>, angle:f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Vector3::new(std::f64::INFINITY, std::f64::INFINITY, std::f64::INFINITY);
        let mut max = Vector3::new(std::f64::NEG_INFINITY, std::f64::NEG_INFINITY, std::f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64)*bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64)*bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64)*bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    min.x = min.x.min(newx);
                    max.x = max.x.max(newx);
                    min.y = min.y.min(y);
                    max.y = max.y.max(y);
                    min.z = min.z.min(newz);
                    max.z = max.z.max(newz);
                }
            }
        }

        let bbox = Aabb::initial(&min, &max);
        RotateY {
            object:object,
            sin_theta:sin_theta,
            cos_theta:cos_theta,
            bbox:bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit (&self, r: &Ray, ray_t:&Interval, rec:&mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin.x = self.cos_theta * r.origin().x - self.sin_theta * r.origin().z;
        origin.z = self.sin_theta * r.origin().x + self.cos_theta * r.origin().z;

        direction.x = self.cos_theta * r.direction().x - self.sin_theta * r.direction().z;
        direction.z = self.sin_theta * r.direction().x + self.cos_theta * r.direction().z;

        let rotated_r = Ray::initial(origin, direction, r.time());

        if !self.object.hit(&rotated_r, &ray_t, rec) {
            return false;
        }
        let mut p = rec.p.clone();
        p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

        let mut normal = rec.normal.clone();
        normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
        normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}