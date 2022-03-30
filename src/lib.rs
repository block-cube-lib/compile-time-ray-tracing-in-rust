#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]
#![feature(const_eval_limit)]
#![const_eval_limit = "0"]
pub mod hit_record;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod util;
pub mod vec3;

pub use hit_record::*;
pub use hittable::*;
pub use hittable_list::*;
pub use ray::*;
pub use sphere::*;
pub use util::*;
pub use vec3::*;

pub type Point3 = Vec3;
pub type Color = Vec3;
