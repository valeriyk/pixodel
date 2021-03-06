use crate::geometry::{Point3d, Vector3d};
use crate::scene::Mesh;
use crate::scene::shading;
use crate::scene::objects::TraceablePrimitive;

pub fn cast_ray(ray_orig: &Point3d, ray_dir: &Vector3d, mesh: &Mesh) -> u8 {
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
		let current_node = &mesh.bvh_nodes[node_idx];
		let distance_to_bb = current_node.bound.get_distance_to(ray_orig, ray_dir);
		if distance_to_bb != None {
			if current_node.children.len() == 0 {
				// we're in a leaf
				for &i in &current_node.pointers {
					let distance_to_obj = mesh.triangles[i].get_distance_to(ray_orig, ray_dir);
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
		let norm_to_surface: Vector3d = mesh.triangles[idx].get_normal(&surface_pt);
		let mut illumination =
			shading::get_phong_illumination(surface_pt, *ray_orig, norm_to_surface, &mesh.lights);
		if illumination > 1.0 {
			illumination = 1.0
		}
		(illumination * u8::MAX as f32) as u8
	} else {
		BG_COLOR
	}
}

