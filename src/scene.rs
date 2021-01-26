pub mod scene {
	use crate::{Light, Traceable};
	
	pub struct Scene {
		pub lights: Vec<Light>,
		//pub objects: Vec<TraceableObj>,
		//objects2: Vec<&'a Traceable>,
		pub objects: Vec<Box<Traceable>>,
	}
	
	impl Scene {
		pub fn new() -> Self {
			Scene { lights: Vec::new(), objects: Vec::new() }
		}
		//pub fn add_obj(&mut self, obj: impl Traceable) {
		pub fn add_obj(&mut self, obj: Box<Traceable>) {
			self.objects.push(obj);
		}
	}
}