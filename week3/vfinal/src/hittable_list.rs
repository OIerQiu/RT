use nalgebra::Vector3;
use std::sync::Arc;

use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::aabb::Aabb;
use crate::rtweekend::random_int;

#[derive(Clone)]
pub struct HittableList {
    pub objects:Vec<Arc<dyn Hittable>>,
    pub bbox:Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects:Vec::new(),
            bbox:Aabb::empty(),
        }
    }

    pub fn initial(object:Arc<dyn Hittable>) -> Self {
        let obj = object.clone();
        HittableList {
            objects:vec![object],
            bbox:obj.bounding_box(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self,object:Arc<dyn Hittable>) {
        self.bbox = Aabb::merge(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }

}

impl Hittable for HittableList {
    fn hit (&self, r:&Ray, ray_t:&Interval, rec:&mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            if object.hit(&r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
    fn bounding_box (&self) -> Aabb {
        self.bbox.clone()
    }
    
    fn pdf_value(&self, origin:&Vector3<f64>, direction:&Vector3<f64>) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in self.objects.iter() {
            sum += weight * object.pdf_value(&origin, &direction);
        }

        sum
    }
    
    fn random (&self, origin:&Vector3<f64>) -> Vector3<f64> {
        let int_size = self.objects.len() as i64;
        self.objects[random_int(0, int_size - 1) as usize].random(&origin)
    }
}