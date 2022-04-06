use core::f64::INFINITY;

use crate::ray::Ray;
use crate::vector::{Vector3, Point3}; 
use crate::utils::degrees_to_radians;
use crate::world::World;
use crate::color::Color;
use crate::utils::random_f64;

pub struct Camera {
    origin: Point3, // 镜头位置
    lower_left_corner: Point3, // 视窗左下角的坐标
    horizontal: Vector3, // focus plane水平宽度及方向
    vertical: Vector3, // focus plane垂直高度及方向

    u: Vector3,
    v: Vector3,
    //w: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vector3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_distance: f64) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // 右手坐标系，w指向正后方
        let w = (lookfrom - lookat).unit();
        //向量vup决定了相机绕w轴的旋转，于是可以用vup和w的叉积来确定相机的x轴（相机的正右方）
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * (viewport_width * focus_distance);
        let vertical = v * (viewport_height * focus_distance);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w*focus_distance;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,v,lens_radius,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) ->Ray {//离焦模糊算法
        let rd = Vector3::random_in_unit_disk() * self.lens_radius;
        //在相机的xy平面偏移
        let offset = self.u * rd.x + self.v * rd.y;

        //只有focus plane上的点可以完美成像，离该平面越远越模糊（也就是不能在viewport平面上聚焦）
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset)
    }

    pub fn take_photo(&self, world: &World, image_width: usize, image_height: usize, samples_per_pixel: i32, max_depth: i32) ->Vec<Color> {
        let mut image_pixels = Vec::new();

        for i in (0 .. image_height).rev() {
            println!("rendering remaining: {}", i);
            for j in 0 .. image_width {
                let mut pixel_color = Color::black();
    
                let mut k = 0;
                while k < samples_per_pixel {
                    let u = (j as f64 + random_f64())/ ((image_width-1) as f64);
                    let v = (i as f64 + random_f64()) / ((image_height-1) as f64);
    
                    let ray = self.get_ray(u, v);//Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
                
                    let color = self.ray_color(&ray, &world, max_depth);
    
                    pixel_color = pixel_color + color;
    
                    k = k + 1;
                }
                
                let pixel_color = pixel_color / samples_per_pixel as f64;
                image_pixels.push(pixel_color);
            }
        }

        image_pixels
    }

    fn ray_color(&self, ray: &Ray, world: &World, depth: i32) ->Color {
        if depth <= 0 {
            return Color::black();
        }
    
        if let Some(hit_info) = world.hit(ray, 0.001, INFINITY) { //射线ray与球面相交
            if let Some(scatter_info) = hit_info.material.scatter(ray, &hit_info) {
                return scatter_info.attenuation.mul_color(&self.ray_color(&scatter_info.scattered, world, depth-1));
            }
                
            return Color::black();
        }
    
        //背景渐变色
        let unit_dir = ray.dir.unit();
    
        let t = 0.5*(unit_dir.y + 1.0);
    
        Color::new(1.0 - 0.5 * t, 1.0 - 0.3 * t, 1.0)
    }
}