use crate::geometry::{Point3d, TraceablePrimitive, Vector3d};
use crate::geometry::aabb::Aabb;
use crate::geometry::triangle::Triangle;
use crate::scene::{Centroid, shading};
use crate::scene::light::Light;

pub struct Mesh {
    pub lights: Vec<Light>,
    pub triangles: Vec<Triangle>,
    vtx_normals: Vec<Vector3d>,
    txt_coords: Vec<Point3d>,
    pub b_boxes: Vec<Aabb>,
    // pub centroids: Vec<Point3d>,
    // pub bounding_box: Aabb,
    //pub root: BvhTreeNode,
    bvh_nodes: Vec<Node>,
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
    
    pub fn cast_ray(&self, ray_orig: &Point3d, ray_dir: &Vector3d) -> u8 {
        // let mut distance_to_nearest_obj1 = f32::MAX;
        // let mut nearest_obj_idx1: Option<usize> = None;
        
        let mut distance_to_nearest_obj2 = f32::MAX;
        let mut nearest_obj_idx2: Option<usize> = None;
        
        const BG_COLOR: u8 = 30u8;
        
        /*for (idx, triangle) in mesh.triangles.iter().enumerate() {
			let distance_to_obj = triangle.get_distance_to(ray_orig, ray_dir);
			match distance_to_obj {
				Some(dist) if dist < distance_to_nearest_obj1 => {
					distance_to_nearest_obj1 = dist;
					nearest_obj_idx1 = Some(idx);
				}
				_ => (),
			}
		}*/
        
        let mut node_stack: Vec<usize> = Vec::new();
        node_stack.push(0);
        while node_stack.len() > 0 {
            let node_idx = node_stack.pop();
            if node_idx == None {
                break;
            }
            let node_idx = node_idx.unwrap();
            let current_node = &self.bvh_nodes[node_idx];
            let distance_to_bb = current_node.bound.get_distance_to(ray_orig, ray_dir);
            if distance_to_bb != None {
                if current_node.children.len() == 0 {
                    // we're in a leaf
                    for &i in &current_node.pointers {
                        let distance_to_obj = self.triangles[i].get_distance_to(ray_orig, ray_dir);
                        match distance_to_obj {
                            Some(dist) if dist < distance_to_nearest_obj2 => {
                                distance_to_nearest_obj2 = dist;
                                nearest_obj_idx2 = Some(i);
                            }
                            _ => (),
                        }
                    }
                } else {
                    current_node.children.iter()
                        .for_each(|&x| node_stack.push(x));
                }
            }
        }
        
        // if nearest_obj_idx1 != nearest_obj_idx2 {
        // 	assert_eq!(nearest_obj_idx1, nearest_obj_idx2);
        // }
        // if distance_to_nearest_obj1 != distance_to_nearest_obj2 {
        // 	assert_eq!(distance_to_nearest_obj1, distance_to_nearest_obj2);
        // }
        
        if let Some(idx) = nearest_obj_idx2 {
            let surface_pt = (*ray_orig + *ray_dir) * distance_to_nearest_obj2;
            let norm_to_surface: Vector3d = self.triangles[idx].get_normal(&surface_pt);
            let mut illumination =
                shading::get_phong_illumination(surface_pt, *ray_orig, norm_to_surface, &self.lights);
            if illumination > 1.0 {
                illumination = 1.0
            }
            (illumination * u8::MAX as f32) as u8
        } else {
            BG_COLOR
        }
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

