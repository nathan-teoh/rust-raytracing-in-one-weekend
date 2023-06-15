use crate::{vec3::Vec3};
use std::ops::{Add, Mul, Sub};

pub trait At {
    fn at(self: &Self,t: f32)->Vec3;
}
// pub trait ray_color{
//     fn ray_color(self: &Self)->Vec3;
// }
pub struct Ray{
    pub dir: Vec3,
    pub orig: Vec3,
}

impl At for Ray{
    fn at(self: &Ray,t: f32)->Vec3 {
        self.orig.add(self.dir.mul(t))
    }
}

impl Ray{
    pub fn origin(&self)-> Vec3{
        self.dir
    }

    pub fn direction(&self)->Vec3{
        self.dir
    }

    // pub fn new(origin: &Vec3, direction: &Vec3)->Self{
    //     Ray { dir: direction.clone(), orig: origin.clone() }
    // }

    pub fn ray_color(&self)-> Vec3{
        let t: f32 = hit_sphere(Vec3{e:[0.0,0.0,-1.0]}, 0.5, self);
        if t > 0.0 {
            let n: Vec3 = self.at(t).sub(Vec3{e:[0.0,0.0,-1.0]}).unit_vector();
            return Vec3{e:[
                n.x().add(1.0).mul(0.5),
                n.y().add(1.0).mul(0.5),
                n.z().add(1.0).mul(0.5)
            ]}
        }
        let unit_direction: Vec3 = Vec3::unit_vector(self.dir);
        let t: f32 = (0.5 as f32).mul(unit_direction.y().add(1.0 as f32));
        Vec3{e:[1.0,1.0,1.0]}.mul((1.0 as f32).sub(t)).add(Vec3{e:[0.5,0.7,1.0]}.mul(t))
    }
}
impl Default for Ray{
    fn default() -> Self {
        Self { dir: Default::default(), orig: Default::default() }
    }
}

pub fn hit_sphere(center: Vec3, radius: f32, r: &Ray)->f32{
    let oc: Vec3 = r.origin().sub(center);
    let a: f32 = r.direction().length_squared();
    let half_b: f32 = r.direction().dot(oc);
    let c: f32 = oc.length_squared().sub(radius*radius);
    let discriminant: f32 = (half_b*half_b).sub(a*c);
    if discriminant < 0.0{
        return -1.0
    }else {
        //return (-b.sub(discriminant.sqrt())).div((2.0).mul(a))
        return (-half_b.sub(discriminant.sqrt()))/a
    }
}

