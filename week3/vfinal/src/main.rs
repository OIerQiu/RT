mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod rtweekend;
mod vec3;
mod material;
mod aabb;
mod bvh;
mod texture;
mod rtw_stb_image;
mod perlin;
mod quad;
mod constant_medium;
mod onb;
mod pdf;

use constant_medium::ConstantMedium;
use nalgebra::Vector3;
use std::sync::Arc;

use crate::color::write_color;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable, Translate, RotateY};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::interval::Interval;
use crate::camera::Camera;
use crate::material::{Material, Lambertian, Metal, Dielectric, DiffuseLight};
use crate::rtweekend::{random_double, random_f64};
use crate::vec3::{vec3_random, vec3_rand};
use crate::bvh::BvhNode;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::quad::{Quad, abox};

fn main() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Vector3::new(0.65, 0.05, 0.05)));
    let white:Arc<dyn Material>= Arc::new(Lambertian::new(Vector3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vector3::new(0.12,0.45,0.15)));
    let light = Arc::new(DiffuseLight::initial(Vector3::new(15.0,15.0,15.0)));

    world.add(Arc::new(Quad::new(Vector3::new(555.0,0.0,0.0), Vector3::new(0.0,555.0,0.0), Vector3::new(0.0,0.0,555.0), green)));
    world.add(Arc::new(Quad::new(Vector3::new(0.0,0.0,0.0), Vector3::new(0.0,555.0,0.0), Vector3::new(0.0,0.0,555.0), red)));
    world.add(Arc::new(Quad::new(Vector3::new(343.0,554.0,332.0), Vector3::new(-130.0,0.0,0.0), Vector3::new(0.0,0.0,-105.0), light)));
    world.add(Arc::new(Quad::new(Vector3::new(0.0,0.0,0.0), Vector3::new(555.0,0.0,0.0), Vector3::new(0.0,0.0,555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vector3::new(555.0,555.0,555.0), Vector3::new(-555.0,0.0,0.0), Vector3::new(0.0,0.0,-555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vector3::new(0.0,0.0,555.0), Vector3::new(555.0,0.0,0.0), Vector3::new(0.0,555.0,0.0), white.clone())));
    
    let box1 = abox(&Vector3::new(0.0,0.0,0.0),&Vector3::new(165.0,330.0,165.0),&white);
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vector3::new(265.0,0.0,295.0)));
    world.add(box1);

    /*
    let box2 = abox(&Vector3::new(0.0,0.0,0.0),&Vector3::new(165.0,165.0,165.0),&white);
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vector3::new(130.0,0.0,65.0)));
    world.add(box2);*/
    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Vector3::new(190.0,90.0,190.0), 90.0, glass)));

    let world:Arc<dyn Hittable> = Arc::new(world);

    let mut lights = HittableList::new();
    let m = Arc::new(DiffuseLight::initial(Vector3::new(15.0, 15.0, 15.0)));
    lights.add(Arc::new(Quad::new(Vector3::new(343.0,554.0,332.0), Vector3::new(-130.0,0.0,0.0), Vector3::new(0.0,0.0,-105.0), m.clone())));
    lights.add(Arc::new(Sphere::new(Vector3::new(190.0,90.0,190.0), 90.0, m)));
    let lights:Arc<dyn Hittable> = Arc::new(lights);

    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width  = 600;
    cam.samples_per_pixel = 1000;
    cam.max_depth = 50;
    cam.background = Vector3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Vector3::new(278.0,278.0,-800.0);
    cam.lookat = Vector3::new(278.0,278.0,0.0);
    cam.vup = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world, &lights);
}
