use crate::{ray::Ray, vec3::Vec3};

pub trait Hittable{
    fn hit(self,r: &Ray, t_min: f32, t_max: f32, rec: HitRecord)-> bool;
}

pub struct HitRecord{
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
}
