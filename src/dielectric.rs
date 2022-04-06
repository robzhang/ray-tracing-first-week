use crate::color::Color;
use crate::material::{Material, ScatterInfo};
use crate::ray::Ray;
use crate::hittable::HitInfo;
use crate::vector::*;
use crate::utils::random_f64;

pub struct Dielectric {
    pub ir: f64 //材质的折射率
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric {
            ir: index_of_refraction
        }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        //此函数用到较多的光学方面的知识，有兴趣的话可以查一下相关资料
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        let t = (1.0 - cosine) * (1.0 - cosine);
        return r0 + (1.0 - r0) * (t * t * (1.0 - cosine));
        //return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo> {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        //入射光线所在介质的折射率与光线即将进入（如果发生折射）的介质折射率的比值
        let refraction_ratio = if hit_info.front_face { 1.0/self.ir } else { self.ir };

        let unit_direction = r_in.dir.unit();
        //计算入射角的正余弦值
        let cos_theta = Vector3::fmin(hit_info.normal.dot(&-unit_direction), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0; //发生全反射
        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_f64() {
            //计算反射光线的方向
            Vector3::reflect(&unit_direction, &hit_info.normal)
        } else {
            //计算折射光线的方向
            Vector3::refract(&unit_direction, &hit_info.normal, refraction_ratio)
        };
                
        let scattered_ray = Ray::new(hit_info.pos, direction);

        Some(ScatterInfo::new(attenuation, scattered_ray))
    }
}