use crate::ray::Ray;
use crate::color::Color;
use crate::hittable::HitInfo;
pub trait Material {//不同的材质对入射光线的处理不同（镜面反射、漫反射、折射等，颜色衰减）
    fn scatter(&self, r_in: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo>;
}

pub struct ScatterInfo {
    pub attenuation: Color,
    pub scattered: Ray,
}

impl ScatterInfo {
    pub fn new(attenuation: Color, scattered: Ray) -> ScatterInfo {
        ScatterInfo {
            attenuation,
            scattered
        }
    }
}