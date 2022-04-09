use crate::vector::{Vector3, Point3};

//光线
pub struct Ray {
    pub orig: Point3, //光线起点
    pub dir: Vector3, //光线方向
    pub time: f64 //光线产生的时间，用于模拟物体运动造成的模糊。运动模糊是一段时间内采集到的像素平均值。
}

impl Ray {
    pub fn new(orig: Point3, dir: Vector3, time: f64) -> Self {
        Ray { orig, dir, time}
    }

    pub fn at(&self, t: f64) -> Point3 {
        let d = self.dir * t;
        self.orig + d
    }
}