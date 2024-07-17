use nalgebra::Vector3;

use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Aabb {
    pub x:Interval,
    pub y:Interval,
    pub z:Interval,
}

impl Aabb{
    pub fn empty () -> Self {
        Aabb {
            x:Interval::empty(),
            y:Interval::empty(),
            z:Interval::empty(),
        }
    }
    pub fn universe () -> Self {
        Aabb {
            x:Interval::universe(),
            y:Interval::universe(),
            z:Interval::universe(),
        }
    }
    pub fn add(&self, offset:&Vector3<f64>) -> Aabb {
        Aabb::new(self.x.add(offset.x), self.y.add(offset.y), self.z.add(offset.z))
    }
    pub fn new(x:Interval, y:Interval, z:Interval) -> Self {
        let mut ans = Aabb {
            x:x.clone(),
            y:y.clone(),
            z:z.clone(),
        };
        ans.pad_to_minimums();
        ans 
    }
    pub fn initial(a:&Vector3<f64>, b:&Vector3<f64>) -> Self {
        let mut ans = Aabb {
            x:Interval::new(a.x.min(b.x),a.x.max(b.x)),
            y:Interval::new(a.y.min(b.y),a.y.max(b.y)),
            z:Interval::new(a.z.min(b.z),a.z.max(b.z)),
        };
        ans.pad_to_minimums();
        ans
    }
    pub fn merge(box0:&Aabb, box1:&Aabb) -> Self{
        Aabb{
            x:Interval::merge(&box0.x,&box1.x),
            y:Interval::merge(&box0.y, &box1.y),
            z:Interval::merge(&box0.z, &box1.z),
        }
    }

    pub fn axis_interval(&self, n:i64) -> Interval {
        if n == 1 {
            return self.y.clone();
        }
        if n == 2 {
            return self.z.clone();
        }
        self.x.clone()
    }

    pub fn hit (&self, r:&Ray, ray_t:&Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();
        let mut ray_t = ray_t.clone();
        for axis in 0..3 {
            let mut ray_dir_axis = ray_dir.z;
            let mut ray_orig_axis = ray_orig.z;
            if axis == 0 {
                ray_dir_axis = ray_dir.x;
                ray_orig_axis = ray_orig.x;
            }
            else if axis == 1 {
                ray_dir_axis = ray_dir.y;
                ray_orig_axis = ray_orig.y;
            }
            let ax = Self::axis_interval(&self, axis);
            if ray_dir_axis == 0.0 {
                if ray_orig_axis >= ax.min && ray_orig_axis <= ax.max {
                    continue;
                }
                return false;
            }
            let adinv = 1.0 / ray_dir_axis;
            let t0 = (ax.min - ray_orig_axis ) * adinv;
            let t1 = (ax.max - ray_orig_axis) * adinv;
            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            }
            else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if (t0 < ray_t.max) {
                    ray_t.max = t0;
                }
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> i64 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {0} else {2}
        }
        else {
            if self.y.size() > self.z.size() {1} else {2}
        }
    }

    pub fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
}