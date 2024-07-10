use nalgebra::Vector3;

use rand::Rng;
use rand::distributions::Uniform;

pub fn near_zero(v:&Vector3<f64>) -> bool {
    let s = 1e-8;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.sample(Uniform::new(0.0, 1.0))
}

pub fn random_f64(min:f64, max:f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.sample(Uniform::new(min, max))
}
