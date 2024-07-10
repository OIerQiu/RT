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

pub fn bouncing_spheres() {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::initial(0.32, &Vector3::new(0.2,0.3,0.1), &Vector3::new(0.9,0.9,0.9)));
    world.add(Arc::new(Sphere::new(Vector3::new(0.0,-1000.0,0.0),1000.0,Arc::new(Lambertian::initial(checker)))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vector3::new(a as f64 + 0.9*random_double(),0.2,b as f64 + 0.9*random_double());
            if (center-Vector3::new(4.0,0.2,0.0)).dot(&(center-Vector3::new(4.0,0.2,0.0))).sqrt() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = vec3_random().component_mul(&vec3_random());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vector3::new(0.0, random_f64(0.0,0.5), 0.0);
                    world.add(Arc::new(Sphere::initial(center, center2, 0.2, sphere_material)));
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

    let mut world:Arc<dyn Hittable> = Arc::new(BvhNode::initial(&mut world));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width  = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vector3::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Vector3::new(13.0,2.0,3.0);
    cam.lookat = Vector3::new(0.0,0.0,0.0);
    cam.vup = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}

pub fn checkered_spheres() {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::initial(0.32, &Vector3::new(0.2,0.3,0.1), &Vector3::new(0.9,0.9,0.9)));
    world.add(Arc::new(Sphere::new(Vector3::new(0.0,-10.0,0.0),10.0,Arc::new(Lambertian::initial(checker.clone())))));
    world.add(Arc::new(Sphere::new(Vector3::new(0.0,10.0,0.0),10.0,Arc::new(Lambertian::initial(checker.clone())))));
    let mut world:Arc<dyn Hittable> = Arc::new(world);

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width  = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vector3::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Vector3::new(13.0,2.0,3.0);
    cam.lookat = Vector3::new(0.0,0.0,0.0);
    cam.vup = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);

}

pub fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::initial(earth_texture));  
    let globe = Arc::new(Sphere::new(Vector3::new(0.0,0.0,0.0), 2.0, earth_surface));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width  = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vector3::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Vector3::new(0.0,0.0,12.0);
    cam.lookat = Vector3::new(0.0,0.0,0.0);
    cam.vup = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    let world:Arc<dyn Hittable> = Arc::new(HittableList::initial(globe));
    cam.render(&world);

}

pub fn perlin_spheres() {
    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::initial(4.0));
    let pertext2 = pertext.clone();
    world.add(Arc::new(Sphere::new(Vector3::new(0.0,-1000.0,0.0), 1000.0, Arc::new(Lambertian::initial(pertext)))));
    world.add(Arc::new(Sphere::new(Vector3::new(0.0,2.0,0.0), 2.0, Arc::new(Lambertian::initial(pertext2)))));
    let world:Arc<dyn Hittable> = Arc::new(world);
    
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width  = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vector3::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Vector3::new(13.0,2.0,3.0);
    cam.lookat = Vector3::new(0.0,0.0,0.0);
    cam.vup = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

pub fn quads() {
    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::new(Vector3::new(1.0,0.2,0.2)));
    let back_green = Arc::new(Lambertian::new(Vector3::new(0.2,1.0,0.2)));
    let right_blue = Arc::new(Lambertian::new(Vector3::new(0.2,0.2,1.0)));
    let upper_orange = Arc::new(Lambertian::new(Vector3::new(1.0,0.5,0.0)));
    let lower_teal = Arc::new(Lambertian::new(Vector3::new(0.2,0.8,0.8)));

    world.add(Arc::new(Quad::new(Vector3::new(-3.0,-2.0,5.0), Vector3::new(0.0,0.0,-4.0), Vector3::new(0.0,4.0,0.0), left_red)));
    world.add(Arc::new(Quad::new(Vector3::new(-2.0,-2.0,0.0), Vector3::new(4.0,0.0,0.0), Vector3::new(0.0,4.0,0.0), back_green)));
    world.add(Arc::new(Quad::new(Vector3::new(3.0,-2.0,1.0), Vector3::new(0.0,0.0,4.0), Vector3::new(0.0,4.0,0.0), right_blue)));
    world.add(Arc::new(Quad::new(Vector3::new(-2.0,3.0,1.0), Vector3::new(4.0,0.0,0.0), Vector3::new(0.0,0.0,4.0), upper_orange)));
    world.add(Arc::new(Quad::new(Vector3::new(-2.0,-3.0,5.0), Vector3::new(4.0,0.0,0.0), Vector3::new(0.0,0.0,-4.0), lower_teal)));
    let world:Arc<dyn Hittable> = Arc::new(world);

    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width  = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vector3::new(0.7, 0.8, 1.0);

    cam.vfov = 80.0;
    cam.lookfrom = Vector3::new(0.0,0.0,9.0);
    cam.lookat = Vector3::new(0.0,0.0,0.0);
    cam.vup = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

