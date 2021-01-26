pub mod sphere {
    use crate::math::math::Vec3f;
    use crate::Traceable;
    
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
        fn is_intersected_by(&self, ray_origin: Vec3f, ray_dir: Vec3f) -> (bool, f32) {
            let l = self.center - ray_origin;
            let tca = l * ray_dir;
            let d_squared = l * l - tca * tca;
            if d_squared > (self.radius * self.radius) {
                return (false, f32::MAX)
            }
            let thc = (self.radius * self.radius - d_squared).sqrt();
            let t0 = tca - thc;
            let t1 = tca + thc;
            if t0 >= 0.0 {
                (true, t0)
            } else if t0 < 0.0 && t1 >= 0.0 {
                (true, t1)
            } else {
                (false, f32::MAX)
            }
        }
        
        fn get_normal(&self, surface_pt: Vec3f) -> Vec3f {
            (surface_pt - self.center).normalize()
        }
        
        // fn get_type(&self) -> TraceableObjType {
        //     TraceableObjType::Sphere
        // }
        //
        // fn get_data(self) -> TraceableObj {
        //     TraceableObj {sphere: self}
        // }
    }
}