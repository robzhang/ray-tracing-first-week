use std::rc::Rc;

use crate::Ray;
use crate::vector::Point3; 
use crate::hittable::{HitInfo, Hittable};
use crate::material::Material;
pub struct Sphere {
    pub center: Point3, // 球心坐标
    pub radius: f64, // 半径

    pub material: Rc<dyn Material>, //材质

    // 模拟球体的运动
    is_moving: bool,
    move_destination: Point3,
    move_begin_time: f64,
    move_duration: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
            is_moving: false,
            move_destination: center,
            move_begin_time: 0.0,
            move_duration: 0.0,
        }
    }

    // 模拟小球的运动
    pub fn move_to(&mut self, destination: &Point3, current_time: f64, duration: f64) {
        self.move_destination = *destination;
        self.move_begin_time = current_time;
        self.move_duration = duration;

        self.is_moving = true;
    }

    // 根据当前时间获取球体中心的位置
    fn get_current_center(&self, current_time: f64) -> Point3 {
        if !self.is_moving {
            self.center
        } else {
            self.center + (self.move_destination - self.center) * ((current_time - self.move_begin_time) / self.move_duration)
        }
    }
}

impl Hittable for Sphere {
    //判断光线r是否击中以center为球心半径为r的圆球
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let current_center = self.get_current_center(r.time);
        //光线起点到球心的向量
        let oc = r.orig - current_center;

        //球面方程：(x - center.x)^2 + (y - center.y)^2 + (z - center.z)^2 - r^2 = 0，其中x,y,z为球面上任意一点p的坐标
        //改写成向量形式为：(p - center)·(p - center) - r^2 = 0，也就是球心到球面上任意一点的向量与自身的点积等于半径的平方
        //射线r(参数方程：p(t) = r.orig + t*r.dir)与球面相交意味着射线上的某一点必须是球面上的点
        //也就是方程(p(t) - center)·(p(t) - center) - r^2 = 0必须有解，把p(t)带入得：
        //(r.orig + t*r.dir − c) · (r.orig + t*r.dir − c) − R^2 = 0，向量点积满足分配律，于是可以化简得：
        //(r.dir · r.dir)t^2 + 2r.dir · (r.orig − c)t + (r.orig − c) · (r.orig − c) − R^2 = 0.
        //这是标准的关于t的二次方程：at^2 + bt + c = 0, 有实数解必须满足b^2 - 4ac>=0
        //详细推导过程请参考《Fundamental of Computer Graphics 第四版》 4.4小节
        let a = r.dir.dot(&r.dir);
        let b = 2.0 * oc.dot(&r.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            return None
        } 

        let sqrtd = discriminant.sqrt();

        //方程的第一个解
        let mut t = (-b - sqrtd ) / (2.0*a);
        
        if t < t_min || t > t_max {
            t = (-b + sqrtd ) / (2.0*a);//方程的第二个解
            if t < t_min || t > t_max {
                return None;
            }
        }

        let pos = r.at(t);
        let info = HitInfo::new(r, pos, &((pos - current_center) / self.radius), t, &*self.material);

        Some(info)
    }
}