pub fn simple_light() {
    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::initial(4.0));
    let pertext2 = pertext.clone();
    world.add(Arc::new(Sphere::new(Vector3::new(0.0,-1000.0,0.0), 1000.0, Arc::new(Lambertian::initial(pertext)))));
    world.add(Arc::new(Sphere::new(Vector3::new(0.0,2.0,0.0), 2.0, Arc::new(Lambertian::initial(pertext2)))));
    let difflight = Arc::new(DiffuseLight::initial(Vector3::new(4.0,4.0,4.0)));
    let difflight2 = difflight.clone();
    world.add(Arc::new(Sphere::new(Vector3::new(0.0,7.0,0.0),2.0,difflight)));
    world.add(Arc::new(Quad::new(Vector3::new(3.0,1.0,-2.0),Vector3::new(2.0,0.0,0.0),Vector3::new(0.0,2.0,0.0),difflight2)));
    let world:Arc<dyn Hittable> = Arc::new(world);
    
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width  = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vector3::new(0.0, 0.0, 0.0);

    cam.vfov = 20.0;
    cam.lookfrom = Vector3::new(26.0,3.0,6.0);
    cam.lookat = Vector3::new(0.0,2.0,0.0);
    cam.vup = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

pub fn cornell_box () {
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

    let box2 = abox(&Vector3::new(0.0,0.0,0.0),&Vector3::new(165.0,165.0,165.0),&white);
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vector3::new(130.0,0.0,65.0)));
    world.add(box2);

    let world:Arc<dyn Hittable> = Arc::new(world);

    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width  = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Vector3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Vector3::new(278.0,278.0,-800.0);
    cam.lookat = Vector3::new(278.0,278.0,0.0);
    cam.vup = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

pub fn cornell_smoke () {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Vector3::new(0.65, 0.05, 0.05)));
    let white:Arc<dyn Material>= Arc::new(Lambertian::new(Vector3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vector3::new(0.12,0.45,0.15)));
    let light = Arc::new(DiffuseLight::initial(Vector3::new(7.0,7.0,7.0)));

    world.add(Arc::new(Quad::new(Vector3::new(555.0,0.0,0.0), Vector3::new(0.0,555.0,0.0), Vector3::new(0.0,0.0,555.0), green)));
    world.add(Arc::new(Quad::new(Vector3::new(0.0,0.0,0.0), Vector3::new(0.0,555.0,0.0), Vector3::new(0.0,0.0,555.0), red)));
    world.add(Arc::new(Quad::new(Vector3::new(113.0,554.0,127.0), Vector3::new(330.0,0.0,0.0), Vector3::new(0.0,0.0,305.0), light)));
    world.add(Arc::new(Quad::new(Vector3::new(0.0,555.0,0.0), Vector3::new(555.0,0.0,0.0), Vector3::new(0.0,0.0,555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vector3::new(0.0,0.0,0.0), Vector3::new(555.0,0.0,0.0), Vector3::new(0.0,0.0,555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vector3::new(0.0,0.0,555.0), Vector3::new(555.0,0.0,0.0), Vector3::new(0.0,555.0,0.0), white.clone())));
    
    let box1 = abox(&Vector3::new(0.0,0.0,0.0),&Vector3::new(165.0,330.0,165.0),&white);
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vector3::new(265.0,0.0,295.0)));
    world.add(Arc::new(ConstantMedium::initial(box1, 0.01, Vector3::new(0.0,0.0,0.0))));

    let box2 = abox(&Vector3::new(0.0,0.0,0.0),&Vector3::new(165.0,165.0,165.0),&white);
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vector3::new(130.0,0.0,65.0)));
    world.add(Arc::new(ConstantMedium::initial(box2, 0.01, Vector3::new(1.0,1.0,1.0))));

    let world:Arc<dyn Hittable> = Arc::new(world);

    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width  = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Vector3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Vector3::new(278.0,278.0,-800.0);
    cam.lookat = Vector3::new(278.0,278.0,0.0);
    cam.vup = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

