use std::ops::Mul;

use crate::vec3::Vec3;

pub trait WriteColor {
    fn write_color(vec: Vec3);
}

impl WriteColor for Vec3{
    fn write_color(vec: Vec3) {
        let x = vec.x().mul(255.999);
        let y = vec.y().mul(255.999);
        let z = vec.z().mul(255.999);
        println!("{} {} {}",x,y,z);
    }
}
