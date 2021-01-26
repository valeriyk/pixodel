pub mod triangle {
	use crate::math::math::Vec3f;
	use crate::Traceable;
	
	#[derive(Copy, Clone)]
	pub struct Triangle {
		v: [Vec3f; 3],
		normal: Vec3f,
	}
	
	fn moller_trumbore(triangle: &Triangle, ray_origin: Vec3f, ray_dir: Vec3f) -> (bool, f32) {
		const EPSILON: f32 = 0.001;
		let v0v1 = triangle.v[1] - triangle.v[0];
		let v0v2 = triangle.v[2] - triangle.v[0];
		let pvec = ray_dir.crossprod(&v0v2);
		let det = v0v1 * pvec;
		
		if det < EPSILON {
			return (false, f32::MAX)
		}
		
		let inv_det = 1.0 / det;
		let tvec = ray_origin - triangle.v[0];
		let u = tvec * pvec * inv_det;
		
		if u < 0.0 || u > 1.0 {
			return (false, f32::MAX)
		}
		
		let qvec = tvec.crossprod(&v0v1);
		let v = ray_dir * qvec * inv_det;
		if v < 0.0 || u + v > 1.0 {
			return (false, f32::MAX)
		}
		
		let t = v0v2 * qvec * inv_det;
		(true, t)
	}
	
	impl Triangle {
		pub fn new(v0: Vec3f, v1: Vec3f, v2: Vec3f) -> Self {
			let v0v1 = v1 - v0;
			let v0v2 = v2 - v0;
			let normal = v0v1.crossprod(&v0v2).normalize();
			Triangle {
				v: [v0, v1, v2],
				normal,
			}
		}
	}
	
	impl Traceable for Triangle {
		fn is_intersected_by(&self, ray_origin: Vec3f, ray_dir: Vec3f) -> (bool, f32) {
			moller_trumbore(self, ray_origin, ray_dir)
		}
		fn get_normal(&self, surface_pt: Vec3f) -> Vec3f {
			// Vec3f::new(0.0, 0.0, 0.0)
			self.normal
		}
	}
}