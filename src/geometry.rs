pub use matrix::Mat4f;
pub use point::{Point3d, Point4d};
pub use vector::Vector3d;

use crate::geometry::aabb::Aabb;

pub(crate) mod matrix;
mod vector;
mod point;
pub mod triangle;
// pub mod axis_aligned_box;
// pub mod plane;
pub mod sphere;
pub(crate) mod aabb;


#[inline]
fn min_of_two_f32(a: f32, b: f32) -> f32 {
	if a < b {
		a
	} else {
		b
	}
}

#[inline]
fn max_of_two_f32(a: f32, b: f32) -> f32 {
	if a > b {
		a
	} else {
		b
	}
}

#[inline]
fn min_of_three_f32(a: f32, b: f32, c: f32) -> f32 {
	min_of_two_f32(a, min_of_two_f32(b, c))
}

#[inline]
fn max_of_three_f32(a: f32, b: f32, c: f32) -> f32 {
	max_of_two_f32(a, max_of_two_f32(b, c))
}

pub trait TraceablePrimitive {
	fn get_distance_to(&self, ray_origin: &Point3d, ray_dir: &Vector3d) -> Option<f32>;
	fn get_normal(&self, surface_pt: &Point3d) -> Vector3d;
	fn get_bounding_box(&self) -> Aabb;
}
