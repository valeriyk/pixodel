use crate::geometry::{Point3d, Vector3d, min_of_two_f32, max_of_two_f32};
use std::mem;
use crate::geometry::TraceablePrimitive;

#[derive(Copy, Clone)]

pub struct Aabb {
	min: Point3d,
	max: Point3d,
}

impl Aabb {
	pub fn new () -> Aabb {
		Aabb {
			min: Point3d::from_coords(f32::MAX, f32::MAX, f32::MAX),
			max: Point3d::from_coords(f32::MIN, f32::MIN, f32::MIN),
		}
	}
	pub fn from_point3d(min: Point3d, max: Point3d) -> Aabb {
		Aabb {
			min,
			max,
		}
	}
	
	pub fn _get_min(&self) -> Point3d {
		self.min
	}
	pub fn _get_max(&self) -> Point3d {
		self.max
	}
	
	pub fn get_superset(&self, other: Self) -> Self {
		Aabb::from_point3d(
			Point3d::from_coords(
				min_of_two_f32(self.min.x, other.min.x),
				min_of_two_f32(self.min.y, other.min.y),
				min_of_two_f32(self.min.z, other.min.z),
			),
			Point3d::from_coords(
				max_of_two_f32(self.max.x, other.max.x),
				max_of_two_f32(self.max.y, other.max.y),
				max_of_two_f32(self.max.z, other.max.z),
			),
		)
	}
	
	pub fn get_centroid(&self) -> Point3d {
		Point3d::from_coords(
			(self.min.x + self.max.x) * 0.5,
			(self.min.y + self.max.y) * 0.5,
			 (self.min.z + self.max.z) * 0.5,
		)
	}
}

// TODO: Use the implementation from "An Efficient and Robust Ray-Box Intersection Algorithm" by Williams et al
impl TraceablePrimitive for Aabb {
	fn get_distance_to(&self, ray_origin: &Point3d, ray_dir: &Vector3d) -> Option<f32> {
		
		let mut tmin = (self.min.x - ray_origin.x) / ray_dir.x;
		let mut tmax = (self.max.x - ray_origin.x) / ray_dir.x;
		if tmin > tmax {
			mem::swap(&mut tmin, &mut tmax);
		}
		
		let mut tymin = (self.min.y - ray_origin.y) / ray_dir.y;
		let mut tymax = (self.max.y - ray_origin.y) / ray_dir.y;
		if tymin > tymax {
			mem::swap(&mut tymin, &mut tymax);
		}
		
		if tmin > tymax || tymin > tmax {
			return None;
		}
		
		if tymin > tmin {
			tmin = tymin;
		}
		if tymax < tmax {
			tmax = tymax;
		}
		
		let mut tzmin = (self.min.z - ray_origin.z) / ray_dir.z;
		let mut tzmax = (self.max.z - ray_origin.z) / ray_dir.z;
		if tzmin > tzmax {
			mem::swap(&mut tzmin, &mut tzmax);
		}
		
		if tmin > tzmax || tzmin > tmax {
			return None;
		}
		
		if tzmin > tmin {
			tmin = tzmin;
		}
		if tzmax < tmax {
			tmax = tzmax;
		}
		
		if tmin >= 0.0 {
			Some(tmin)
		} else {
			Some(tmax)
		}
	}
	
	//fn intersect (&self, ray r)
	
	fn get_normal(&self, _: &Point3d) -> Vector3d {
		Vector3d::new()
	}
	
	fn get_bounding_box(&self) -> Aabb {
		*self
	}
}
