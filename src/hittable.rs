use crate::Sphere;

#[derive(Clone, Copy)]
pub enum Hittable {
    Sphere(Sphere),
}
