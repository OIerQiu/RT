use image::{DynamicImage, GenericImageView};
use std::path::Path;
use nalgebra::Vector3;

#[derive(Clone)]
pub struct RtwImage {
    pub image_width:i64,
    pub image_height:i64,
    pub data:Option<DynamicImage>,
}

impl RtwImage {
    pub fn initial () -> Self {
        RtwImage {
            image_width:0,
            image_height:0,
            data:None,
        }
    }

    pub fn new (image_filename:&str) -> Self {
        let filename = image_filename.to_string();
        let mut image = Self::initial();
        let search_path = [
            "images",
            "../images",
            "../../images",
            "../../../images",
            "../../../../images",
            "../../../../../images",
            "../../../../../../images",
        ];
        let mut found = false;
        for pre_path in search_path.iter() {
            let path = Path::new(pre_path).join(&filename);
            if let Ok(img) = image::open(&path) {
                let img = img.clone();
                image = Self {
                    image_width:img.width() as i64,
                    image_height:img.height() as i64,
                    data:Some(img),
                };
                found = true;
                break;
            }
        }
        if !found {
            eprintln!("ERROR: Could not load image file '{}'.", image_filename);
        }
        image
    }

    pub fn clamp(x:i64, low:i64, high:i64) -> i64 {
        if x < low {
            return low;
        }
        if x < high {
            return x;
        }
        high - 1
    }

    pub fn width (&self) -> i64 {
        self.image_width
    }

    pub fn height (&self) -> i64 {
        self.image_height
    }

    pub fn pixel_data(&self, x:i64, y:i64) -> Vector3<i64> {
        if let Some(img) = &self.data {
            let x = Self::clamp(x, 0, self.image_width);
            let y = Self::clamp(y, 0, self.image_height);
            let pixel = img.get_pixel(x as u32, y as u32);
            Vector3::new(pixel[0] as i64, pixel[1] as i64, pixel[2] as i64)
        }
        else {
            Vector3::new(255, 0, 255)
        }
    }
}