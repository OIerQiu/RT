use std::sync::Arc;

use nalgebra::Vector3;

use crate::{hittable::Hittable, onb::Onb, rtweekend::random_double, vec3::{random_cosine_direction, random_unit_vector}};

pub trait Pdf: Send + Sync {
    fn value(&self, direction:&Vector3<f64>) -> f64;
    fn generate(&self) -> Vector3<f64>;
}

pub struct SpherePdf {}

impl SpherePdf {
    pub fn new() -> Self {
        SpherePdf {}
    }
}

impl Pdf for SpherePdf {
    fn value(&self, direction:&Vector3<f64>) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }

    fn generate(&self) -> Vector3<f64> {
        random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw:Onb,
}

impl CosinePdf {
    pub fn new(w:&Vector3<f64>) -> Self {
        let mut uvw = Onb::new();
        uvw.build_from_w(&w);
        CosinePdf {
            uvw:uvw,
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction:&Vector3<f64>) -> f64 {
        let cosine_theta = direction.normalize().dot(&self.uvw.w());
        (cosine_theta/std::f64::consts::PI).max(0.0)
    }

    fn generate(&self) -> Vector3<f64> {
        self.uvw.local_vec(&random_cosine_direction())
    }
}

pub struct HittablePdf {
    objects:Arc<dyn Hittable>,
    origin:Vector3<f64>,
}

impl HittablePdf {
    pub fn new(objects:Arc<dyn Hittable>, origin:Vector3<f64>) -> Self {
        HittablePdf {
            objects:objects.clone(),
            origin:origin.clone(),
        }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction:&Vector3<f64>) -> f64 {
        self.objects.pdf_value(&self.origin, &direction)
    }

    fn generate(&self) -> Vector3<f64> {
        self.objects.random(&self.origin)
    }
}

pub struct MixturePdf {
    p:[Arc<dyn Pdf>;2],
}

impl MixturePdf {
    pub fn new (p0:Arc<dyn Pdf>, p1:Arc<dyn Pdf>) -> Self {
        MixturePdf {
            p:[p0.clone(),p1.clone()],
        }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction:&Vector3<f64>) -> f64 {
        0.5 * self.p[0].value(&direction) + 0.5 * self.p[1].value(&direction)
    }

    fn generate(&self) -> Vector3<f64> {
        if random_double() < 0.5 { self.p[0].generate() } else { self.p[1].generate() }
    }
}