use crate::color::Color;
use crate::material::{Material, ScatterInfo};
use crate::ray::Ray;
use crate::hittable::HitInfo;
use crate::vector::*;

pub struct Lambertian {
    pub albedo: Color
}

impl Lambertian {
    pub fn new(a: &Color) ->Self {
        Lambertian {
            albedo: *a
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo> {
        let mut scatter_direction = hit_info.normal + Vector3::random_unit_vector();

        if scatter_direction.near_zero(){
            scatter_direction = hit_info.normal;
        }

        let scatter_info = ScatterInfo::new(self.albedo, Ray::new(hit_info.pos, scatter_direction, _r_in.time));
        
        Some(scatter_info)
    }
}