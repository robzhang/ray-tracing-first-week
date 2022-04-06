extern crate rand ;
use rand::Rng;

const PI:f64 = 3.1415926535897932385;

#[inline]
pub fn degrees_to_radians(degrees: f64) ->f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_f64_range(min: f64, max: f64) ->f64 {
    let mut rng = rand::thread_rng(); 

    rng.gen_range(min..=max)
}

#[inline]
pub fn random_f64() ->f64 {
    let mut rng = rand::thread_rng(); 

    rng.gen()
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}