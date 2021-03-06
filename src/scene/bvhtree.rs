/*use crate::geometry::aabb::Aabb;
use crate::geometry::triangle::Triangle;
use crate::geometry::Point3d;

#[derive(Clone)]
struct BvhTreeNode {
	bounding_box: Aabb,
	child: [Box<Option<BvhTreeNode>>; 2],
	triangles: Vec<Triangle>,
}

#[derive(Copy, Clone)]
struct BvhInfo {
	bb: Aabb,
	centroid: Point3d,
	triangle: Triangle,
}

impl BvhTreeNode {
	pub fn new() -> BvhTreeNode {
		BvhTreeNode {
			bounding_box: Aabb::new(),
			child: [Box::new(None); 2],
			triangles: Vec::new(),
		}
	}
	pub fn build_tree(&mut self, array: Vec<BvhInfo>) -> Self {
		let len = array.len();
		if len == 1 {
			// create leaf node
			let mut leaf = BvhTreeNode::new();
			leaf.bounding_box = array[0].bb;
			leaf.triangles.push(array[0].triangle);
			leaf
		} else {
			let midpoint = len as f32 / 2.0;
			let (left, right) = array.iter().partition(|x| x.centroid.x < midpoint);
			let mut inner = BvhTreeNode::new();
			//inner.bounding_box = TODO
			inner.child[0] = Box::new(Some(BvhTreeNode::build_tree(self, left)));
			inner.child[1] = Box::new(Some(BvhTreeNode::build_tree(self, right)));
			inner
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::test::TestResult::TrIgnored;
	use crate::scene::objects::TraceablePrimitive;
	
	#[test]
	fn test1() {
		let p0 = Point3d::from_coords(-3.0, 1.0, 1.0);
		let p1 = Point3d::from_coords(-3.0, 0.0, 1.0);
		let p2 = Point3d::from_coords(-2.0, 0.0, 1.0);
		let p3 = Point3d::from_coords(-2.0, 1.0, 1.0);
		let p4 = Point3d::from_coords(-1.0, 0.0, 1.0);
		let p5 = Point3d::from_coords(-1.0, 1.0, 1.0);
		let p6 = Point3d::from_coords(0.0, 0.0, 1.0);
		let p7 = Point3d::from_coords(1.0, 0.0, 1.0);
		let p8 = Point3d::from_coords(2.0, 2.0, 1.0);
		let p9 = Point3d::from_coords(2.0, 0.0, 1.0);
		let p10 = Point3d::from_coords(3.0, -2.0, 1.0);
	
		let t0 = Triangle::new(p0, p1, p2);
		let t1 = Triangle::new(p3, p4, p5);
		let t2 = Triangle::new(p6, p7, p8);
		let t3 = Triangle::new(p8, p9, p10);
		
		let triangles = vec![t0, t1, t2, t3];
		
		let mut node_stack: Vec<BvhTreeNode> = Vec::with_capacity(triangles.len() * 2);
		
		let mut scene_bbox = Aabb::new();
		triangles
			.iter()
			//.map(|x| x.get_bounding_box())
			.for_each(|x| scene_bbox = Aabb::get_superset(&scene_bbox, x.get_bounding_box()));
		
		node_stack.push(BvhTreeNode::new());
		
		assert_eq!(true, true);
	}
}
*/