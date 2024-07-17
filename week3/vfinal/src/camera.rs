use nalgebra::Vector3;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use crate::pdf::{CosinePdf, HittablePdf, MixturePdf, Pdf};
use crate::ray::Ray;
use crate::hittable_list::HittableList;
use crate::hittable::{HitRecord,Hittable};
use crate::interval::Interval;
use crate::color::write_color;
use crate::rtweekend::{random_double, random_f64};
use crate::vec3::{random_on_hemisphere, random_unit_vector, random_in_unit_disk};
use crate::material::{Material, ScatterRecord};

#[derive(Clone)]
pub struct Camera {
    pub aspect_ratio:f64,
    pub image_width:i64,
    pub samples_per_pixel:i64,
    pub max_depth:i64,
    pub background:Vector3<f64>,
    pub vfov:f64,
    pub lookfrom:Vector3<f64>,
    pub lookat:Vector3<f64>,
    pub vup:Vector3<f64>,
    pub defocus_angle:f64,
    pub focus_dist:f64,

    pub image_height:i64,
    pub pixel_samples_scale:f64,
    pub sqrt_spp:i64,
    pub recip_sqrt_spp:f64,
    pub center:Vector3<f64>,
    pub pixel00_loc:Vector3<f64>,
    pub pixel_delta_u:Vector3<f64>,
    pub pixel_delta_v:Vector3<f64>,
    pub u:Vector3<f64>,
    pub v:Vector3<f64>,
    pub w:Vector3<f64>,
    pub defocus_disk_u:Vector3<f64>,
    pub defocus_disk_v:Vector3<f64>,
}

impl Camera {
    pub fn new () -> Self {
        Camera {
            aspect_ratio:1.0,
            image_width:100,
            samples_per_pixel:10,
            max_depth:10,
            background:Vector3::zeros(),
            vfov:90.0,
            lookfrom:Vector3::new(0.0,0.0,0.0),
            lookat:Vector3::new(0.0,0.0,-1.0),
            vup:Vector3::new(0.0,1.0,0.0),
            defocus_angle:0.0,
            focus_dist:10.0,

            image_height:100,
            pixel_samples_scale:0.0,
            sqrt_spp:0,
            recip_sqrt_spp:0.0,
            center:Vector3::new(0.0,0.0,0.0),
            pixel00_loc:Vector3::new(0.0,0.0,0.0),
            pixel_delta_u:Vector3::new(0.0,0.0,0.0),
            pixel_delta_v:Vector3::new(0.0,0.0,0.0),
            u:Vector3::new(0.0,0.0,0.0),
            v:Vector3::new(0.0,0.0,0.0),
            w:Vector3::new(0.0,0.0,0.0),
            defocus_disk_u:Vector3::new(0.0,0.0,0.0),
            defocus_disk_v:Vector3::new(0.0,0.0,0.0),
        }
    }
    pub fn ray_color (&self, r:&Ray, depth:i64, world:&Arc<dyn Hittable>, lights:&Arc<dyn Hittable>) -> Vector3<f64> {
        if depth <= 0 {
            return Vector3::new(0.0,0.0,0.0);
        }
        let mut rec = HitRecord::new();

        if !world.hit(&r,&Interval::new(0.001, f64::INFINITY),&mut rec) {
            return self.background;
        }

        let mut srec = ScatterRecord::new();
        let color_from_emission = rec.mat.emitted(&r, &rec, rec.u, rec.v, &rec.p);
        if !rec.mat.scatter(&r, &rec, &mut srec) {
            return color_from_emission;
        }

        if srec.skip_pdf {
            return srec.attenuation.component_mul(&Self::ray_color(&self, &srec.skip_pdf_ray, depth-1,  &world, &lights));
        }

        let light_ptr = Arc::new(HittablePdf::new(lights.clone(), rec.p.clone()));
        let p = MixturePdf::new(light_ptr, srec.pdf_ptr.clone());

        let scattered = Ray::initial(rec.p.clone(), p.generate(), r.time());
        let pdf_val = p.value(&scattered.direction()); 

        let scattering_pdf = rec.mat.scattering_pdf(&r, &rec, &scattered);
        
        let sample_color = Self::ray_color(&self, &scattered, depth-1, &world, &lights);
        let color_from_scatter = (srec.attenuation * scattering_pdf).component_mul(&sample_color) / pdf_val;
        color_from_emission + color_from_scatter
    }

