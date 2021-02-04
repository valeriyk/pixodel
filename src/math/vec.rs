#[derive(Copy, Clone)]
pub struct Vec3f {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Copy, Clone)]
pub struct Vec4f {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn new_zeroed(&self) -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn normalize(&self) -> Self {
        let length: f32 = (*self * *self).sqrt();
        let length_inverted = 1.0 / length;
        Self {
            x: self.x * length_inverted,
            y: self.y * length_inverted,
            z: self.z * length_inverted,
        }
    }

    /// Cross product
    pub fn crossprod(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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

/// Dot product
impl core::ops::Mul<Vec3f> for Vec3f {
    type Output = f32;

    fn mul(self, other: Self) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

// /// Cross product
// impl core::ops::Mul<Vec3f> for Vec3f {
// 	type Output = Vec3f;
//
// 	fn mul(self, other: Self) -> Self::Output {
// 		Vec3f::new{
// 			x: self.y * other.z - self.z * other.y,
// 			y: self.z * other.x - self.x * other.z,
// 			z: self.x * other.y - self.y * other.x,
// 		}
// 	}
// }

impl core::ops::Mul<f32> for Vec3f {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl core::ops::Neg for Vec3f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/*impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {x, y, z, w}
    }

    // pub fn normalize(&self) -> Self {
    // 	let length: f32 = (*self * *self).sqrt();
    // 	let length_inverted = 1.0 / length;
    // 	Self::new(self.x * length_inverted, self.y * length_inverted, self.z * length_inverted, self.w * length_inverted)
    // }
}

impl From<Vec3f> for Vec4f {
    //type Output = Self;

    fn from(input: Vec3f) -> Self {
        Self::new(input.x, input.y, input.z, 1.0)
    }
}*/
