use std::{ops::{Sub, Div}};

use crate::{hittable::{Hittable, HitRecord}, vec3::Vec3, ray::At};
pub struct Sphere{
    center: Vec3,
    radius: f32,
}

impl Default for Sphere{
    fn default() -> Self {
        Self { center: Default::default(), radius: Default::default() }
    }
}


impl Hittable for Sphere{
    fn hit(self,r: &crate::ray::Ray, t_min: f32,t_max: f32, mut rec: HitRecord)-> bool {
        let oc = r.origin()-self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared().sub(self.radius*self.radius);
        let discriminant = (half_b*half_b).sub(a*c);
        if discriminant < 0.0{
            return false
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b.sub(sqrtd)).div(a*c);
        if (root < t_min) || (t_max < root){
            root = (-half_b + sqrtd).div(a);
            if(root < t_min) || t_max < root{
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true

    }
}


