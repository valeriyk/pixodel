use light::Light;

use crate::geometry::{Point3d, Vector3d};
use crate::geometry::triangle::Triangle;
use crate::scene::objects::{TraceableObject, TraceablePrimitive};
pub use crate::scene::objects::triangle::TriObj;
pub use crate::scene::objects::wfobj::WfObj;
use crate::geometry::aabb::Aabb;
use std::ops::Range;

pub mod light;
pub mod objects;
pub mod tracing;
mod shading;
mod bvhtree;

pub struct Scene {
    pub lights: Vec<Light>,
    pub objects: Vec<Box<dyn TraceableObject>>,
}


#[repr(C, align(32))]
struct VtxAttr {
    vtx_coords: Point3d,
    norm_coords: Vector3d,
    txt_coords: Point3d,
}


impl Scene {
    pub fn new() -> Self {
        Scene {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn add_obj(&mut self, obj: Box<dyn TraceableObject>) {
        self.objects.push(obj);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn to_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new();
        //let mut tmp: Vec<BvhInfo> = Vec::new();
        for mut obj in &self.objects {
            for t in obj.triangulate() {
                mesh.triangles.push(t);
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

pub struct Mesh {
    pub lights: Vec<Light>,
    pub triangles: Vec<Triangle>,
    pub vtx_normals: Vec<Vector3d>,
    pub txt_coords: Vec<Point3d>,
    pub b_boxes: Vec<Aabb>,
    // pub centroids: Vec<Point3d>,
    // pub bounding_box: Aabb,
    //pub root: BvhTreeNode,
    pub bvh_nodes: Vec<Node>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            lights: Vec::new(),
            triangles: Vec::new(),
            vtx_normals: Vec::new(),
            txt_coords: Vec::new(),
            b_boxes: Vec::new(),
            // centroids: Vec::new(),
            // bounding_box: Aabb::new(),
            bvh_nodes: Vec::new(),
        }
    }
    
    pub fn build_bvh(&mut self, v: &Vec<Centroid>) -> usize {
        
        let (mut min, _) = v[0]; //TODO refactor
        let (mut max, _) = v[0]; //TODO refactor
        let mut node_bbox = Aabb::new();
        
        for &(val, idx) in v.iter() {
            if val.x < min.x { //TODO refactor
                min = val;
            }
            if val.x > max.x { //TODO refactor
                max = val;
            }
            node_bbox = node_bbox.get_superset(self.b_boxes[idx]);
        }
        
        let mut n = Node::new(node_bbox);
        self.bvh_nodes.push(n);
        let node_idx = self.bvh_nodes.len() - 1;
        
        if min.x == max.x || v.len() < 8 { // TODO refactor
            // we are the leaf node
            v.iter().for_each(|&(_, idx)| self.bvh_nodes[node_idx].pointers.push(idx));
            
        } else {
            // we are the inner node
            let mid_value = (min.x + max.x) / 2.0; //TODO refactor
            let (left, right): (Vec<Centroid>, Vec<Centroid>) = v.iter().partition(|&&(x, _)| x.x < mid_value);
            //std::mem::drop(v); //TODO is this needed for memory usage optimization?
            if left.len() > 0 {
                let child0 = Mesh::build_bvh(self, &left);
                self.bvh_nodes[node_idx].children.push(child0);
            }
            if right.len() > 0 {
                let child1 = Mesh::build_bvh(self, &right);
                self.bvh_nodes[node_idx].children.push(child1);
            }
        }
    
        //self.bvh_nodes.len() - 1
        node_idx
    }
}

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

type Centroid = (Point3d, usize);
