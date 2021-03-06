use crate::geometry::{Point3d, Vector3d};
use crate::scene::light::Light;

pub(crate) fn get_phong_illumination(
	surface_pt: Point3d,
	camera_pt: Point3d,
	surface_normal: Vector3d,
	lights: &Vec<Light>,
) -> f32 {
	let shininess: f32 = 20.0;
	let diffuse_reflection: f32 = 1.0;
	let specular_reflection: f32 = 0.1;
	let ambient_reflection: f32 = 0.1;
	
	let surface_to_camera = (camera_pt - surface_pt).normalize();
	let mut illumination = ambient_reflection;
	for l in lights {
		let surface_to_light = (l.position - surface_pt).normalize();
		let diffuse_factor = surface_to_light * surface_normal; // cos of the light to normal angle
		if diffuse_factor > 0.0 {
			let light_reflected_off_surface =
				surface_normal * diffuse_factor * 2.0 - surface_to_light;
			let specular_factor = light_reflected_off_surface * surface_to_camera; // cos of the camera to reflected ray angle
			let mut specular_factor = specular_factor.powf(shininess);
			if specular_factor < 0.0 {
				specular_factor = 0.0;
			}
			illumination +=
				diffuse_factor * diffuse_reflection + specular_factor * specular_reflection;
		}
	}
	illumination
}