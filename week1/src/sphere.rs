use nalgebra::Vector3;
use std::sync::Arc;

use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::material::Material;

pub struct Sphere {
    pub center:Vector3<f64>,
    pub radius:f64,
    pub mat:Arc<dyn Material>,
}

impl Sphere {
    pub fn new(cent:Vector3<f64>, rad:f64, mate:Arc<dyn Material>) -> Self {
        Sphere {
            center:cent.clone(),
            radius:rad.clone(),
            mat:mate,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r:&Ray, ray_t:&Interval, rec:&mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().dot(&r.direction());
        let h = r.direction().dot(&oc);
        let c = oc.dot(&oc)-self.radius*self.radius;
        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd: f64 = discriminant.sqrt();
        let mut root = (h-sqrtd)/a;
        if !ray_t.surrounds(root) {
            root = (h+sqrtd)/a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center)/self.radius;
        rec.set_face_normal(&r, &outward_normal);
        rec.mat = self.mat.clone();
        true
    }
}