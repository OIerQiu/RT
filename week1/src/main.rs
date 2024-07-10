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

use nalgebra::Vector3;
use std::sync::Arc;

use crate::color::write_color;
use crate::ray::Ray;
use crate::hittable::{HitRecord,Hittable};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::interval::Interval;
use crate::camera::Camera;
use crate::material::{Material, Lambertian, Metal, Dielectric};
use crate::rtweekend::{random_double, random_f64};
use crate::vec3::{vec3_random, vec3_rand};

fn main() {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Vector3::new(0.5,0.5,0.5)));
    world.add(Arc::new(Sphere::new(Vector3::new(0.0,-1000.0,0.0),1000.0,ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vector3::new(a as f64 + 0.9*random_double(),0.2,b as f64 + 0.9*random_double());
            if (center-Vector3::new(4.0,0.2,0.0)).dot(&(center-Vector3::new(4.0,0.2,0.0))).sqrt() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = vec3_random().component_mul(&vec3_random());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
                else if choose_mat < 0.95 {
                    let albedo = vec3_rand(0.5,1.0);
                    let fuzz = random_f64(0.0,0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
                else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Vector3::new(0.0,1.0,0.0), 1.0, material1)));
    let material2 = Arc::new(Lambertian::new(Vector3::new(0.4,0.2,0.1)));
    world.add(Arc::new(Sphere::new(Vector3::new(-4.0,1.0,0.0), 1.0, material2)));
    let material3 = Arc::new(Metal::new(Vector3::new(0.7,0.6,0.5),0.0));
    world.add(Arc::new(Sphere::new(Vector3::new(4.0,1.0,0.0), 1.0, material3)));


    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width  = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Vector3::new(13.0,2.0,3.0);
    cam.lookat = Vector3::new(0.0,0.0,0.0);
    cam.vup = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}