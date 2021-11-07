//use std::ops::Range;

//use mesh::Mesh;

use geometry::ray::Ray3d;
use geometry::{Mat4f, Point3d, Point4d, PrimitiveType, TraceablePrimitive, Vector3d};
use geometry::aabb::Aabb;
use geometry::triangle::Triangle;
pub use crate::scene::light::Light;
pub use crate::scene::sphere::SphereObj;
pub use crate::scene::triangle::TriObj;
pub use crate::scene::wfobj::WfObj;
use std::ops::Deref;
use std::sync::Arc;

use lbvh::*;
use wavefront_obj::obj::Primitive;
//use crate::traceable::PrimitiveType;

pub mod light;
pub mod triangle;
pub mod wfobj;
//pub mod tracing;
mod bvhtree;
pub mod shading;
mod sphere;
//pub(crate) mod mesh;

// struct FlatObj {
// 	p: Vec<(PrimitiveType, Aabb)>,
// }

pub struct Scene {
    pub lights: Vec<Light>,
    pub objects: Vec<SceneObj>,
    //pub mesh: Vec<SceneMeshItem>,
    primitives: Vec<PrimitiveType>,
    boundboxes: Vec<Aabb>,
    //centroids: Vec<Point3d>,
    bvh_nodes: Vec<Node>,
}

// #[repr(C, align(32))]
// struct VtxAttr {
//     vtx_coords: Point3d,
//     norm_coords: Vector3d,
//     txt_coords: Point3d,
// }

type IndexedCentroid = (usize, Point3d);

