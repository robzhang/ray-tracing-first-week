mod color;
mod vector;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod world;
mod utils;
mod material;
mod lambertian;
mod metal;
mod dielectric;

use std::io::Write;
use std::fs::{File, OpenOptions};
use std::rc::Rc;

use crate::ray::Ray;
use crate::color::Color;
use crate::vector::{Point3,Vector3};
use crate::sphere::Sphere;
use crate::world::World;
use crate::camera::Camera;
use crate::utils::*;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::dielectric::Dielectric;

const ASPECT_RATIO: f64  = 16.0 / 9.0;
const WIDTH:usize = 400;
const HEIGHT:usize = (WIDTH as f64 / ASPECT_RATIO) as usize; 
const SAMPLES_PER_PIXEL:i32 = 100;
const MAX_DEPTH:i32 = 50;

static IMAGE_FILE: &str = "1.ppm";

fn main() {
    // 创建一个包含若干不同材质球体的3D世界
    let world = create_3d_world();

    // 创建相机
    let camera = create_camera();

    // 拍照
    let image_pixels = camera.take_photo(&world, WIDTH, HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH);
    
    // 保存照片
    save_image_to_file(String::from(IMAGE_FILE), image_pixels, WIDTH, HEIGHT);
}

fn create_3d_world() -> World {
    let mut world = World::new();

    // 创建用于大地的材质
    let ground_material = Rc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_material.clone())));

    for a in -11..11  {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(a as f64 + 0.9*random_f64(), 0.2, b as f64 + 0.9*random_f64());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // 漫反射材质球
                    let albedo = Color::random_color().mul_color(&Color::random_color());
                    let sphere_material = Rc::new(Lambertian::new(&albedo));
                    let mut sphere = Box::new(Sphere::new(center, 0.2, sphere_material.clone()));
                    sphere.move_to(&(center + Vector3::new(0.0, random_f64_range(0.0, 0.5), 0.0)), 0.0, 1.0);
                    world.add(sphere);
                } else if choose_mat < 0.95 {
                    // 金属球
                    let albedo = Color::random_color_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(&albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    // 玻璃类材质球
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1.clone())));

    let material2 = Rc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2.clone())));

    let material3 = Rc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3.clone())));

    return world;
}

fn create_camera() -> Camera {
    let  lookfrom = Point3::new(13.0,2.0,3.0);
    let lookat = Point3::new(0.0,0.0,0.0);
    let vup = Vector3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0)
}

fn save_image_to_file(name: String, image: Vec<Color>, image_width: usize, image_height: usize) {
    let mut f = create_ppm_file(name, image_width, image_height);

    for c in &image {
        write_ppm_file_with_color(&mut f, c);
    }
}

fn create_ppm_file(name: String, image_width: usize, image_height: usize) -> File {
    let mut f = OpenOptions::new()    
        .create(true) // 新建，若文件存在则打开这个文件
        .write(true)   
        .truncate(true)
        .open(name).unwrap();     
        
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);

    f.write(header.as_bytes()).expect("write header");

    f
}

fn write_ppm_file_with_color(ppm: &mut File, color: &Color) {
    let r = color.f64_r().sqrt();
    let g = color.f64_g().sqrt();
    let b = color.f64_b().sqrt();

    let line = format!("{} {} {}\n", 
            (256.0 * clamp(r, 0.0, 0.999)) as u8, 
            (256.0 * clamp(g, 0.0, 0.999)) as u8, 
            (256.0 * clamp(b, 0.0, 0.999)) as u8
    );

    ppm.write(line.as_bytes()).expect("write color");
}
