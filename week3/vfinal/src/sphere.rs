use nalgebra::Vector3;
use std::sync::Arc;

use crate::onb::Onb;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::Aabb;
use crate::rtweekend::random_double;

pub struct Sphere {
    pub center1:Vector3<f64>,
    pub radius:f64,
    pub mat:Arc<dyn Material>,
    pub is_moving:bool,
    pub center_vec:Vector3<f64>,
    pub bbox:Aabb,
}

impl Sphere {
    pub fn new(cent:Vector3<f64>, rad:f64, mate:Arc<dyn Material>) -> Self {
        let rvec = Vector3::new(rad, rad, rad);
        Sphere {
            center1:cent.clone(),
            radius:rad,
            mat:mate.clone(),
            is_moving:false,
            center_vec:Vector3::zeros(),
            bbox:Aabb::initial(&(cent - rvec), &(cent + rvec)),
        }
    }
    pub fn initial(center_1:Vector3<f64>, center2:Vector3<f64>, rad:f64, mate:Arc<dyn Material>) -> Self {
        let rvec = Vector3::new(rad, rad, rad);
        let box1 = Aabb::initial(&(center_1 - rvec), &(center_1 + rvec));
        let box2 = Aabb::initial(&(center2 - rvec), &(center2 + rvec));
        Sphere {
            center1:center_1.clone(),
            radius:rad.max(0.0),
            mat:mate.clone(),
            is_moving:true,
            center_vec:center2 - center_1,
            bbox:Aabb::merge(&box1, &box2),
        }
    }
    pub fn sphere_center(&self, time:f64) -> Vector3<f64> {
        self.center1 + time * self.center_vec
    }
    pub fn get_sphere_uv(p:&Vector3<f64>, u:&mut f64, v:&mut f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;
        *u = phi / (2.0 * std::f64::consts::PI);
        *v = theta / std::f64::consts::PI;
    }
    pub fn random_to_sphere(radius:f64, distance_squared:f64) -> Vector3<f64> {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared ).sqrt() - 1.0);

        let phi = 2.0 * std::f64::consts::PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        Vector3::new(x, y, z)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r:&Ray, ray_t:&Interval, rec:&mut HitRecord) -> bool {
        let mut center = self.center1;
        if self.is_moving {
            center = Self::sphere_center(&self, r.time());
        }
        let oc = center - r.origin();
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
        let outward_normal = (rec.p - center)/self.radius;
        rec.set_face_normal(&r, &outward_normal);
        Self::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = self.mat.clone();
        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
    
    fn pdf_value(&self, origin:&Vector3<f64>, direction:&Vector3<f64>) -> f64 {
        let mut rec = HitRecord::new();
        if !Self::hit(&self, &Ray::new(origin.clone(), direction.clone()), &Interval::new(0.001, std::f64::INFINITY), &mut rec) {
            return 0.0;
        }

        let cos_theta_max = (1.0 - self.radius * self.radius / (self.center1 - origin).dot(&(self.center1 - origin))).sqrt();
        let solid_angle = 2.0 * std::f64::consts::PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }
    
    fn random (&self, origin:&Vector3<f64>) -> Vector3<f64> {
        let direction = self.center1 - origin;
        let distance_squared = direction.dot(&direction);
        let mut uvw = Onb::new();
        uvw.build_from_w(&direction);
        uvw.local_vec(&Self::random_to_sphere(self.radius, distance_squared))
    }
}