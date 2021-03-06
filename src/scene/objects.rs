use crate::geometry::triangle::Triangle;
use crate::geometry::aabb::Aabb;
use crate::geometry::{Mat4f, Point3d, Vector3d};

pub(crate) mod wfobj;
pub(crate) mod triangle;

pub trait TraceablePrimitive {
	fn get_distance_to(&self, ray_origin: &Point3d, ray_dir: &Vector3d) -> Option<f32>;
	fn get_normal(&self, surface_pt: &Point3d) -> Vector3d;
	fn get_bounding_box(&self) -> Aabb;
}

pub trait TraceableObject {
	fn triangulate(&self) -> Vec<Triangle>;// {
	// 	let model_to_world = self.set_model_mtx();
	// 	self.iter().map(|t| {
	// 		Triangle::new(
	// 			Point3d::from(&model_to_world * Point4d::from(t.v[0])),
	// 			Point3d::from(&model_to_world * Point4d::from(t.v[1])),
	// 			Point3d::from(&model_to_world * Point4d::from(t.v[2])),
	// 		)
	// 	}).collect()
	// }
	//fn iter(&self) -> Iterator<Item = Triangle>;
	fn set_model_mtx(&self) -> Mat4f;
	//fn get_model_mtx(&self) -> &Mat4f;
	fn rotate(&mut self, x: f32, y: f32, z: f32);
	fn scale(&mut self, x: f32, y: f32, z: f32);
	fn translate(&mut self, x: f32, y: f32, z: f32);
}
