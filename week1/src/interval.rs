use nalgebra::Vector3;

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

    pub fn empty() -> Self {
        Self::new(f64::INFINITY, f64::NEG_INFINITY)
    }

    pub fn universe() -> Self {
        Self::new(f64::NEG_INFINITY, f64::INFINITY)
    }
}
