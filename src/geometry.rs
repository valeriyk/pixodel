
mod matrix;
mod vector;
pub mod matrix_transform;
mod point;
pub mod triangle;
// pub mod axis_aligned_box;
// pub mod plane;
pub mod sphere;
pub(crate) mod aabb;


pub use matrix::Mat4f;
pub use point::{Point3d, Point4d};
pub use vector::Vector3d;

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
