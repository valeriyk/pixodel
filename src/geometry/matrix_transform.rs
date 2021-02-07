use crate::geometry::Mat4f;

pub fn rotate_about_x(m: &Mat4f, angle_deg: f32) -> Mat4f {
	let sin = angle_deg.to_radians().sin();
	let cos = angle_deg.to_radians().cos();
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

pub fn rotate_about_y(m: &Mat4f, angle_deg: f32) -> Mat4f {
	let sin = angle_deg.to_radians().sin();
	let cos = angle_deg.to_radians().cos();
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


pub fn rotate_about_z(m: &Mat4f, angle_deg: f32) -> Mat4f {
	let sin = angle_deg.to_radians().sin();
	let cos = angle_deg.to_radians().cos();
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

pub fn translate_xyz(m: &Mat4f, translation: &[f32]) -> Mat4f {
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

pub fn scale_xyz(m: &Mat4f, scale: &[f32]) -> Mat4f {
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