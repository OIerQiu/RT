use nalgebra::Vector3;
use crate::rtweekend::{random_double,random_f64};

pub fn vec3_random() -> Vector3<f64> {
    Vector3::new(random_double(),random_double(),random_double())
}
pub fn vec3_rand(min:f64, max:f64) -> Vector3<f64> {
    Vector3::new(random_f64(min,max),random_f64(min,max),random_f64(min,max))
}

pub fn reflect(v:&Vector3<f64>, n:&Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(&n) * n
}

pub fn refract(uv:&Vector3<f64>, n:&Vector3<f64>, etai_over_etat:f64) -> Vector3<f64> {
    let cos_theta = -uv.dot(&n).min(1.0);
    let r_out_perp = etai_over_etat * (uv+cos_theta*n);
    let r_out_parallel = -(1.0 - r_out_perp.dot(&r_out_perp)).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = vec3_rand(-1.0,1.0);
        if p.dot(&p) < 1.0 && p.dot(&p) > 0.0{
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vector3<f64> {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal:&Vector3<f64>) -> Vector3<f64> {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(&normal) > 0.0 {
        return on_unit_sphere;
    }
    -on_unit_sphere
}

pub fn random_in_unit_disk() -> Vector3<f64> {
    loop {
        let p = Vector3::new(random_f64(-1.0,1.0),random_f64(-1.0,1.0),0.0);
        if p.dot(&p) < 1.0 && p.dot(&p) > 0.0 {
            return p;
        }
    }
}