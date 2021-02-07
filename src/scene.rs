use crate::light::Light;
use crate::math::{Mat4f, Point3d, Point4d, Vector3d};
use crate::primitives::triangle::Triangle;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use wavefront_obj::obj::{self, ObjSet};
use wavefront_obj::ParseError;

pub struct Scene {
    pub lights: Vec<Light>,
    //pub objects: Vec<WfObj>,
    pub objects: Vec<Box<dyn TraceableObj>>,
}

pub struct Mesh {
    pub lights: Vec<Light>,
    pub triangles: Vec<Triangle>,
    pub vtx_normals: Vec<Vector3d>,
    pub txt_coords: Vec<Point3d>,
}

pub struct WfObj {
    model: Arc<ObjSet>,
    scale: [f32; 3],
    rotation: [f32; 3],
    translation: [f32; 3],
    //model_to_world: Mat4f,
    // world_to_model: Mat4f,
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

    pub fn add_obj(&mut self, obj: WfObj) {
        self.objects.push(Box::new(obj));
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

pub struct IterWfObj<'a> {
    wfobj: &'a WfObj,
    oidx: usize,
    gidx: usize,
    sidx: usize,
}

impl<'a> Iterator for IterWfObj<'a> {
    type Item = Triangle;
    fn next(&mut self) -> Option<Self::Item> {
        let object = self.wfobj.model.objects.get(self.oidx)?;
        let geometry = object.geometry.get(self.gidx)?;
        let shape = geometry.shapes.get(self.sidx)?;

        let coord_idx = match shape.primitive {
            obj::Primitive::Triangle((coord_a, a1, a2), (coord_b, b1, b2), (coord_c, c1, c2)) => {
                //println!("IterObjSet {}:{}:{}", coord_a, coord_b, coord_c);
                Some((coord_a, coord_b, coord_c))
            }
            _ => {
                //println!("Unsupported primitive!");
                None
            }
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

        Some(Triangle::new(
            Point3d::from_coords(a.x as f32 * 10.0, a.y as f32 * 10.0, a.z as f32),
            Point3d::from_coords(b.x as f32 * 10.0, b.y as f32 * 10.0, b.z as f32),
            Point3d::from_coords(c.x as f32 * 10.0, c.y as f32 * 10.0, c.z as f32),
        ))
    }
}

impl WfObj {
    pub fn new(model: Arc<ObjSet>) -> Self {
        WfObj {
            model,
            scale: [0.0, 0.0, 0.0],
            rotation: [0.0, 00.0, 0.0],
            translation: [0.0, 0.0, 0.0],
            //model_to_world: Mat4f::identity(),
            // world_to_model: Mat4f::identity(),
        }
    }

    pub fn iter(&self) -> IterWfObj {
        IterWfObj {
            wfobj: &self,
            oidx: 0,
            gidx: 0,
            sidx: 0,
        }
    }
}

impl TraceableObj for WfObj {
    fn triangulate(&self) -> Vec<Triangle> {
        let model_to_world = self.set_model_mtx();
        self.iter().map(|t| {
            Triangle::new(
                Point3d::from(&model_to_world * Point4d::from(t.v[0])),
                Point3d::from(&model_to_world * Point4d::from(t.v[1])),
                Point3d::from(&model_to_world * Point4d::from(t.v[2])),
            )
        }).collect()
    }
    
    fn set_model_mtx(&self) -> Mat4f {
        let t = translate_xyz(&Mat4f::identity(), &self.translation);
        let rx = rotate_about_x(&t, self.rotation[0]);
        let rxy = rotate_about_y(&rx, self.rotation[1]);
        let rxyz = rotate_about_z(&rxy, self.rotation[2]);
        let model_to_world = scale_xyz(&rxyz, &self.scale);
        model_to_world
    }
    
    // fn get_model_mtx(&self) -> &Mat4f {
    //     &self.model_to_world
    // }
    
    fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = [x, y, z];
    }
    fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = [x, y, z];
    }
    fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.translation = [x, y, z];
    }
}

fn rotate_about_x(m: &Mat4f, angle: f32) -> Mat4f {
    let sin = angle.to_radians().sin();
    let cos = angle.to_radians().cos();
    let rx = Mat4f {
        raw: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    m * &rx
}

fn rotate_about_y(m: &Mat4f, angle: f32) -> Mat4f {
    let sin = angle.to_radians().sin();
    let cos = angle.to_radians().cos();
    let ry = Mat4f {
        raw: [
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    m * &ry
}


fn rotate_about_z(m: &Mat4f, angle: f32) -> Mat4f {
    let sin = angle.to_radians().sin();
    let cos = angle.to_radians().cos();
    let rz = Mat4f {
        raw: [
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    m * &rz
}

fn translate_xyz(m: &Mat4f, translation: &[f32]) -> Mat4f {
    let t = Mat4f {
        raw: [
            [1.0, 0.0, 0.0, translation[0]],
            [0.0, 1.0, 0.0, translation[1]],
            [0.0, 0.0, 1.0, translation[2]],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    m * &t
}

fn scale_xyz(m: &Mat4f, scale: &[f32]) -> Mat4f {
    let s = Mat4f {
        raw: [
            [scale[0], 0.0, 0.0, 0.0],
            [0.0, scale[1], 0.0, 0.0],
            [0.0, 0.0, scale[2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    m * &s
}
