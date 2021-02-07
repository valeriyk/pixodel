pub mod light;
pub mod primitives;
pub mod wfobj;

use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;

use wavefront_obj::obj::{self, ObjSet};
use wavefront_obj::ParseError;

use light::Light;
use primitives::triangle::Triangle;

use crate::math::{Mat4f, Point3d, Point4d, Vector3d};
pub use crate::scene::wfobj::WfObj;



pub struct Scene {
    pub lights: Vec<Light>,
    pub objects: Vec<Box<dyn TraceableObj>>,
}

pub struct Mesh {
    pub lights: Vec<Light>,
    pub triangles: Vec<Triangle>,
    pub vtx_normals: Vec<Vector3d>,
    pub txt_coords: Vec<Point3d>,
}


pub trait TraceableObj {
    fn triangulate(&self) -> Vec<Triangle>;
    fn set_model_mtx(&self) -> Mat4f;
    //fn get_model_mtx(&self) -> &Mat4f;
    fn rotate(&mut self, x: f32, y: f32, z: f32);
    fn scale(&mut self, x: f32, y: f32, z: f32);
    fn translate(&mut self, x: f32, y: f32, z: f32);
}

#[repr(C, align(32))]
struct VtxAttr {
    vtx_coords: Point3d,
    norm_coords: Vector3d,
    txt_coords: Point3d,
}

pub fn new_wavefront_obj(path: &str) -> Result<ObjSet, ParseError> {
    let file_content = {
        let mut f = File::open(path).unwrap();
        let mut content = String::new();
        f.read_to_string(&mut content);
        content
    };
    obj::parse(file_content)
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn add_obj(&mut self, obj: Box<dyn TraceableObj>) {
        self.objects.push(obj);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn to_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new();
        for mut obj in &self.objects {
            for t in obj.triangulate() {
                mesh.triangles.push(t);
            }
        }
        
        mesh.lights = self.lights.clone();
        mesh
    }
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            lights: Vec::new(),
            triangles: Vec::new(),
            vtx_normals: Vec::new(),
            txt_coords: Vec::new(),
        }
    }
}