pub fn final_scene(image_width:i64, samples_per_pixel:i64, max_depth:i64) {
    let mut boxes1 = HittableList::new();
    let ground:Arc<dyn Material> = Arc::new(Lambertian::new(Vector3::new(0.48,0.83,0.53)));

    let boxes_per_side:i64 = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64(1.0,101.0);
            let z1 = z0 + w;

            boxes1.add(abox(&Vector3::new(x0,y0,z0), &Vector3::new(x1,y1,z1), &ground));
        }
    }

    let mut world = HittableList::new();
    world.add(Arc::new(BvhNode::initial(&mut boxes1)));

    let light = Arc::new(DiffuseLight::initial(Vector3::new(7.0,7.0,7.0)));
    world.add(Arc::new(Quad::new(Vector3::new(123.0,554.0,147.0), Vector3::new(300.0,0.0,0.0), Vector3::new(0.0,0.0,265.0), light)));

    let center1 = Vector3::new(400.0,400.0,200.0);
    let center2 = center1 + Vector3::new(30.0,0.0,0.0);
    let sphere_material = Arc::new(Lambertian::new(Vector3::new(0.7,0.3,0.1)));
    world.add(Arc::new(Sphere::initial(center1, center2, 50.0, sphere_material)));

    world.add(Arc::new(Sphere::new(Vector3::new(260.0,150.0,45.0),50.0, Arc::new(Dielectric::new(1.5)))));
    world.add(Arc::new(Sphere::new(Vector3::new(0.0,150.0,145.0), 50.0, Arc::new(Metal::new(Vector3::new(0.8,0.8,0.9),1.0)))));

    let boundary = Arc::new(Sphere::new(Vector3::new(360.0,150.0,145.0), 70.0, Arc::new(Dielectric::new(1.5))));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::initial(boundary, 0.2, Vector3::new(0.2,0.4,0.9))));
    let boundary = Arc::new(Sphere::new(Vector3::new(0.0,0.0,0.0), 5000.0, Arc::new(Dielectric::new(1.5))));
    world.add(Arc::new(ConstantMedium::initial(boundary, 0.0001, Vector3::new(1.0,1.0,1.0))));

    let emat = Arc::new(Lambertian::initial(Arc::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Arc::new(Sphere::new(Vector3::new(400.0,200.0,400.0), 100.0, emat)));
    let pertext = Arc::new(NoiseTexture::initial(0.2));
    world.add(Arc::new(Sphere::new(Vector3::new(220.0,280.0,300.0), 80.0, Arc::new(Lambertian::initial(pertext)))));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Vector3::new(0.73,0.73,0.73)));
    let ns = 1000;
    for j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(vec3_rand(0.0,165.0), 10.0, white.clone())));
    }
    world.add(Arc::new(Translate::new(Arc::new(RotateY::new(Arc::new(BvhNode::initial(&mut boxes2)), 15.0)), Vector3::new(-100.0,270.0,395.0))));

    let world:Arc<dyn Hittable> = Arc::new(world);

    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width  = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Vector3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Vector3::new(478.0,278.0,-600.0);
    cam.lookat = Vector3::new(278.0,278.0,0.0);
    cam.vup = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {

    match 9 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        10 => final_scene(400, 250, 4),
        _ => todo!(),
    }
}
