use crate::geometry::{Point3d, Point4d, Vector3d, Mat4f};
use crate::geometry::TraceablePrimitive;
use crate::geometry::aabb::Aabb;

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Point3d,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3d, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl TraceablePrimitive for Sphere {
    fn get_distance_to(&self, ray_origin: &Point3d, ray_dir: &Vector3d) -> Option<f32> {
        let l = self.center - *ray_origin;
        let tca = l * *ray_dir;
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

    fn get_normal(&self, surface_pt: &Point3d) -> Vector3d {
        (*surface_pt - self.center).normalize()
    }
    
    fn get_bounding_box(&self) -> Aabb {
        Aabb::from_point3d(
            self.center - self.radius,
            self.center + self.radius,
        )
    }
    
    fn model_to_world(&self, model: &Mat4f) -> Self {
        Sphere::new(
            Point3d::from(model * Point4d::from(self.center)),
            self.radius, //TODO: need to scale it too!
        )
    }
}
