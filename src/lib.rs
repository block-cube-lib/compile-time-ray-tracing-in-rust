#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]
#![feature(const_eval_limit)]
#![const_eval_limit = "0"]
pub mod util;
pub mod vec3;
pub mod ray;

pub use vec3::*;
pub use ray::*;

pub type Point3 = Vec3;
pub type Color = Vec3;
