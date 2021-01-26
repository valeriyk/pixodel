pub mod triangle {
	use crate::math::math::Vec3f;
	use crate::Traceable;
	
	#[derive(Copy, Clone)]
	pub struct Triangle {
		v: [Vec3f; 3],
		normal: Vec3f,
	}
	
	impl Triangle {
		pub fn new(a: Vec3f, b: Vec3f, c: Vec3f) -> Self {
			Triangle { v: [a, b, c], normal: a.crossprod(&b).normalize() }
		}
	}
	
	impl Traceable for Triangle {
		fn is_intersected_by(&self, ray_origin: Vec3f, ray_dir: Vec3f) -> (bool, f32) {
			(false, 0.0)
		}
		fn get_normal(&self, surface_pt: Vec3f) -> Vec3f {
			// Vec3f::new(0.0, 0.0, 0.0)
			self.normal
		}
		
		// fn get_type(&self) -> TraceableObjType {
		//     TraceableObjType::Triangle
		// }
		//
		// fn get_data(self) -> TraceableObj {
		//     TraceableObj {triangle: self}
		// }
	}
}