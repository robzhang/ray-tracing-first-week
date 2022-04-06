use crate::hittable::{HitInfo, Hittable};
use crate::ray::Ray;

pub struct World  {
    objects: Vec<Box<dyn Hittable>>
}

impl World {
    pub fn new() ->Self {
        World {
            objects: Vec::new()
        }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let mut hit_info: Option<HitInfo> = None;
        let mut closest_so_far = t_max;
    
        for object in &self.objects {
            if let Some(tmp_info) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = tmp_info.t;
                hit_info = Some(tmp_info);
            }
        }
    
        hit_info
    }
}

