use std::{ops::{Sub, Div}};

use crate::{hittable::{Hittable, HitRecord}, vec3::Vec3, ray::At};
pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
}

impl Default for Sphere{
    fn default() -> Self {
        Self { center: Default::default(), radius: Default::default() }
    }
}


impl Hittable for Sphere{
    fn hit(self: &Sphere,r: crate::ray::Ray, t_min: f32,t_max: f32)-> Option<HitRecord> {
        let oc = r.origin()-self.center;
        let a = r.direction().dot(r.direction());
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared().sub(self.radius*self.radius);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let rt: f32 = (-half_b - sqrt_discriminant) / a;
            if rt < t_max && rt > t_min {
                let p = r.at(rt);
                let n = (p - self.center) / self.radius;
                return Some(HitRecord { p: p, normal: n, t: rt, front_face: false })
            }
        }
        None
    }
}


