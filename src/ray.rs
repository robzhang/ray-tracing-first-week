use crate::vector::{Vector3, Point3};

//光线
pub struct Ray {
    pub orig: Point3, //光线起点
    pub dir: Vector3 //光线方向
}

impl Ray {
    pub fn new(orig: Point3, dir: Vector3) -> Self {
        Ray { orig, dir}
    }

    pub fn at(&self, t: f64) -> Point3 {
        let d = self.dir * t;
        self.orig + d
    }
}