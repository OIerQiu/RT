use nalgebra::Vector3;
use std::sync::Arc;
use std::cmp::Ordering;

use crate::hittable::{Hittable, HitRecord};
use crate::aabb::Aabb;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::rtweekend::random_int;

pub struct BvhNode {
    left:Arc<dyn Hittable>,
    right:Arc<dyn Hittable>,
    bbox:Aabb,
}

impl BvhNode {
    pub fn initial (list:&mut HittableList) -> Self {
        let len = list.objects.len() as i64;
        Self::new(&mut list.objects, 0, len)
    }

    pub fn new (objects:&mut Vec<Arc<dyn Hittable>>, start:i64, end:i64) -> Self {
        let mut bvh = BvhNode {
            left: objects[start as usize].clone(),
            right: objects[start as usize].clone(),
            bbox: Aabb::empty(),
        };
        for object_index in start..end {
            bvh.bbox = Aabb::merge(&bvh.bbox, &objects[object_index as usize].bounding_box());
        }
        let axis = bvh.bbox.longest_axis();
        let object_span = end - start;
        if object_span == 1 {}
        else if object_span == 2 {
            bvh.right = objects[start as usize + 1].clone();
        }
        else {
            let mut to_sort:Vec<Arc<dyn Hittable>> = Vec::new();
            for i in start..end {
                to_sort.push(objects[i as usize].clone());
            }
            to_sort.sort_by(|a, b| Self::box_compare(&a, &b, axis) );
            for i in start..end {
                objects[i as usize] = to_sort[i as usize - start as usize].clone();
            }
            let mid = start + object_span / 2;
            bvh.left = Arc::new(BvhNode::new(objects, start, mid));
            bvh.right = Arc::new(BvhNode::new(objects, mid, end));
        }
        bvh.bbox = Aabb::merge(&bvh.left.bounding_box(),&bvh.right.bounding_box());
        bvh
    }

    pub fn box_compare (a:&Arc<dyn Hittable>, b:&Arc<dyn Hittable>, axis_index:i64) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        if a_axis_interval.min < b_axis_interval.min {Ordering::Less} 
        else if a_axis_interval.min > b_axis_interval.min {Ordering::Greater}
        else {Ordering::Equal}
    }
}

impl Hittable for BvhNode {
    fn hit (&self, r: &Ray, ray_t:&Interval, rec:&mut HitRecord) -> bool {
        if !self.bbox.hit(&r, &ray_t) {
            return false;
        }
        let hit_left = self.left.hit(&r, &ray_t, rec);
        let hit_right = self.right.hit(&r, &Interval::new(ray_t.min, if hit_left {rec.t} else {ray_t.max}), rec);
        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
    
    fn pdf_value(&self, origin:&Vector3<f64>, direction:&Vector3<f64>) -> f64 {
        0.0
    }
    
    fn random (&self, origin:&Vector3<f64>) -> Vector3<f64> {
        Vector3::new(1.0,0.0,0.0)
    }
}