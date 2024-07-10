use nalgebra::Vector3;
use std::sync::Arc;

use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::{Material,Lambertian};

#[derive(Clone)]
pub struct HitRecord {
    pub p:Vector3<f64>,
    pub normal:Vector3<f64>,
    pub mat:Arc<dyn Material>,
    pub t:f64,
    pub front_face:bool,
}

impl HitRecord { 
    pub fn new() -> Self {
        HitRecord {
            p:Vector3::new(0.0,0.0,0.0),
            normal:Vector3::new(0.0,0.0,0.0),
            mat:Arc::new(Lambertian::initial()),
            t:0.0,
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
}