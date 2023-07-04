use std::ops::{Mul, Div, Sub, Add};

use crate::{ray::Ray, vec3::Vec3};

pub struct Camera{
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub focalLength: Vec3,
    pub lower_left_corner: Vec3,
}

impl Default for Camera{
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let v_height = 2.0;
        let v_width  = aspect_ratio.mul(v_height);
        let horizontal = Vec3{e:[v_width,0.0,0.0]};
        let vertical = Vec3{e:[0.0,v_height,0.0]};
        let focal: Vec3 = Vec3{e:[0.0,0.0,1.0]};
        let lower_left_corner = Vec3::default().sub(horizontal.div(2.0)).sub(vertical.div(2.0)).sub(focal);
        Camera { origin:  Vec3::default(), horizontal: horizontal , vertical: vertical, focalLength: focal, lower_left_corner: lower_left_corner}
    }
}
impl Camera  {
    

    pub fn get_ray(&self, u: f32, v: f32)-> Ray{
        //../Ray{orig:origin,dir:d};
        return Ray { orig: self.origin, dir: self.lower_left_corner.add(self.horizontal.mul(u)).add(self.vertical.mul(v)).sub(self.origin)}
    }
}



            //
            //let d = lower_left_corner.add(horizontal.mul(u)).add(vertical.mul(v)).sub(origin);