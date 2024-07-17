use nalgebra::Vector3;

pub struct Onb {
    axis:[Vector3<f64>;3],
}

impl Onb {
    pub fn new() -> Self {
        Onb {
            axis:[Vector3::zeros();3],
        }
    }

    pub fn u(&self) -> Vector3<f64> {
        self.axis[0]
    }

    pub fn v(&self) -> Vector3<f64> {
        self.axis[1]
    }

    pub fn w(&self) -> Vector3<f64> {
        self.axis[2]
    }

    pub fn local(&self, a:f64, b:f64, c:f64) -> Vector3<f64> {
        a*self.axis[0] + b*self.axis[1] + c*self.axis[2]
    }

    pub fn local_vec(&self, a:&Vector3<f64>) -> Vector3<f64> {
        a.x*self.axis[0] + a.y*self.axis[1] + a.z*self.axis[2]
    }

    pub fn build_from_w(&mut self, w:&Vector3<f64>) {
        let unit_w = w.normalize();
        let a = if unit_w.x.abs() > 0.9 {Vector3::new(0.0,1.0,0.0)} else {Vector3::new(1.0,0.0,0.0)};
        let v = unit_w.cross(&a).normalize();
        let u = unit_w.cross(&v);
        self.axis[0] = u;
        self.axis[1] = v;
        self.axis[2] = unit_w;
    }

}