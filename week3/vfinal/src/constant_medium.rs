use nalgebra::Vector3;
use std::sync::Arc;

use crate::{aabb::Aabb, hittable::{HitRecord, Hittable}, interval::Interval, material::{Isotropic, Material}, ray::Ray, rtweekend::random_double, texture::Texture};

pub struct ConstantMedium {
    boundary:Arc<dyn Hittable>,
    neg_inv_density:f64,
    phase_function:Arc<dyn Material>
}

impl ConstantMedium {
    pub fn new (boundary:Arc<dyn Hittable>, density:f64, tex:Arc<dyn Texture>) -> Self {
        ConstantMedium {
            boundary:boundary.clone(),
            neg_inv_density:-1.0 / density,
            phase_function:Arc::new(Isotropic::new(tex.clone())),
        }
    }

    pub fn initial (boundary:Arc<dyn Hittable>, density:f64, albedo:Vector3<f64>) -> Self {
        ConstantMedium {
            boundary:boundary.clone(),
            neg_inv_density:-1.0 / density,
            phase_function:Arc::new(Isotropic::initial(albedo.clone())),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit (&self, r:&Ray, ray_t:&Interval, rec:&mut HitRecord) -> bool {
        let enable_debug = false;
        let debugging = enable_debug && random_double() < 0.00001;

        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.hit(&r, &Interval::universe(), &mut rec1) {
            return false;
        }

        if !self.boundary.hit(&r, &Interval::new(rec1.t + 0.0001, std::f64::INFINITY), &mut rec2) {
            return false;
        }

        if debugging {
            eprintln!("");
            eprintln!("t_min={}, t_max={}", rec1.t, rec2.t);
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }

        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().dot(&r.direction()).sqrt();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            eprintln!("hit_distance = {}", hit_distance);
            eprintln!("rec.t = {}", rec.t);
            eprintln!("rec.p = {}", rec.p);
        }

        rec.normal = Vector3::new(1.0,0.0,0.0);
        rec.front_face = true;
        rec.mat = self.phase_function.clone();

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box().clone()
    }
    
    fn pdf_value(&self, origin:&Vector3<f64>, direction:&Vector3<f64>) -> f64 {
        0.0
    }
    
    fn random (&self, origin:&Vector3<f64>) -> Vector3<f64> {
        Vector3::new(1.0,0.0,0.0)
    }
}