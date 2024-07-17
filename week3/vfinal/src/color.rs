use nalgebra::Vector3;
use crate::interval::Interval;

pub fn linear_to_gamma(linear_component:f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn write_color(pixel_color:&Vector3<f64>) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;
    if r != r{
        r = 0.0;
    }
    if g != g {
        g = 0.0;
    }
    if b != b {
        b = 0.0;
    }
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);
    let intensity = Interval::new(0.000,0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i64;
    let gbyte = (256.0 * intensity.clamp(g)) as i64;
    let bbyte = (256.0 * intensity.clamp(b)) as i64;
    println!("{} {} {}",rbyte,gbyte,bbyte);
}