use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::utils::random_f64_range;

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) ->f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vector3) ->f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Self {
        let u = self.y * other.z - self.z * other.y;
        let v = self.z * other.x - self.x * other.z;
        let w = self.x * other.y - self.y * other.x;
        
        Vector3::new(u, v, w)
    }
/*
    pub fn mul_vector3(&self, other: &Vector3) ->Self {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
*/

    pub fn unit(&self) -> Self {
        self.div(self.length())
    }

    #[inline]
    pub fn random(min:f64, max:f64) ->Vector3 {
        return Vector3::new(random_f64_range(min,max), random_f64_range(min,max), random_f64_range(min,max));
    }

    pub fn random_in_unit_sphere() ->Vector3 {
        loop {
            let p = Vector3::random(-1.0,1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn random_unit_vector() ->Vector3 {
        Vector3::random_in_unit_sphere().unit()
    }

    pub fn near_zero(&self) ->bool {
        let s = 1e-8;
        (Vector3::fabs(self.x) < s) && Vector3::fabs(self.y) < s && Vector3::fabs(self.z) < s
    }

    #[inline]
    fn fabs(x: f64) -> f64 {
        if x >= 0.0 {
            x
        } else {
            -x
        }
    }
    #[inline]
    pub fn fmin(x: f64, y: f64) -> f64 {
        if x <= y {
            x
        } else {
            y
        }
    }

    //镜面反射，遵循光线反射定律，入射角等于反射角
    //参数v: 入射光线方向向量，n: 法线单位向量，返回反射光线的方向向量
    #[inline]
    pub fn reflect(v: &Vector3, n: &Vector3) ->Vector3 {
        *v - *n * ((v.dot(n) * 2.0))
    }

    //折射，遵循光线折射定律。至于为什么这么求折射光线可以利用几何光学自行推导或查阅相关资料
    //参数uv: 入射光线方向向量，n: 法线单位向量，etai_over_etat：入射介质折射率与折射截止折射率的比值，返回折射光线的方向向量
    #[inline]
    pub fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f64) -> Vector3 {
        //入射角的余弦
        let cos_theta = Vector3::fmin(n.dot(&-*uv), 1.0);
        //折射光线垂直于法线的分量
        let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
        //折射光线平行于法线的分量
        let r_out_parallel = *n * (-(Vector3::fabs(1.0 - r_out_perp.length_squared())).sqrt());
        
        //合成折射向量
        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vector3 {
        loop {
            let p = Vector3::new(random_f64_range(-1.0,1.0), random_f64_range(-1.0,1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, t: f64) ->Self {
        Vector3::new(self.x * t, self.y * t, self.z * t)
    }
}
impl Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, t: f64) ->Self {
        self.mul(1.0/t)
    }
}
impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vector3::new(-self.x, -self.y , -self.z)
    }
}

pub type Point3 = Vector3;