
use crate::geometry::aabb::Aabb;
use crate::geometry::{Point3d, TraceablePrimitive};
use std::cmp::Ordering;
use rayon::prelude::*;
use std::ops::Deref;
use crate::geometry::triangle::Triangle;

const OCTREE_MAX_NUM_CHILDREN: usize = 8;

struct OctreeItem {
	idx: usize,
	centroid: Point3d,
}
struct OctreeLeafNode {
	parent: Option<usize>,
	bb: Aabb,
	items: [Option<usize>; OCTREE_MAX_NUM_CHILDREN]
}
struct OctreeInnerNode {
	parent: Option<usize>,
	bb: Aabb,
	children: [Option<usize>; OCTREE_MAX_NUM_CHILDREN],
}
enum OctreeNode {
	Leaf(OctreeLeafNode),
	Inner(OctreeInnerNode),
}
impl OctreeNode {
	fn get_bb (&self) -> Aabb {
		match self {
			OctreeNode::Inner(n) => n.bb,
			OctreeNode::Leaf(n) => n.bb,
		}
	}
	fn set_bb (&mut self, bb: Aabb) {
		match self {
			OctreeNode::Inner(n) => n.bb = bb,
			OctreeNode::Leaf(n) => n.bb = bb,
		}
	}
	// fn get_children (&self) -> [Option<usize>; OCTREE_MAX_NUM_CHILDREN] {
	// 	match self {
	// 		OctreeNode::Inner(n) => n.children,
	// 		OctreeNode::Leaf(n) => [None; OCTREE_MAX_NUM_CHILDREN],
	// 	}
	// }
	fn set_child (&mut self, child_idx: usize, val: usize) {
		match self {
			OctreeNode::Inner(n) => n.children[child_idx] = Some(val),
			OctreeNode::Leaf(n) => panic!(),
		}
	}
	// fn get_items (&self) -> [Option<usize>; OCTREE_MAX_NUM_CHILDREN] {
	// 	match self {
	// 		OctreeNode::Inner(n) => [None; OCTREE_MAX_NUM_CHILDREN],
	// 		OctreeNode::Leaf(n) => n.items,
	// 	}
	// }
}
impl OctreeLeafNode {
	fn new() -> OctreeLeafNode {
		OctreeLeafNode {
			parent: None,
			bb: Aabb::new(),
			items: [None; 8],
		}
	}
}
impl OctreeInnerNode {
	fn new() -> OctreeInnerNode {
		OctreeInnerNode {
			parent: None,
			bb: Aabb::new(),
			children: [None; 8],
		}
	}
}

struct Octree {
	nodes: Vec<OctreeNode>,
	max_node_capacity: usize,
}

impl Octree {
	pub fn new(primitives: &[Triangle], max_node_capacity: usize) -> Octree {
		let min_num_nodes = Octree::get_min_num_nodes(primitives.len(), max_node_capacity);
		let mut octree = Octree {
			nodes: Vec::with_capacity(min_num_nodes),
			max_node_capacity,
		};
		
		let mut items: Vec<OctreeItem> = primitives
			.iter()
			.enumerate()
			.map(|(idx, t)| OctreeItem {idx, centroid: t.get_centroid()})
			.collect();
		
		octree.build(primitives, &mut items, None);
		octree
	}
	fn get_min_num_nodes(num_elems: usize, max_node_capacity: usize) -> usize {
		let depth = (num_elems as f32).log2().ceil() / (max_node_capacity as f32).log2().ceil();
		let num_nodes = (8.0_f32.powf(depth + 1.0) - 1.0) / 7.0;
		num_nodes as usize
	}
	
	fn sort_and_split <T>(elems: &mut [OctreeItem], sort: T) -> (&mut [OctreeItem], &mut [OctreeItem])
	where T: FnOnce(&Point3d, &Point3d) -> Ordering + Send + Copy + 'static
	{
		elems.sort_by(|a, b| sort(&a.centroid, &b.centroid));
		let (below, above) = elems.split_at_mut((elems.len() / 2) as usize);
		(below, above)
	}
	
