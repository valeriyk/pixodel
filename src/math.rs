pub mod math {
	#[derive(Copy, Clone)]
	pub struct Vec3f {
		x: f32,
		y: f32,
		z: f32,
	}
	
	impl Vec3f {
		pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
			Vec3f {x, y, z}
		}
		
		pub fn normalize(&self) -> Vec3f {
			let length: f32 = (*self * *self).sqrt();
			let length_inverted = 1.0 / length;
			Vec3f::new(self.x * length_inverted, self.y * length_inverted, self.z * length_inverted)
		}
	}
	
	impl core::ops::Add for Vec3f {
		type Output = Self;
		
		fn add(self, other: Self) -> Self::Output {
			Self {
				x: self.x + other.x,
				y: self.y + other.y,
				z: self.z + other.z,
			}
		}
	}
	
	impl core::ops::Sub for Vec3f {
		type Output = Self;
		
		fn sub(self, other: Self) -> Self::Output {
			Self {
				x: self.x - other.x,
				y: self.y - other.y,
				z: self.z - other.z,
			}
		}
	}
	
	impl core::ops::Mul<Vec3f> for Vec3f {
		type Output = f32;
		
		fn mul(self, other: Vec3f) -> Self::Output {
			self.x * other.x + self.y * other.y + self.z * other.z
		}
	}
	
	impl core::ops::Mul<f32> for Vec3f {
		type Output = Vec3f;
		
		fn mul(self, other: f32) -> Self::Output {
			Vec3f { x: self.x * other, y: self.y * other, z: self.z * other }
		}
	}
}