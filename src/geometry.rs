pub use matrix::Mat4f;
pub use point::{Point3d, Point4d};
pub use vector::Vector3d;

use crate::geometry::aabb::Aabb;
use crate::geometry::triangle::Triangle;
use crate::geometry::sphere::Sphere;

pub(crate) mod matrix;
mod vector;
mod point;
pub mod triangle;
// pub mod axis_aligned_box;
// pub mod plane;
pub mod sphere;
pub(crate) mod aabb;
mod octree;


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
	fn get_centroid(&self) -> Point3d;
	fn model_to_world(&self, model: &Mat4f) -> Self;
}

pub enum PrimitiveType {
	Triangle(Triangle),
	Sphere(Sphere)
}

impl TraceablePrimitive for PrimitiveType {
	fn get_distance_to(&self, ray_origin: &Point3d, ray_dir: &Vector3d) -> Option<f32> {
		match self {
			PrimitiveType::Triangle(t) => t.get_distance_to(ray_origin, ray_dir),
			PrimitiveType::Sphere(s) => s.get_distance_to(ray_origin, ray_dir),
		}
	}
	
	fn get_normal(&self, surface_pt: &Point3d) -> Vector3d {
		match self {
			PrimitiveType::Triangle(t) => t.get_normal(surface_pt),
			PrimitiveType::Sphere(s) => s.get_normal(surface_pt),
		}
	}
	
	fn get_bounding_box(&self) -> Aabb {
		match self {
			PrimitiveType::Triangle(t) => t.get_bounding_box(),
			PrimitiveType::Sphere(s) => s.get_bounding_box(),
		}
	}
	
	fn get_centroid(&self) -> Point3d {
		match self {
			PrimitiveType::Triangle(t) => t.get_centroid(),
			PrimitiveType::Sphere(s) => s.get_centroid(),
		}
	}
	
	fn model_to_world(&self, model: &Mat4f) -> Self {
		match self {
			PrimitiveType::Triangle(t) => PrimitiveType::Triangle(t.model_to_world(model)),
			PrimitiveType::Sphere(s) => PrimitiveType::Sphere(s.model_to_world(model)),
		}
	}
}