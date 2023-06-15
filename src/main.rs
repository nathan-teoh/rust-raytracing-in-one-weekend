use std::ops::{Sub, Div, Add, Mul};

use crate::{vec3::Vec3, color::WriteColor, ray::{Ray}};


mod vec3;
mod color;
mod ray;

//image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
//const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32).div(ASPECT_RATIO);
//const MULT: f64 = 255.999;

//camera

fn main() {
    //image
    let image_height: f32  = (IMAGE_WIDTH as f32).div(ASPECT_RATIO);
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO.mul(viewport_height);
    let focal_length: f32 = 1.0;

    let origin = Vec3::default();
    let horizontal = Vec3{e:[viewport_width,0.0,0.0]};
    let vertical = Vec3{e:[0.0,viewport_height,0.0]};
    let focal = Vec3{e:[0.0,0.0,focal_length]};
    let lower_left_corner = origin.sub(horizontal.div(2.0)).sub(vertical.div(2.0)).sub(focal);

    println!("P3\n{IMAGE_WIDTH} {image_height}\n255");

    //let mut i = 0;
    //let mut j = (IMAGE_HEIGHT as i32).sub(1);

    for mut j in (0..(image_height as i32).sub(1)).rev(){
        j-=1;
        for i in 0..IMAGE_WIDTH{
            //let _ = i+1;
            if i >= IMAGE_WIDTH{
                break;
            }
            let u = (i as f32).div(IMAGE_WIDTH.sub(1) as f32);
            let v = (j as f32).div((image_height as i32).sub(1) as f32);
            let d = lower_left_corner.add(horizontal.mul(u)).add(vertical.mul(v)).sub(origin);
            let ray = Ray{orig:origin,dir:d};
            let color = ray.ray_color();
            Vec3::write_color(color);
        }
    }
}


