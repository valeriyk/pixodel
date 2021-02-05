//pub mod light {
use crate::math::Point3d;

pub struct Light {
    pub position: Point3d,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Point3d, intensity: f32) -> Light {
        Light {
            position,
            intensity,
        }
    }
}
//}
