use crate::{util::*, vec3::*, HitRecord, Point3, Ray};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }

    pub const fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let mut hit_record = HitRecord {
            t,
            p,
            normal: Vec3::ZERO,
            front_face: false,
        };
        hit_record.set_face_normal(&r, outward_normal);
        Some(hit_record)
    }
}
