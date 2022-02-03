use crate::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub const fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    pub const fn origin(&self) -> Vec3 {
        self.orig
    }

    pub const fn direction(&self) -> Vec3 {
        self.dir
    }

    pub const fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}
