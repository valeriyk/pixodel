use crate::math::Point3d;

pub struct Mat4f {
    pub raw: [[f32; 4]; 4],
}

impl Mat4f {
    pub fn new() -> Self {
        Mat4f { raw: [[0.0; 4]; 4] }
    }
    pub fn new_identity() -> Self {
        Mat4f {
            raw: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl<'a, 'b> core::ops::Mul<&'b Mat4f> for &'a Mat4f {
    type Output = Mat4f;

    fn mul(self, other: &'b Mat4f) -> Self::Output {
        let mut m = Mat4f::new();
        for i in 0..4 {
            for j in 0..4 {
                m.raw[i][j] = 0.0;
                for k in 0..4 {
                    m.raw[i][j] += self.raw[i][k] * other.raw[k][j];
                }
            }
        }
        m
    }
}

impl<'a> core::ops::Mul<Point3d> for &'a Mat4f {
    type Output = Point3d;

    fn mul(self, other: Point3d) -> Self::Output {
        let mut p = Point3d::new(0.0, 0.0, 0.0);
        for i in 0..4 {
            p[i] = 0.0;
            for j in 0..4 {
                p[i] += self.raw[i][j] * other[j];
            }
        }
        p
    }
}
