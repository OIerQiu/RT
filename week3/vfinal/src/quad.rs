use nalgebra::Vector3;
use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::random_double;

pub struct Quad {
    q:Vector3<f64>,
    u:Vector3<f64>,
    v:Vector3<f64>,
    w:Vector3<f64>,
    mat:Arc<dyn Material>,
    bbox:Aabb,
    normal:Vector3<f64>,
    d:f64,
    area:f64,
}

impl Quad {
    pub fn new(q:Vector3<f64>, u:Vector3<f64>, v:Vector3<f64>, mat:Arc<dyn Material>) -> Self {
        let mut ans = Quad {
            q:q.clone(),
            u:u.clone(),
            v:v.clone(),
            w:Vector3::zeros(),
            mat:mat.clone(),
            bbox:Aabb::empty(),
            normal:Vector3::zeros(),
            d:0.0,
            area:0.0,
        };
        let n = u.cross(&v);
        ans.normal = n.normalize();
        ans.d = ans.normal.dot(&q);
        ans.w = n / n.dot(&n);
        ans.area = n.dot(&n).sqrt();
        ans.set_bounding_box();
        ans
    }

    pub fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = Aabb::initial(&self.q, &(self.q + self.u + self.v));
        let bbox_diagonal2 = Aabb::initial(&(self.q + self.u), &(self.q + self.v));
        self.bbox = Aabb::merge(&bbox_diagonal1, &bbox_diagonal2);
    }

    pub fn is_interior(a:f64, b:f64, rec:&mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit (&self, r: &crate::ray::Ray, ray_t:&crate::interval::Interval, rec:&mut crate::hittable::HitRecord) -> bool {
        let denom = self.normal.dot(&r.direction());
        if denom.abs() < 1e-8 {
            return false;
        }
        let t = (self.d - self.normal.dot(&r.origin()))/denom;
        if !ray_t.contains(t) {
            return false;
        }
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);
        true
    }

    fn bounding_box(&self) -> crate::aabb::Aabb {
        self.bbox.clone()
    }
    
    fn pdf_value(&self, origin:&Vector3<f64>, direction:&Vector3<f64>) -> f64 {
        let mut rec = HitRecord::new();

        if !Self::hit(&self, &Ray::new(origin.clone(), direction.clone()), &Interval::new(0.001, std::f64::INFINITY), &mut rec) {
            return 0.0;
        }

        let distance_squared = rec.t * rec.t * direction.dot(&direction);
        let cosine = direction.dot(&rec.normal).abs() / direction.dot(&direction).sqrt();

        distance_squared / (cosine * self.area)
    }
    
    fn random (&self, origin:&Vector3<f64>) -> Vector3<f64> {
        let p = self.q + random_double() * self.u + random_double() * self.v;
        p - origin
    }
}

pub fn abox(a:&Vector3<f64>, b:&Vector3<f64>, mat:&Arc<dyn Material>) -> Arc<HittableList> {
    let mut sides = HittableList::new();
    let min = Vector3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Vector3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
    
    let dx = Vector3::new(max.x-min.x, 0.0, 0.0);
    let dy = Vector3::new(0.0, max.y-min.y, 0.0);
    let dz = Vector3::new(0.0, 0.0, max.z-min.z);

    sides.add(Arc::new(Quad::new(Vector3::new(min.x,min.y,max.z),dx.clone(),dy.clone(),mat.clone())));
    sides.add(Arc::new(Quad::new(Vector3::new(max.x,min.y,max.z),-dz.clone(),dy.clone(),mat.clone())));
    sides.add(Arc::new(Quad::new(Vector3::new(max.x,min.y,min.z),-dx.clone(),dy.clone(),mat.clone())));
    sides.add(Arc::new(Quad::new(Vector3::new(min.x,min.y,min.z),dz.clone(),dy.clone(),mat.clone())));
    sides.add(Arc::new(Quad::new(Vector3::new(min.x,max.y,max.z),dx.clone(),-dz.clone(),mat.clone())));
    sides.add(Arc::new(Quad::new(Vector3::new(min.x,min.y,min.z),dx.clone(),dz.clone(),mat.clone())));

    Arc::new(sides)
}