use std::ops::{Add, Mul, Div};

use crate::utils::{random_f64, random_f64_range};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {r,g,b}
    }


    #[inline]
    pub fn f64_r(&self) -> f64 {
        self.r
    }
    #[inline]
    pub fn f64_g(&self) -> f64 {
        self.g
    }
    #[inline]
    pub fn f64_b(&self) -> f64 {
        self.b
    }

    pub fn black() ->Self {
        Color {r: 0.0, g: 0.0, b: 0.0}
    }

    pub fn mul_color(&self, other: &Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }

    #[inline]
    pub fn random_color() -> Color {
        return Color::new(random_f64(), random_f64(), random_f64());
    }
    #[inline]
    pub fn random_color_range(min: f64, max: f64) -> Color {
        return Color::new(random_f64_range(min,max), random_f64_range(min,max), random_f64_range(min,max));
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}
impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, t: f64) ->Self {
        Self::new(self.r * t, self.g * t, self.b * t)
    }
}
impl Div<f64> for Color {
    type Output = Self;
    fn div(self, t: f64) ->Self {
        self.mul(1.0/t)
    }
}