use std::ops::{Index, Mul, Add};
use std::iter::FromIterator;

#[derive(Copy, Clone)]
pub struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Copy, Clone)]
pub struct Point4d {
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
}

impl Point3d {
    pub fn new() -> Point3d {
        Point3d {x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn from_array(a: &[f32; 3]) -> Point3d {
        Point3d {x: a[0], y: a[1], z: a[2]}
    }
    pub fn from_coords(x: f32, y: f32, z: f32) -> Point3d {
        Point3d {x, y, z}
    }
}

impl core::convert::From<Point4d> for Point3d {
    fn from(other: Point4d) -> Point3d {
        let w_inv = 1.0 / other.w;
        Self {
            x: other.x * w_inv,
            y: other.y * w_inv,
            z: other.z * w_inv,
        }
    }
}

impl<'a> core::ops::Add<&'a Point3d> for &'a Point3d {
    type Output = Vector3d;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
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
        }
    }
}

impl core::ops::Add<Vector3d> for Point3d {
    type Output = Self;

    fn add(self, other: Vector3d) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl core::ops::Mul<f32> for Point3d {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
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
            _ => panic!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Point3d {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(),
        }
    }
}

/*pub struct IterPoint3d {
    pt: Point3d,
    item_idx: usize,
}

impl Iterator for IterPoint3d {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.item_idx {
            0 => self.pt.x,
            1 => self.pt.y,
            2 => self.pt.z,
            _ => return None,
        };
        self.item_idx += 1;
        Some(result)
    }
}
impl IntoIterator for Point3d {
    type Item = f32;
    type IntoIter = IterPoint3d;
    fn into_iter(self) -> Self::IntoIter {
        IterPoint3d {
            pt: self,
            item_idx: 0,
        }
    }
}
*/




impl Point4d {
    pub fn new() -> Point4d {
        Point4d {x: 0.0, y: 0.0, z: 0.0, w: 0.0}
    }
    pub fn from_array(a: &[f32; 4]) -> Point4d {
        Point4d {x: a[0], y: a[1], z: a[2], w: a[3]}
    }
    pub fn from_coords(x: f32, y: f32, z: f32, w: f32) -> Point4d {
        Point4d {x, y, z, w}
    }
}

impl core::convert::From<Point3d> for Point4d {
    fn from(other: Point3d) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z,
            w: 1.0,
        }
    }
}

impl std::ops::Index<usize> for Point4d {
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

impl std::ops::IndexMut<usize> for Point4d {
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
    pub fn new() -> Vector3d {
        Vector3d {x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn from_array(a: &[f32; 3]) -> Vector3d {
        Vector3d {x: a[0], y: a[1], z: a[2]}
    }
    pub fn from_coords(x: f32, y: f32, z: f32) -> Vector3d {
        Vector3d {x, y, z}
    }
    
    pub fn normalize(&self) -> Self {
        let length = (*self * *self).sqrt();
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

impl core::ops::Add<Vector3d> for Vector3d {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
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
        }
    }
}

/// Dot product
impl core::ops::Mul<Vector3d> for Vector3d {
    type Output = f32;

    fn mul(self, other: Self) -> Self::Output {
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
        }
    }
}

/*pub struct IterVector3d {
    vec: Vector3d,
    item_idx: usize,
}

impl Iterator for IterVector3d {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.item_idx {
            0 => self.vec.x,
            1 => self.vec.y,
            2 => self.vec.z,
            3 => self.vec.w,
            _ => return None,
        };
        self.item_idx += 1;
        Some(result)
    }
}
impl IntoIterator for Vector3d {
    type Item = f32;
    type IntoIter = IterVector3d;
    fn into_iter(self) -> Self::IntoIter {
        IterVector3d {
            vec: self,
            item_idx: 0,
        }
    }
}
*/



/*/// Point + Vector = Point; Vector + Vector = Vector,
fn add_vector<T>(this: T, other: &[f32]) -> T
    where
        T: IntoIterator + FromIterator<<<T as IntoIterator>::Item as Add<f32>>::Output>,
        T::Item: Add<f32>
{
    this.into_iter().zip(other).map(|x| x.0 + *x.1).collect()
}

/// Point + Point = Vector; Vector + Point = Vector,
fn add_point<T>(this: T, other: &[f32]) -> [f32; 4]
    where
        T: IntoIterator + FromIterator<<<T as IntoIterator>::Item as Add<f32>>::Output>,
        T::Item: Add<f32>
{
    this.into_iter().zip(other).map(|x| x.0 + *x.1).collect()
}

fn add_any<T>(this: &[f32], other: &[f32]) -> T
    where
         T: FromIterator<f32>
    //     T::Item: Add<f32>
{
    this.into_iter().zip(other).take(3).map(|x| x.0 + *x.1).collect()
}

impl core::ops::Add<Point3d_2> for Point3d_2 {
    type Output = Vector3d_2;
    
    fn add(self, other: Self) -> Self::Output {
        add_any::<Self::Output>(&self.v, &other.v)
    }
}

impl core::ops::Add<Vector3d_2> for Point3d_2 {
    type Output = Point3d_2;
    
    fn add(self, other: Vector3d_2) -> Self::Output {
        add_any::<Self::Output>(&self.v, &other.v)
    }
}*/