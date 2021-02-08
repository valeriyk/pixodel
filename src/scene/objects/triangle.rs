use crate::geometry::triangle::Triangle;
use crate::scene::objects::TraceableObject;
use crate::geometry::{Mat4f, Point3d, Point4d};
use crate::geometry::matrix_transform::{translate_xyz, rotate_about_x, rotate_about_y, rotate_about_z, scale_xyz};

pub struct TriObj {
	model: Triangle,
	scale: [f32; 3],
	rotation: [f32; 3],
	translation: [f32; 3],
	//model_to_world: Mat4f,
	// world_to_model: Mat4f,
}

impl TriObj {
	pub fn new(model: Triangle) -> Self {
		TriObj {
			model,
			scale: [0.0, 0.0, 0.0],
			rotation: [0.0, 00.0, 0.0],
			translation: [0.0, 0.0, 0.0],
			//model_to_world: Mat4f::identity(),
			// world_to_model: Mat4f::identity(),
		}
	}
	
	pub fn iter(&self) -> IterTriObj {
		IterTriObj {
			triobj: &self,
			idx: 0,
		}
	}
}

impl TraceableObject for TriObj {
	fn triangulate(&self) -> Vec<Triangle> {
		let model_to_world = self.set_model_mtx();
		self.iter().map(|t| {
			Triangle::new(
				Point3d::from(&model_to_world * Point4d::from(t.v[0])),
				Point3d::from(&model_to_world * Point4d::from(t.v[1])),
				Point3d::from(&model_to_world * Point4d::from(t.v[2])),
			)
		}).collect()
	}
	
	fn set_model_mtx(&self) -> Mat4f {
		let t = translate_xyz(&Mat4f::identity(), &self.translation);
		let rx = rotate_about_x(&t, self.rotation[0]);
		let rxy = rotate_about_y(&rx, self.rotation[1]);
		let rxyz = rotate_about_z(&rxy, self.rotation[2]);
		let model_to_world = scale_xyz(&rxyz, &self.scale);
		model_to_world
	}
	
	
	// fn get_model_mtx(&self) -> &Mat4f {
	//     &self.model_to_world
	// }
	
	fn rotate(&mut self, x: f32, y: f32, z: f32) {
		self.rotation = [x, y, z];
	}
	fn scale(&mut self, x: f32, y: f32, z: f32) {
		self.scale = [x, y, z];
	}
	fn translate(&mut self, x: f32, y: f32, z: f32) {
		self.translation = [x, y, z];
	}
}

pub struct IterTriObj<'a> {
	triobj: &'a TriObj,
	idx: usize,
}

impl<'a> Iterator for IterTriObj<'a> {
	type Item = Triangle;
	fn next(&mut self) -> Option<Self::Item> {
		if self.idx == 0 {
			self.idx += 1;
			Some(self.triobj.model)
		} else {
			None
		}
	}
}