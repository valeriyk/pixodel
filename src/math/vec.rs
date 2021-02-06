#[derive(Copy, Clone)]
pub struct Point3d {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Copy, Clone)]
pub struct Vector3d {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Point3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn new_zeroed(&self) -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn normalize(&self) -> Self {
        let w_inverted = 1.0 / self.w;
        Self {
            x: self.x * w_inverted,
            y: self.y * w_inverted,
            z: self.z * w_inverted,
            w: 1.0,
        }
    }
}

impl core::ops::Add<Point3d> for Point3d {
    type Output = Vector3d;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: 0.0,
        }
    }
}

impl core::ops::Sub<Point3d> for Point3d {
    type Output = Vector3d;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: 0.0,
        }
    }
}

impl core::ops::Add<Vector3d> for Point3d {
    type Output = Point3d;

    fn add(self, other: Vector3d) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w,
        }
    }
}

// impl core::ops::Sub<Vector3d> for Point3d {
//     type Output = Point3d;
//
//     fn sub(self, other: Self) -> Self::Output {
//         Self::Output {
//             x: self.x - other.x,
//             y: self.y - other.y,
//             z: self.z - other.z,
//             w: self.w,
//         }
//     }
// }

// /// Dot product
// impl core::ops::Mul<Vec3f> for Point3d {
//     type Output = f32;
//
//     fn mul(self, other: Self) -> Self::Output {
//         self.x * other.x + self.y * other.y + self.z * other.z
//     }
// }

impl core::ops::Mul<f32> for Point3d {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other, // REALLY?
        }
    }
}

impl core::ops::Neg for Point3d {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::ops::Index<usize> for Point3d {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Point3d {
    //type Output = f32;

    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!(),
        }
    }
}

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn new_zeroed(&self) -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn normalize(&self) -> Self {
        let length: f32 = (*self * *self).sqrt(); // W is zero, hence doesn't contribute to length at all
        let length_inverted = 1.0 / length;
        Self {
            x: self.x * length_inverted,
            y: self.y * length_inverted,
            z: self.z * length_inverted,
            w: 0.0, // self.w is always zero, it would be zero even if I multiplied it by length_inverted
        }
    }

    /// Cross product
    pub fn crossprod(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0, // TBD: no cross product in 4D? but if w is zero, we don't care?
        }
    }
}

impl core::ops::Add<Vector3d> for Vector3d {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: 0.0, // no need to calculate, 0 + 0 is 0
        }
    }
}

impl core::ops::Sub<Vector3d> for Vector3d {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: 0.0, // no need to calculate, 0 - 0 is 0
        }
    }
}

impl core::ops::Add<Point3d> for Vector3d {
    type Output = Point3d;

    fn add(self, other: Point3d) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: other.w,
        }
    }
}

// impl core::ops::Sub<Point3d> for Vector3d {
//     type Output = Point3d;
//
//     fn sub(self, other: Self) -> Self::Output {
//         Self::Output {
//             x: self.x - other.x,
//             y: self.y - other.y,
//             z: self.z - other.z,
//             w: -other.w, // no need to calculate, 0 - 0 is 0
//         }
//     }
// }

/// Dot product
impl core::ops::Mul<Vector3d> for Vector3d {
    type Output = f32;

    fn mul(self, other: Self) -> Self::Output {
        // self.w is zero and doesn't contribute
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl core::ops::Mul<f32> for Vector3d {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: 0.0, // w was zero so remains zero
        }
    }
}

impl core::ops::Neg for Vector3d {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -0.0, // TBD: Not sure if 0.0 or -0.0 maks more sense
        }
    }
}
