use std::ops::{Sub, Div, Add, Mul};

use rand::Rng;

use crate::{vec3::Vec3, color::WriteColor, ray::{Ray}, utils::deg_to_rads, hittable::HittableList, sphere::Sphere, camera::Camera};


mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod utils;
mod camera;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 800;

fn main() {
    //image
    let image_height: f32  = (IMAGE_WIDTH as f32).div(ASPECT_RATIO);
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO.mul(viewport_height);
    let focal_length: f32 = 1.0;
    let samples = 100;
    let origin = Vec3::default();
    let horizontal = Vec3{e:[viewport_width,0.0,0.0]};
    let vertical = Vec3{e:[0.0,viewport_height,0.0]};
    let focal = Vec3{e:[0.0,0.0,focal_length]};
    let max_depth: i32 = 50;
    let lower_left_corner = origin.sub(horizontal.div(2.0)).sub(vertical.div(2.0)).sub(focal);
    
    let mut world: HittableList = HittableList::default(); 
    let cam = Camera::default();
    world.add(Sphere{center: Vec3 { e: [0.0,0.0,-1.0] }, radius:0.5});
    world.add(Sphere{center: Vec3 { e: [0.0,-100.5,-1.0] }, radius:100.0});
    println!("P3\n{IMAGE_WIDTH} {image_height}\n255");
    for j in (0..(image_height as i32)).rev(){

        for i in 0..IMAGE_WIDTH{

            if i >= IMAGE_WIDTH{
                break;
            }
            // let u = (i as f32).div(IMAGE_WIDTH.sub(1) as f32);
            // let v = (j as f32).div((image_height as i32).sub(1) as f32);
            // let d = lower_left_corner.add(horizontal.mul(u)).add(vertical.mul(v)).sub(origin);
            // let ray = Ray{orig:origin,dir:d};
            // let color = ray.ray_color(&world);
            // Vec3::write_color(color,samples);
            let mut color = Vec3::default();
            for k in (0..samples){
                let mut rng = rand::thread_rng();
                let u = (i as f32 + rng.gen::<f32>() ).div(IMAGE_WIDTH.sub(1) as f32);
                let v = (j as f32 + rng.gen::<f32>() ).div((image_height as i32).sub(1) as f32);
                let ray = cam.get_ray(u, v);
                color+= ray.ray_color(&world,max_depth);
            }
            Vec3::write_color(color, samples);
        }
    }

}


