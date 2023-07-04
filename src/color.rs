use std::ops::Mul;

use crate::{vec3::Vec3, utils::clamp};

pub trait WriteColor {
    fn write_color(vec: Vec3, samples_per_pixel: i32);
}

impl WriteColor for Vec3{
    fn write_color(vec: Vec3, samples_per_pixel: i32) {
        let mut r = vec.x();
        let mut g = vec.y();
        let mut b = vec.z();

        let scale = 1.0 / samples_per_pixel as f32;

        r = r.mul(scale as f32).sqrt();
        g = g.mul(scale as f32).sqrt();
        b = b.mul(scale as f32).sqrt();

        println!("{} {} {}", 256 as f32 * clamp(r, 0.0, 0.999), 256 as f32 * clamp(g, 0.0, 0.999), 256 as f32 * clamp(b, 0.0, 0.999))

        //divide color by number of pixels 
        //println!("{x} {y} {z}");
    }
}
