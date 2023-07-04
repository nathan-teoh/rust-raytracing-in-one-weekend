use std::ops::{Mul, Div};
use rand::{self, Rng};

pub const PI: f32 = 3.1415926535897932385;
pub const INFTY: f32 = f32::INFINITY;


pub fn deg_to_rads(deg: f32)-> f32{
    deg.mul(PI).div(180.0)
}

pub fn clamp(x :f32, min: f32, max: f32)-> f32{
    if x<min {
        return min
    }else if x > max {
        return max
    }else {
        return x
    }
}