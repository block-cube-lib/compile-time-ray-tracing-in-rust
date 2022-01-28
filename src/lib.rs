#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]
pub mod util;
pub mod vec3;

pub use vec3::*;

pub type Point3 = Vec3;
pub type Color = Vec3;