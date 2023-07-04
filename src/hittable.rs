use std::{sync::Arc, rc::Rc, borrow::BorrowMut};

use crate::{ray::Ray, vec3::Vec3};

pub trait Hittable{
    fn hit(&self,r: Ray, t_min: f32, t_max: f32)-> Option<HitRecord>;
}

#[derive(Clone, Copy)]
pub struct HitRecord{
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord{
    pub fn set_face_normal(mut self: Self, r: &Ray, outward_normal: Vec3){
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face == true {outward_normal} else {-outward_normal}
        
    }
}

impl Default for HitRecord{
    fn default() -> Self {
        Self { p: Vec3 { e: [0.0,0.0,0.0] }, normal: Vec3 { e: [0.0,0.0,0.0] }, t: 1.0, front_face: false }
    }
}

#[derive(Default)]
pub struct HittableList{
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList{
    pub fn add(&mut self, hittable: impl Hittable + 'static){
        //self.objects.push(Box::new(hittable));
        self.objects.push(Box::new(hittable))

    }
}

impl Hittable for HittableList{
    fn hit(&self,r: Ray, t_min: f32, t_max: f32)-> Option<HitRecord> {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything: Option<HitRecord>  = None;
        let mut closest_so_far: f32 = t_max;
        for i in self.objects.iter(){
            if let Some(hit) = i.hit(r, t_min, closest_so_far){
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }   
}