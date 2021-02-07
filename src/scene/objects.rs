use crate::geometry::triangle::Triangle;
use crate::geometry::{Mat4f, Point3d, Vector3d};

pub(crate) mod wfobj;

pub trait TraceablePrimitive {
	fn get_distance_to(&self, ray_origin: Point3d, ray_dir: Vector3d) -> Option<f32>;
	fn get_normal(&self, surface_pt: Point3d) -> Vector3d;
}

pub trait TraceableObject {
	fn triangulate(&self) -> Vec<Triangle>;
	fn set_model_mtx(&self) -> Mat4f;
	//fn get_model_mtx(&self) -> &Mat4f;
	fn rotate(&mut self, x: f32, y: f32, z: f32);
	fn scale(&mut self, x: f32, y: f32, z: f32);
	fn translate(&mut self, x: f32, y: f32, z: f32);
}
