//use std::ops::Range;

use light::Light;
use mesh::Mesh;

use crate::geometry::{Mat4f, Point3d, Point4d, TraceablePrimitive};
//use crate::geometry::aabb::Aabb;
use crate::geometry::triangle::Triangle;
pub use crate::scene::triangle::TriObj;
pub use crate::scene::wfobj::WfObj;

pub mod light;
pub mod wfobj;
pub mod triangle;
//pub mod tracing;
pub(crate) mod shading;
mod bvhtree;
pub(crate) mod mesh;

pub struct Scene {
    pub lights: Vec<Light>,
    pub objects: Vec<SceneObj>,
}


// #[repr(C, align(32))]
// struct VtxAttr {
//     vtx_coords: Point3d,
//     norm_coords: Vector3d,
//     txt_coords: Point3d,
// }


impl Scene {
    pub fn new() -> Self {
        Scene {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }
    pub fn add_obj(mut self, obj: SceneObj) -> Self {
        self.objects.push(obj);
        self
    }
    pub fn add_light(mut self, light: Light) -> Self {
        self.lights.push(light);
        self
    }

    pub fn to_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new();
        //let mut tmp: Vec<BvhInfo> = Vec::new();
        for obj in &self.objects {
            for t in obj.apply_model_transform().iter() {
                mesh.triangles.push(t.clone());
                mesh.b_boxes.push(t.get_bounding_box());
                /*let info = BvhInfo {
                    bb: t.get_bounding_box(),
                    centroid: t.get_bounding_box().get_centroid(),
                    triangle: t,
                };
                mesh.bounding_box.get_superset(info.bb);
                tmp.push(info);*/
            }
        }
    
        let centroids: Vec<Centroid> = mesh.b_boxes.iter()
            .enumerate()
            .map(|(idx, &t)| (t.get_centroid(), idx))
            .collect();
    
        mesh.build_bvh(&centroids);
        
        mesh.lights = self.lights.clone();
        mesh
    }
}

type Centroid = (Point3d, usize);

pub trait IntoTriangles {
	fn triangulate(&self) -> Vec<Triangle>;
}

pub struct SceneObj {
	triangles: Vec<Triangle>,
	scale: [f32; 3],
	rotation: [f32; 3],
	translation: [f32; 3],
	//model_to_world: Mat4f,
	// world_to_model: Mat4f,
}

impl SceneObj {
	pub(crate) fn new(a: &impl IntoTriangles) -> Self {
		SceneObj {
			triangles: a.triangulate(),
			scale: [0.0, 0.0, 0.0],
			rotation: [0.0, 0.0, 0.0],
			translation: [0.0, 0.0, 0.0],
		}
	}
	
	fn set_model_mtx(&self) -> Mat4f {
		Mat4f::identity()
			.translate_xyz(&self.translation)
			.rotate_about_x(self.rotation[0])
			.rotate_about_y(self.rotation[1])
			.rotate_about_z(self.rotation[2])
			.scale_xyz(&self.scale)
	}

	pub fn rotate(mut self, x: f32, y: f32, z: f32) -> Self {
		self.rotation = [x, y, z];
		self
	}
	pub fn scale(mut self, x: f32, y: f32, z: f32) -> Self {
		self.scale = [x, y, z];
		self
	}
	pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
		self.translation = [x, y, z];
		self
	}
	pub fn apply_model_transform(&self) -> Vec<Triangle> {
		let model_to_world = self.set_model_mtx();
		self.triangles.iter().map(|t|
			Triangle::new(
				Point3d::from(&model_to_world * Point4d::from(t.v[0])),
				Point3d::from(&model_to_world * Point4d::from(t.v[1])),
				Point3d::from(&model_to_world * Point4d::from(t.v[2])),
			)
		).collect()
	}
}

