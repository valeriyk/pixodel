// pub mod axis_aligned_box;
// pub mod plane;
//pub mod sphere;
pub mod triangle;

use crate::math::{Point3d, Vector3d};

pub trait Traceable: Sync + Send {
    fn get_distance_to(&self, ray_origin: Point3d, ray_dir: Vector3d) -> Option<f32>;
    fn get_normal(&self, surface_pt: Point3d) -> Vector3d;
}