fn reflection_dir(surface_normal: Vector3d, surface_to_camera: Vector3d) -> Vector3d {
    let l2n_cos = surface_to_camera * surface_normal;
    surface_normal * l2n_cos * 2.0 - surface_to_camera
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            lights: Vec::new(),
            objects: Vec::new(),
            primitives: Vec::new(),
            boundboxes: Vec::new(),
            //centroids: Vec::new(),
            bvh_nodes: Vec::new(),
        }
    }
    pub fn add_obj(mut self, obj: SceneObj) -> Self {
        let model = obj.get_model_mtx();
        obj.object.to_primitives().into_iter().for_each(|x_model| {
            let x_world = x_model.model_to_world(&model);
            let x_world_bb = x_world.get_bounding_box();
            let x_world_bb_centroid = x_world_bb.get_centroid();
            self.primitives.push(x_world);
            self.boundboxes.push(x_world_bb);
            //self.centroids.push(x_world_bb_centroid);
        });
        //self.objects.push(obj); //TODO remove
        self
    }
    pub fn add_light(mut self, light: Light) -> Self {
        self.lights.push(light);
        self
    }

    // pub fn to_mesh(&self) -> Mesh {
    //     let mut mesh = Mesh::new();
    //     //let mut tmp: Vec<BvhInfo> = Vec::new();
    //     for obj in &self.objects {
    //         for t in obj.apply_model_transform().iter() {
    //             mesh.triangles.push(t.clone());
    //             mesh.b_boxes.push(t.get_bounding_box());
    //             /*let info = BvhInfo {
    //                 bb: t.get_bounding_box(),
    //                 centroid: t.get_bounding_box().get_centroid(),
    //                 triangle: t,
    //             };
    //             mesh.bounding_box.get_superset(info.bb);
    //             tmp.push(info);*/
    //         }
    //     }
    //
    //     let centroids: Vec<Centroid> = mesh.b_boxes.iter()
    //         .enumerate()
    //         .map(|(idx, &t)| (t.get_centroid(), idx))
    //         .collect();
    //
    //     mesh.build_bvh(&centroids);
    //
    //     mesh.lights = self.lights.clone();
    //     mesh
    // }
    pub fn accelerate(&mut self) {
        let centroids: Vec<IndexedCentroid> = self
            .boundboxes
            .iter()
            .enumerate()
            .map(|(idx, &bb)| (idx, bb.get_centroid()))
            .collect();
        self.build_bvh(centroids.as_slice());
    }
    
    pub fn build_lbvh<const N: usize>(&self) -> Octree<PrimitiveType, N>{
        lbvh::Octree::<PrimitiveType, N>::new(&self.primitives)
    }

    pub fn build_bvh(&mut self, v: &[IndexedCentroid]) -> usize {
        let (_, mut min) = v[0]; //TODO refactor
        let (_, mut max) = v[0]; //TODO refactor
        let mut node_bbox = Aabb::new();

        for &(idx, val) in v.iter() {
            let mut needs_update = false;

            if val.x < min.x {
                //TODO refactor
                min = val;
                needs_update = true;
            }
            // if val.y < min.y { //TODO refactor
            // 	needs_update = true;
            // }
            // if val.z < min.z { //TODO refactor
            // 	needs_update = true;
            // }
            if val.x > max.x {
                //TODO refactor
                max = val;
                needs_update = true;
            }
            // if val.y > max.y { //TODO refactor
            // 	needs_update = true;
            // }
            // if val.z > max.z { //TODO refactor
            // 	needs_update = true;
            // }
            //if needs_update {
            //node_bbox = node_bbox.get_superset(self.boundboxes[idx]);
            node_bbox = node_bbox + self.boundboxes[idx];
            //}
        }

        let n = Node::new(node_bbox);
        self.bvh_nodes.push(n);
        let node_idx = self.bvh_nodes.len() - 1;

        if min.x == max.x || v.len() < 8 {
            // TODO refactor
            // we are the leaf node
            v.iter()
                .for_each(|&(idx, _)| self.bvh_nodes[node_idx].pointers.push(idx));
        } else {
            // we are the inner node
            let mid_value = (min.x + max.x) / 2.0; //TODO refactor
            let (left, right): (Vec<IndexedCentroid>, Vec<IndexedCentroid>) =
                v.into_iter().partition(|(idx, x)| x.x < mid_value); //TODO add more dimensions
                                                                     //std::mem::drop(v); //TODO is this needed for memory usage optimization?
            if left.len() > 0 {
                let child0 = self.build_bvh(&left);
                self.bvh_nodes[node_idx].children.push(child0);
            }
            if right.len() > 0 {
                let child1 = self.build_bvh(&right);
                self.bvh_nodes[node_idx].children.push(child1);
            }
        }

        //self.bvh_nodes.len() - 1
        node_idx
    }

    pub fn cast_ray<F>(&self, ray: &Ray3d, vtx_shader: &F, depth: usize) -> [u8; 3]
    where
        F: FnOnce(Point3d, Point3d, Vector3d, &Vec<Light>) -> f32 + Send + Copy + 'static,
    {
        const BG_COLOR: [u8; 3] = [30u8; 3];
        const DEPTH_THRESHOLD: usize = 0;

        if depth < DEPTH_THRESHOLD {
            return BG_COLOR;
        }

        let mut distance_to_nearest_obj2 = f32::MAX;
        let mut nearest_obj_idx2: Option<usize> = None;

        let mut node_stack: Vec<usize> = Vec::new();
        node_stack.push(0);
        while node_stack.len() > 0 {
            let node_idx = node_stack.pop();
            if node_idx == None {
                break;
            }
            let node_idx = node_idx.unwrap();
            let current_node = &self.bvh_nodes[node_idx];
            let distance_to_bb = current_node.bound.get_distance_to(ray);
            if distance_to_bb != None {
                if current_node.children.len() == 0 {
                    // we're in a leaf
                    for &i in &current_node.pointers {
                        let distance_to_obj = self.primitives[i].get_distance_to(ray);
                        match distance_to_obj {
                            Some(dist) if dist < distance_to_nearest_obj2 => {
                                distance_to_nearest_obj2 = dist;
                                nearest_obj_idx2 = Some(i);
                            }
                            _ => (),
                        }
                    }
                } else {
                    current_node
                        .children
                        .iter()
                        .for_each(|&x| node_stack.push(x));
                }
            }
        }

        if let Some(idx) = nearest_obj_idx2 {
            let surface_pt = *ray * distance_to_nearest_obj2;
            let surface_normal: Vector3d = self.primitives[idx].get_normal(&surface_pt);

            let refl_dir = reflection_dir(surface_normal, -ray.get_direction()).normalize(); //TODO: normalize really needed?

            //let refl_color = self.cast_ray(&surface_pt, &refl_dir, vtx_shader, depth + 1);

            let mut illumination =
                vtx_shader(surface_pt, ray.get_origin(), surface_normal, &self.lights);
            if illumination > 1.0 {
                illumination = 1.0
            }
            let self_color = [(illumination * u8::MAX as f32) as u8; 3];
            //[refl_color[0] + self_color[0]; 3]
            [self_color[0]; 3]
        //[100,100,100]
        } else {
            BG_COLOR
        }
    }
    
    pub fn cast_ray_lbvh<F, const N: usize>(&self, lbvh: &Octree<PrimitiveType, N>, ray: &Ray3d, vtx_shader: &F, depth: usize) -> [u8; 3]
        where
            F: FnOnce(Point3d, Point3d, Vector3d, &Vec<Light>) -> f32 + Send + Copy + 'static,
    {
        const BG_COLOR: [u8; 3] = [30u8; 3];
        const DEPTH_THRESHOLD: usize = 0;
        
        if depth < DEPTH_THRESHOLD {
            return BG_COLOR;
        }
        
        let nearest: Option<(usize, f32)> = lbvh.traverse(ray);
        
        if nearest != None {
            let (nearest_obj, dist) = nearest.unwrap();
            let surface_pt = *ray * dist;
            let surface_normal: Vector3d = self.primitives[nearest_obj].get_normal(&surface_pt);
            
            let refl_dir = reflection_dir(surface_normal, -ray.get_direction()).normalize(); //TODO: normalize really needed?
            
            //let refl_color = self.cast_ray(&surface_pt, &refl_dir, vtx_shader, depth + 1);
            
            let mut illumination =
                vtx_shader(surface_pt, ray.get_origin(), surface_normal, &self.lights);
            if illumination > 1.0 {
                illumination = 1.0
            }
            let self_color = [(illumination * u8::MAX as f32) as u8; 3];
            //[refl_color[0] + self_color[0]; 3]
            [self_color[0]; 3]
            //[100,100,100]
        } else {
            BG_COLOR
        }
    }
}