	pub fn build(&mut self, primitives: &[Triangle], elems: &mut [OctreeItem], parent_idx: Option<usize>) -> (usize, Aabb) {
		let sort_x = |p0: &Point3d, p1: &Point3d| p0.x.partial_cmp(&p1.x).unwrap();
		let sort_y = |p0: &Point3d, p1: &Point3d| p0.y.partial_cmp(&p1.y).unwrap();
		let sort_z = |p0: &Point3d, p1: &Point3d| p0.z.partial_cmp(&p1.z).unwrap();
		// let sort_y = |p: Point3d| p.y;
		// let sort_z = |p: Point3d| p.z;
		if elems.len() <= self.max_node_capacity {
			let leaf_idx = self.push_leaf(primitives, elems, parent_idx);
			let leaf_bb = self.nodes[leaf_idx].get_bb();
			return (leaf_idx, leaf_bb);
		}
		else {
			let inner_idx = self.push_inner(elems, parent_idx);
			
			let (left, right) = Octree::sort_and_split(elems, sort_x);
			let (left_bot, left_top) = Octree::sort_and_split(left, sort_y);
			let (right_bot, right_top) = Octree::sort_and_split(right, sort_y);
			let (left_bot_near, left_bot_far) = Octree::sort_and_split(left_bot, sort_z);
			let (left_top_near, left_top_far) = Octree::sort_and_split(left_top, sort_z);
			let (right_bot_near, right_bot_far) = Octree::sort_and_split(right_bot, sort_z);
			let (right_top_near, right_top_far) = Octree::sort_and_split(right_top, sort_z);
			let mut array: [&mut [OctreeItem]; 8] = [
				left_bot_near,
				left_bot_far,
				left_top_near,
				left_top_far,
				right_bot_near,
				right_bot_far,
				right_top_near,
				right_top_far
			];
			
			let mut inner_bb = Aabb::new();

			array.iter_mut().enumerate().for_each(|(idx, slice)| {
				if slice.len() > 0 {
					let (child_idx, child_bb) = self.build(primitives, slice, Some(inner_idx));
					self.nodes[inner_idx].set_child(idx, child_idx);
					inner_bb += child_bb;
				}
			});
			self.nodes[inner_idx].set_bb(inner_bb);
			(inner_idx, inner_bb)
		}
	}
	
	// returns the index of the created leaf node
	fn push_leaf(&mut self, primitives: &[Triangle], elems: &[OctreeItem], parent_idx: Option<usize>) -> usize {
		let mut leaf = OctreeLeafNode::new();//TODO
		leaf.parent = parent_idx;
		elems.iter().enumerate().for_each(|(idx, item)| {
			leaf.items[idx] = Some(item.idx);
			leaf.bb += primitives[item.idx].get_bounding_box();
		});
		self.nodes.push(OctreeNode::Leaf(leaf));
		self.nodes.len() - 1
	}
	fn push_inner(&mut self, elems: &[OctreeItem], parent_idx: Option<usize>) -> usize {
		let mut inner = OctreeInnerNode::new();
		inner.parent = parent_idx;
		self.nodes.push(OctreeNode::Inner(inner));
		self.nodes.len() - 1
	}
	// pub fn find(&self, f: FnOnce()) -> Option<[Option<usize>; 8]> {
	// 	None
	// }
}

#[cfg(test)]
mod tests {
	use crate::geometry::octree::Octree;
	use super::*;
	use wavefront_obj::obj::Primitive::Point;
	
	const max_node_capacity: usize = 1;
	
	#[test]
	fn test1() {
		let mut triangles: Vec<Triangle> = Vec::new();
		triangles.push(
			Triangle::new(
				Point3d::from_coords(1.0, 1.0, 1.0),
				Point3d::from_coords(2.0, 2.0, 2.0),
				Point3d::from_coords(3.0, 3.0, 3.0),
			)
		);
		let octree = Octree::new(&*triangles, max_node_capacity);
		octree.nodes.iter().for_each(|n| println!("{}", n.get_bb()));
	}
}