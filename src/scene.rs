pub mod scene {
    use crate::{Light, Traceable};

    pub struct Scene {
        pub lights: Vec<Light>,
        pub objects: Vec<Box<dyn Traceable>>,
    }

    impl Scene {
        pub fn new() -> Self {
            Scene {
                lights: Vec::new(),
                objects: Vec::new(),
            }
        }

        pub fn add_obj(&mut self, obj: Box<dyn Traceable>) {
            self.objects.push(obj);
        }

        pub fn add_light(&mut self, light: Light) {
            self.lights.push(light);
        }
    }
}