//type Centroid = (Point3d, usize);

// pub trait IntoTriangles {
// 	fn triangulate(&self) -> Vec<Triangle>;
// }
pub trait IntoPrimitives {
    fn to_primitives(&self) -> Vec<PrimitiveType>;
}
pub struct SceneObj {
    object: Arc<dyn IntoPrimitives + Sync + Send>,
    scale: [f32; 3],
    rotation: [f32; 3],
    translation: [f32; 3],
    //model_to_world: Mat4f,
    // world_to_model: Mat4f,
}

impl SceneObj {
    pub fn new(obj: Arc<dyn IntoPrimitives + Send + Sync>) -> Self {
        SceneObj {
            object: obj,
            scale: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            translation: [0.0, 0.0, 0.0],
        }
    }

    fn get_model_mtx(&self) -> Mat4f {
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
    // pub fn apply_model_transform(&self) -> Vec<Triangle> {
    // 	let model_to_world = self.get_model_mtx();
    // 	self.primitives.iter().map(|t|
    // 		Triangle::new(
    // 			Point3d::from(&model_to_world * Point4d::from(t.v[0])),
    // 			Point3d::from(&model_to_world * Point4d::from(t.v[1])),
    // 			Point3d::from(&model_to_world * Point4d::from(t.v[2])),
    // 		)
    // 	).collect()
    // }
}

// struct TraceableObject {
// 	map: Option<Vec<u8>>,
// 	texture: Option<Vec<u8>>,
// 	primitives: Vec<PrimitiveType>,
// }

pub struct Node {
    //    parent: Option<usize>,
    children: Vec<usize>,
    pointers: Vec<usize>,
    bound: Aabb,
}

impl Node {
    pub fn new(bound: Aabb) -> Self {
        Self {
            children: Vec::<usize>::new(),
            pointers: Vec::<usize>::new(),
            bound,
        }
    }
}
