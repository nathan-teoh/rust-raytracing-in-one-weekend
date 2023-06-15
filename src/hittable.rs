use crate::{ray::Ray, vec3::Vec3};

pub trait Hittable{
    fn hit(self,r: &Ray, t_min: f32, t_max: f32, rec: HitRecord)-> bool;
    // fn set_face_normal(&self, r: &Ray, outward_normal: Vec3)->bool{
    //     //front_face = r.direction().dot(outward_normal) < 0.0;
        
    // }
}

pub struct HitRecord{
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

#[derive(Default)]
pub struct HittableList{
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList{
    pub fn push(&mut self, hittable: impl Hittable + 'static){
        self.list.push(Box::new(hittable))
    }
}

impl HitRecord{
    pub fn set_face_normal(mut self: Self, r: &Ray, outward_normal: Vec3){
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face == true {outward_normal} else {-outward_normal}
        
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
}

