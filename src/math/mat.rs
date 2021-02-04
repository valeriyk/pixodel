pub struct Mat4f {
    mat: [[f32; 4]; 4],
}

impl Mat4f {
    pub fn new() -> Self {
        Mat4f {
            mat: [[0.0 as f32; 4]; 4],
        }
    }
    // pub fn new_identity() -> Self {
    //     Mat4f {
    //         mat: [[0.0 as f32; 4]; 4],
    //     }
    // }
}
