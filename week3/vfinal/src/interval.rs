use nalgebra::Vector3;

#[derive(Clone)]
pub struct Interval {
    pub min:f64,
    pub max:f64,
}

impl Interval {
    pub fn initial() -> Self {
        Interval {
            min:f64::INFINITY,
            max:f64::NEG_INFINITY,
        }
    }

    pub fn new(mn:f64, mx:f64) -> Self {
        Interval {
            min:mn,
            max:mx,
        }
    }

    pub fn merge(a:&Interval, b:&Interval) -> Self {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x:f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x:f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x:f64) -> f64 {
        if x<self.min {
            return self.min;
        }
        if x>self.max {
            return self.max;
        }
        x
    }

    pub fn empty() -> Interval {
        Interval::new(f64::INFINITY, f64::NEG_INFINITY)
    }

    pub fn universe() -> Interval {
        Interval::new(f64::NEG_INFINITY, f64::INFINITY)
    }

    pub fn add(&self, displacement:f64) -> Interval {
        Interval::new(self.min + displacement, self.max + displacement)
    }

    pub fn expand(&self,delta:f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}
