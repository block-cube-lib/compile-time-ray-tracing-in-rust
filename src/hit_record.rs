use crate::{vec3::*, Point3, Ray};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub const fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
