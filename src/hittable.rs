use crate::vector::{Vector3, Point3};
use crate::ray::Ray;
use crate::material::Material;
pub struct HitInfo<'a> {
    pub pos: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool, //光线是否来自于正面，对于球体来说，正面指的球体的外面

    pub material: &'a dyn Material
}

impl<'a> HitInfo<'a> {
    pub fn new(r: &Ray, pos: Point3, outward_normal: &Vector3, t: f64, material: &'a dyn Material) ->Self {
        let front_face = !HitInfo::with_same_direction(&r.dir, outward_normal);
        let normal = HitInfo::correct_normal(front_face, outward_normal);

        HitInfo {
            pos,
            normal,
            t,
            front_face,

            material,
        }
    }
/*
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        //根据向量点积的定义，如果两个向量之间的夹角小于90度（大体方向一致），则它们的点积应该大于0
        //r.dir是光线的方向，outward_normal始终从球心指向球的外面
        self.front_face = !HitInfo::with_same_direction(&r.dir, outward_normal);//r.dir.dot(outward_normal) < 0.0;
        self.normal = HitInfo::correct_normal(self.front_face, outward_normal);
    }
*/
    #[inline]
    fn with_same_direction(v1: &Vector3, v2: &Vector3) -> bool {
        v1.dot(v2) >= 0.0
    }

    #[inline]
    fn correct_normal(front_face: bool, outward_normal: &Vector3) ->Vector3 {
        if front_face {*outward_normal} else {-*outward_normal}
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo>;
}
