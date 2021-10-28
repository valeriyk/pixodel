use wavefront_obj::obj::{self, ObjSet};
use wavefront_obj::ParseError;
use std::sync::Arc;
use std::fs::File;
use std::io::Read;

use crate::scene::IntoTriangles;
use crate::geometry::triangle::Triangle;
use crate::geometry::{Mat4f, Point3d, Point4d};
//use crate::geometry::matrix_transform::*;

pub struct WfObj {
	model: Arc<ObjSet>,
}

impl WfObj {
	pub fn new(model: Arc<ObjSet>) -> Self {
		WfObj {
			model,
		}
	}
	fn iter(&self) -> IterWfObj {
		IterWfObj {
			wfobj: &self,
			oidx: 0,
			gidx: 0,
			sidx: 0,
		}
	}
}

impl IntoTriangles for WfObj {
	fn triangulate(&self) -> Vec<Triangle> {
		self.iter().collect()
	}
}

pub struct IterWfObj<'a> {
	wfobj: &'a WfObj,
	oidx: usize,
	gidx: usize,
	sidx: usize,
}

impl<'a> Iterator for IterWfObj<'a> {
	type Item = Triangle;
	fn next(&mut self) -> Option<Self::Item> {
		let object = self.wfobj.model.objects.get(self.oidx)?;
		let geometry = object.geometry.get(self.gidx)?;
		let shape = geometry.shapes.get(self.sidx)?;
		
		let coord_idx = match shape.primitive {
			obj::Primitive::Triangle(
					(coord_a, _a1, _a2),
					(coord_b, _b1, _b2),
					(coord_c, _c1, _c2)
				) => {
				//println!("IterObjSet {}:{}:{}", coord_a, coord_b, coord_c);
				Some((coord_a, coord_b, coord_c))
			}
			_ => {
				//println!("Unsupported primitive!");
				None
			}
		}?;
		
		let a = object.vertices[coord_idx.0];
		let b = object.vertices[coord_idx.1];
		let c = object.vertices[coord_idx.2];
		
		self.sidx += 1;
		if self.sidx >= geometry.shapes.len() {
			self.sidx = 0;
			self.gidx += 1;
		}
		if self.gidx >= object.geometry.len() {
			self.gidx = 0;
			self.oidx += 1;
		}
		
		Some(Triangle::new(
			Point3d::from_coords(a.x as f32, a.y as f32, a.z as f32),
			Point3d::from_coords(b.x as f32, b.y as f32, b.z as f32),
			Point3d::from_coords(c.x as f32, c.y as f32, c.z as f32),
		))
	}
}


pub fn new_wavefront_obj(path: &str) -> Result<ObjSet, ParseError> {
	let file_content = {
		let mut f = File::open(path).unwrap();
		let mut content = String::new();
		f.read_to_string(&mut content);
		content
	};
	obj::parse(file_content)
}
