pub mod scene {
    use crate::{Light, Traceable};
    use std::io::Read;
    use std::fs::File;
    use wavefront_obj::obj::{self, ObjSet};
    use crate::triangle::triangle::Triangle;
    use crate::math::math::Vec3f;
    
    pub struct Scene {
        pub lights: Vec<Light>,
        //pub objects: Vec<Box<dyn Traceable>>,
        pub objects: Vec<ObjSetWrapper>,
    }
    
    pub struct ObjSetWrapper {
        pub objset: ObjSet,
    }
    
    impl Scene {
        pub fn new() -> Self {
            Scene {
                lights: Vec::new(),
                objects: Vec::new(),
            }
        }
    
        pub fn add_wavefront_obj(&mut self, path: &str) {
            let file_content = {
                let mut f = File::open(path).unwrap();
                let mut content = String::new();
                f.read_to_string(&mut content);
                content
            };
            let model = obj::parse(file_content).unwrap();
            let model: ObjSetWrapper = ObjSetWrapper {
                objset: model,
            };
            self.objects.push(model);
        }
        
        //pub fn add_obj(&mut self, obj: Box<dyn Traceable>) {
        //     self.objects.push(obj);
        // }

        pub fn add_light(&mut self, light: Light) {
            self.lights.push(light);
        }
    }
    
    // impl Traceable for ObjSet {
    //     fn get_distance_to(&self, ray_origin: Vec3f, ray_dir: Vec3f) -> Option<f32> {
    //         moller_trumbore(self, ray_origin, ray_dir)
    //     }
    //     fn get_normal(&self, surface_pt: Vec3f) -> Vec3f {
    //         // Vec3f::new(0.0, 0.0, 0.0)
    //         self.normal
    //     }
    // }
    
    pub struct IterObjSet<'a> {
        objset: &'a ObjSet,
        oidx: usize,
        gidx: usize,
        sidx: usize,
    }
    
    impl<'a> Iterator for IterObjSet<'a> {
        type Item = Triangle;
        fn next(&mut self) -> Option<Self::Item> {
            
            let object = self.objset.objects.get(self.oidx)?;
            let geometry = object.geometry.get(self.gidx)?;
            let shape = geometry.shapes.get(self.sidx)?;
            
            let coord_idx = match shape.primitive {
                obj::Primitive::Triangle(
                    (coord_a, a1, a2),
                    (coord_b, b1, b2),
                    (coord_c, c1, c2),
                ) => {
                    //println!("IterObjSet {}:{}:{}", coord_a, coord_b, coord_c);
                    Some((coord_a, coord_b, coord_c))
                },
                _ => {
                    //println!("Unsupported primitive!");
                    None
                },
            }?;
            
            let a = object.vertices[coord_idx.0];
            let b = object.vertices[coord_idx.1];
            let c = object.vertices[coord_idx.2];
            
            self.sidx += 1;
            if self.sidx >= geometry.shapes.len() {
                self.sidx = 0;
                self.gidx += 1;
            }
            if self.gidx >= object.geometry.len() {
                self.gidx = 0;
                self.oidx += 1;
            }
            
            Some(
                Triangle::new(
                    Vec3f::new(a.x as f32, a.y as f32, a.z as f32 - 10.0),
                    Vec3f::new(b.x as f32, b.y as f32, b.z as f32 - 10.0),
                    Vec3f::new(c.x as f32, c.y as f32, c.z as f32 - 10.0),
                )
            )
        }
    }
    
    impl ObjSetWrapper {
        pub fn iter(&self) -> IterObjSet {
            IterObjSet {
                objset: &self.objset,
                oidx: 0,
                gidx: 0,
                sidx: 0,
            }
        }
    }
}
