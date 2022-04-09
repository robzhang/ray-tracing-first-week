use crate::color::Color;
use crate::material::{Material, ScatterInfo};
use crate::ray::Ray;
use crate::hittable::HitInfo;
use crate::vector::*;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn new(a: &Color, f: f64) ->Self {
        Metal {
            albedo: *a,
            fuzz: f
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo> {
        let reflected = Vector3::reflect(&r_in.dir.unit(), &hit_info.normal);
        
        let scattered_ray= Ray::new(hit_info.pos, reflected + Vector3::random_in_unit_sphere()*self.fuzz, r_in.time);

        if scattered_ray.dir.dot(&hit_info.normal) > 0.0 {
            Some(ScatterInfo::new(self.albedo, scattered_ray))
        } else {
            None
        }
    }
}