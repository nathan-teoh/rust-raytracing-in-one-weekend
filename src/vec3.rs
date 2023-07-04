use std::ops::{Add, Mul, Div, AddAssign, MulAssign, DivAssign, Neg, Sub};

use rand::Rng;

#[derive(Copy,Clone,Debug)]
pub struct Vec3{
    pub e : [f32;3],
}

impl Vec3{
    pub fn length_squared(&self) -> f32{
        self.e[0].mul(self.e[0]).add(self.e[1].mul(self.e[1])).add(self.e[2].mul(self.e[2]))
    }
    
    pub fn length(&self) -> f32{
        self.length_squared().sqrt()
    }

    pub fn x(&self)->f32{
        self.e[0]
    }

    pub fn y(&self)->f32{
        self.e[1]
    }

    pub fn z(&self)->f32{
        self.e[2]
    }
    
    // pub fn new(e0: f32, e1: f32, e2: f32)->Self{
    //     Vec3 { e: [e0,e1,e2] }
    // }

    pub fn unit_vector(self)-> Vec3{
        self.div(self.length())
    }

    pub fn dot(self,rhs: Self)-> f32{
        self.e[0].mul(rhs.e[0]).add(self.e[1].mul(rhs.e[1])).add(self.e[2].mul(rhs.e[2]))
    }

    pub fn random() -> Vec3{
        let mut rng = rand::thread_rng();
        Vec3 { e: [rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>()] }
    }

    pub fn random_unit_vector() -> Vec3{
        Self::unit_vector(Self::random_in_unit_sphere())
    }

    pub fn randommm(min: f32, max: f32)-> Vec3{
        let mut rng = rand::thread_rng();
        Vec3 { e: [rng.gen_range(min..max),rng.gen_range(min..max),rng.gen_range(min..max)] }
    }

    pub fn random_in_unit_sphere() -> Vec3{
        loop{
            let p = Vec3::randommm(-1.0,1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }else{
                return p
            }

        }
    }
    // pub fn cross(self,rhs: Self)-> Vec3{
    //     Vec3 { 
    //         e: [
    //             self.e[1].mul(rhs.e[2]).sub(self.e[2].mul(rhs.e[1])),
    //             self.e[2].mul(rhs.e[0]).sub(self.e[0].mul(rhs.e[2])),
    //             self.e[0].mul(rhs.e[1]).sub(self.e[1].mul(rhs.e[0]))   
    //             ] 
    //     }
    // }
    
}
impl Default for Vec3{
    fn default() -> Self {
        Vec3 { e: [0.0,0.0,0.0] }
    }
}
impl Add for Vec3{
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3{
            e: [
                self.e[0].add(rhs.e[0]),
                self.e[1].add(rhs.e[1]),
                self.e[2].add(rhs.e[2]),
            ]
        }
    }
}

impl AddAssign for Vec3{
    fn add_assign(&mut self, rhs: Self) {
        self.e[0].add_assign(rhs.e[0]);
        self.e[1].add_assign(rhs.e[1]);
        self.e[2].add_assign(rhs.e[2]);
    }
}

impl Mul<f32> for Vec3{
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3{
            e:[
                self.e[0].mul(rhs),
                self.e[1].mul(rhs),
                self.e[2].mul(rhs),
            ]
        }
    }
}

impl Mul for Vec3{
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3{
            e:[
                self.e[0].mul(rhs.e[0]),
                self.e[1].mul(rhs.e[1]),
                self.e[2].mul(rhs.e[2]),
            ]
        }
    }
}

impl MulAssign<f32> for Vec3{
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0].mul_assign(rhs);
        self.e[1].mul_assign(rhs);
        self.e[2].mul_assign(rhs);
    }
}

impl Sub for Vec3{
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3{
            e:[
                self.e[0].sub(rhs.e[0]),
                self.e[1].sub(rhs.e[1]),
                self.e[2].sub(rhs.e[2]),
            ]
        }
    }
}

impl Neg for Vec3{
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3{
            e:[
                self.e[0].neg(),
                self.e[1].neg(),
                self.e[2].neg()
            ]
        }
    }
}

impl Div<f32> for Vec3{
    type Output = Vec3;

    fn div(self, rhs: f32)-> Self::Output{
        Vec3{
            e:[
                self.e[0].div(rhs),
                self.e[1].div(rhs),
                self.e[2].div(rhs)
            ]
        }
    }
}

impl DivAssign<f32> for Vec3{
    fn div_assign(&mut self, rhs: f32) {
        self.mul_assign((1.0).div(rhs))
    }
}

impl PartialEq for Vec3{
    fn eq(&self, other: &Self) -> bool {
        (self.e[0].eq(&other.e[0])) && ((self.e[1].eq(&other.e[1])) ) && ((self.e[2].eq(&other.e[2])) )
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    const EMPTY: Vec3 = Vec3{e:[0.0,0.0,0.0]};
    const ONES: Vec3 = Vec3{e:[1.0,1.0,1.0]};
    const HALVES: Vec3 = Vec3{e:[0.5,0.5,0.5]};
    #[test]
    fn test_add(){
        let ans = Vec3{e:[1.0,1.0,1.0]};
        assert_eq!(EMPTY.add(ans),ans);
        assert_eq!(HALVES.add(HALVES),ONES);
    }
    #[test]
    fn test_add_assign(){
        let mut aaa = ONES;
        aaa+=aaa;
        let ans= Vec3{e:[2.0,2.0,2.0]};
        assert_eq!(aaa,ans);
    }

    #[test]
    fn test_mul_f32(){
        let ans = Vec3{e:[6.0,6.0,6.0]};
        assert_eq!(ONES.mul(6.0),ans);
    }

    #[test]
    fn test_mul(){
        assert_eq!(ONES.mul(HALVES),HALVES);
    }

    #[test]
    fn test_mul_assign(){
        let ans = Vec3{e:[0.25,0.25,0.25]};
        let mut aaa = HALVES;
        aaa*=0.5;
        assert_eq!(aaa,ans);
    }


    #[test]
    fn test_sub(){
        assert_eq!(ONES-HALVES,HALVES);
    }

    #[test]
    fn test_div(){
        assert_eq!(ONES/0.5,ONES*2.0);
    }

    #[test]
    fn test_neg(){
        let ans: Vec3 = Vec3{e:[-1.0,-1.0,-1.0]};

        assert_eq!(-ONES, ans);
    }

    #[test]
    fn test_div_assign(){
        let mut aaa = Vec3{e:[1.0,1.0,1.0]};
        aaa.div_assign(0.5);
        assert_eq!(aaa,ONES*2.0);
    }
    #[test]
    fn test_partial_eq(){
        assert_eq!(HALVES==HALVES, true);
        assert_eq!(HALVES!=ONES, true);
    }





}