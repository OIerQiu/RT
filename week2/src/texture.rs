use nalgebra::Vector3;
use std::sync::Arc;
use crate::color;
use crate::rtw_stb_image::RtwImage;
use crate::interval::Interval;
use crate::perlin::Perlin;

pub trait Texture: Send + Sync {
    fn value (&self, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64>;
}

pub struct SolidColor {
    albedo:Vector3<f64>
}

impl SolidColor {
    pub fn new (color:Vector3<f64>) -> Self {
        SolidColor {
            albedo:color.clone(),
        }
    }

    pub fn initial (red:f64, green:f64, blue:f64) -> Self {
        SolidColor {
            albedo:Vector3::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value (&self, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64> {
        self.albedo.clone()
    }
}

pub struct CheckerTexture {
    inv_scale:f64,
    even:Arc<dyn Texture>,
    odd:Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale:f64, even:Arc<dyn Texture>, odd:Arc<dyn Texture>) -> Self {
        CheckerTexture {
            inv_scale:1.0/scale,
            even:even,
            odd:odd,
        }
    }

    pub fn initial (scale:f64, c1:&Vector3<f64>, c2:&Vector3<f64>) -> Self {
        CheckerTexture {
            inv_scale:1.0/scale,
            even:Arc::new(SolidColor::new(c1.clone())),
            odd:Arc::new(SolidColor::new(c2.clone())),
        }
    }
}

impl Texture for CheckerTexture {
    fn value (&self, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64> {
        let x_integer = (self.inv_scale * p.x - 0.5).round() as i64;
        let y_integer = (self.inv_scale * p.y - 0.5).round() as i64;
        let z_integer = (self.inv_scale * p.z - 0.5).round() as i64;
        let is_even = (x_integer + y_integer + z_integer ) % 2 == 0;
        if is_even {self.even.value(u,v,&p)} else {self.odd.value(u,v,&p)}
    }
}

pub struct ImageTexture {
    image:RtwImage,
}

impl ImageTexture {
    pub fn new (filename:&str) -> Self {
        ImageTexture {
            image:RtwImage::new(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value (&self, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64> {
        if self.image.height() <= 0 {
            return Vector3::new(0.0,1.0,1.0);
        }
        let u = Interval::new(0.0,1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0,1.0).clamp(v);
        let i = (u * self.image.width() as f64) as i64;
        let j = (v * self.image.height() as f64) as i64;
        let pixel = self.image.pixel_data(i, j);
        let color_scale = 1.0 / 255.0;
        Vector3::new(color_scale * pixel[0] as f64, color_scale * pixel[1] as f64, color_scale * pixel[2] as f64)
    }
}

pub struct NoiseTexture {
    noise:Perlin,
    scale:f64,
}

impl NoiseTexture {
    pub fn new () -> Self {
        NoiseTexture {
            noise:Perlin::new(),
            scale:1.0,
        }
    }

    pub fn initial (scale:f64) -> Self {
        NoiseTexture {
            noise:Perlin::new(),
            scale:scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value (&self, u:f64, v:f64, p:&Vector3<f64>) -> Vector3<f64> {
        Vector3::new(0.5,0.5,0.5) * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(&p, 7)).sin())
    }
}