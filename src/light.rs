pub mod light {
    use crate::math::math::Vec3f;

    pub struct Light {
        pub position: Vec3f,
        pub intensity: f32,
    }

    impl Light {
        pub fn new(position: Vec3f, intensity: f32) -> Light {
            Light {
                position,
                intensity,
            }
        }
    }
}
