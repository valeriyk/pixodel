pub mod sphere;
pub mod triangle;

use crate::math::Vec3f;

pub trait Traceable: Sync + Send {
    fn get_distance_to(&self, ray_origin: Vec3f, ray_dir: Vec3f) -> Option<f32>;
    fn get_normal(&self, surface_pt: Vec3f) -> Vec3f;
}