    pub fn render(&mut self, world:&Arc<dyn Hittable>, lights:&Arc<dyn Hittable>) {
        self.initialize();
        println!("P3");
        println!("{} {}",self.image_width, self.image_height);
        println!("255");
        let wself = Arc::new(self.clone());
        let wworld = Arc::new(world.clone());
        let wlights = Arc::new(lights.clone());

        let mut pb = ProgressBar::new(self.image_height as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {percent:>7}%"));
        for j in 0..self.image_height {
            pb.set_position(j as u64);
            let mut line_pixel_color = Arc::new(Mutex::new(Vec::new()));
            for i in 0..self.image_width {
                line_pixel_color.lock().unwrap().push(Vector3::new(0.0,0.0,0.0));
            }
            let thread_num:i64 = 28;
            let mut threads = Vec::new();
            for k in 0..thread_num {
                let wself = Arc::clone(&wself);
                let wworld = Arc::clone(&wworld);
                let wlights = Arc::clone(&wlights);
                let mut line_pixel_color = Arc::clone(&line_pixel_color);
                let render_thread = thread::spawn(move || {
                    let mut i = k;
                    while i < wself.image_width {
                        let mut pixel_color = Vector3::new(0.0,0.0,0.0);
                        for s_j in 0..wself.sqrt_spp{
                            for s_i in 0..wself.sqrt_spp {
                                let r = Self::get_ray(&wself, i, j, s_i, s_j);
                                pixel_color += Self::ray_color(&wself, &r, wself.max_depth, &wworld, &wlights);
                            }
                        }
                        (line_pixel_color.lock().unwrap())[i as usize] = pixel_color.clone();
                        i += thread_num;
                    }
                });
                threads.push(render_thread);
            }
            for render_thread in threads {
                render_thread.join().unwrap();
            }
            let pixel_color = line_pixel_color.lock().unwrap().clone();
            for i in 0..self.image_width {
                write_color(&(self.pixel_samples_scale * pixel_color[i as usize]));
            }
        }
        pb.finish_and_clear();
    }

    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i64;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as i64;
        self.pixel_samples_scale = 1.0 / (self.sqrt_spp * self.sqrt_spp) as f64;
        self.recip_sqrt_spp = 1.0 / self.sqrt_spp as f64;

        self.center = self.lookfrom;

        let theta = self.vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.w = (self.lookfrom-self.lookat).normalize();
        self.u = self.vup.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);
        
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;
        let viewport_upper_left = self.center - (self.focus_dist * self.w)-viewport_u/2.0-viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        let defocus_radius = self.focus_dist * (self.defocus_angle.to_radians() / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn get_ray(&self,i:i64, j:i64, s_i:i64, s_j:i64) -> Ray {
        let offset = Self::sample_square_straitified(&self, s_i, s_j);
        let pixel_sample = self.pixel00_loc + (i as f64+offset.x) * self.pixel_delta_u + (j as f64+offset.y) * self.pixel_delta_v;
        let mut ray_origin = self.center;
        if self.defocus_angle > 0.0{
            ray_origin = Self::defocus_disk_sample(&self);
        }
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();
        Ray::initial(ray_origin, ray_direction, ray_time)
    }

    pub fn sample_square_straitified(&self, s_i:i64, s_j:i64) -> Vector3<f64> {
        let px = ((s_i as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;

        Vector3::new(px, py, 0.0)
    }

    pub fn sample_square() -> Vector3<f64> {
        Vector3::new(random_double()-0.5,random_double()-0.5,0.0)
    }

    pub fn defocus_disk_sample(&self) -> Vector3<f64> {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}