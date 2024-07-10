use nalgebra::Vector3;

pub struct Ray{
    pub orig:Vector3<f64>,
    pub dir:Vector3<f64>,
    pub tm:f64,
}

impl Ray {
    pub fn new(origin:Vector3<f64>, direction:Vector3<f64>) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
            tm: 0.0,
        }
    }
    pub fn new_zero() -> Self {
        Ray {
            orig: Vector3::zeros(),
            dir: Vector3::zeros(),
            tm: 0.0,
        }
    }
    pub fn initial(origin:Vector3<f64>, direction:Vector3<f64>, time:f64) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
            tm: time,
        }
    }
    pub fn origin(&self) -> Vector3<f64> {
        self.orig
    }
    pub fn direction(&self) -> Vector3<f64> {
        self.dir
    }
    pub fn at(&self, t:f64) -> Vector3<f64> {
        self.orig + t*self.dir
    }
    pub fn time(&self) -> f64 {
        self.tm
    }
}