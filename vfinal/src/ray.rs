use nalgebra::Vector3;

pub struct Ray{
    pub orig:Vector3<f64>,
    pub dir:Vector3<f64>,
}

impl Ray {
    pub fn new(origin:Vector3<f64>, direction:Vector3<f64>) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
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
}