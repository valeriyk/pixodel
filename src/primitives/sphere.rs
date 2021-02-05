/*use crate::math::Vec3f;
use crate::primitives::Traceable;

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3f,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Traceable for Sphere {
    fn get_distance_to(&self, ray_origin: Vec3f, ray_dir: Vec3f) -> Option<f32> {
        let l = self.center - ray_origin;
        let tca = l * ray_dir;
        let d_squared = l * l - tca * tca;
        if d_squared > (self.radius * self.radius) {
            return None;
        }
        let thc = (self.radius * self.radius - d_squared).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 >= 0.0 {
            Some(t0)
        } else if t0 < 0.0 && t1 >= 0.0 {
            Some(t1)
        } else {
            None
        }
    }

    fn get_normal(&self, surface_pt: Vec3f) -> Vec3f {
        (surface_pt - self.center).normalize()
    }
}
*/