extern crate image;
extern crate rayon;

use rayon::prelude::*;

use std::time::Instant;

//use std::ops::Deref;
use std::sync::{Arc};
//use std::thread;

use image::{ImageBuffer, Rgb};

//use scene::{IntoTriangles, mesh};
use scene::light::Light;
use scene::mesh::Mesh;
use scene::shading;

use crate::geometry::{Mat4f, Point3d, Point4d};
use crate::geometry::triangle::Triangle;
//use crate::img_tiles::{Tile, TileGenerator, TilesLayout};
use crate::scene::{Scene};
use crate::scene::wfobj;

mod img_tiles;
mod geometry;
mod scene;

//const NUM_SLAVES: u32 = 8;

const FRAME_WIDTH: u32 = 640;
const FRAME_HEIGHT: u32 = 640;

// const TILE_WIDTH: u32 = 32;
// const TILE_HEIGHT: u32 = TILE_WIDTH;

fn create_scene_mesh() -> Mesh {
    

    // scene.add_obj(Box::new(Sphere::new(Vec3f::new(10.0, 10.0, -100.0), 10.0)));
    // scene.add_obj(Box::new(Sphere::new(
    //     Vec3f::new(-10.0, -10.0, -100.0),
    //     10.0,
    // )));
    // scene.add_obj(Box::new(Sphere::new(Vec3f::new(0.0, 0.0, -50.0), 5.0)));
    // scene.add_obj(Box::new(Sphere::new(Vec3f::new(-50.0, 25.0, -100.0), 8.0)));
    // scene.add_obj(Box::new(Sphere::new(Vec3f::new(50.0, -25.0, -100.0), 7.0)));
    // scene.add_obj(Box::new(Triangle::new(
    //     Vec3f::new(0.0, 10.0, -70.0),
    //     Vec3f::new(-10.0, 15.0, -50.0),
    //     Vec3f::new(2.0, -10.0, -100.0),
    // )));
    // scene.add_obj(Box::new(Triangle::new(
    //     Vec3f::new(0.0, 10.0, -70.0),
    //     Vec3f::new(2.0, -10.0, -100.0),
    //     Vec3f::new(10.0, 15.0, -50.0),
    // )));
    // scene.add_obj(Box::new(Triangle::new(
    //     Vec3f::new(10.0, 15.0, -50.0),
    //     Vec3f::new(-10.0, 15.0, -50.0),
    //     Vec3f::new(0.0, 10.0, -70.0),
    // )));
    //scene.add_wavefront_obj("models/cube2.obj");
    //scene.add_wavefront_obj("models/african_head.obj");

    let head_model = scene::WfObj::new(Arc::new(wfobj::new_wavefront_obj("models/african_head.obj").unwrap()));
    let head_0 = scene::SceneObj::new(&head_model)
        .scale(7.0, 7.0, 7.0)
        .rotate(0.0, 0.0, 0.0)
        .translate(3.0, 0.0, -30.0);
    let head_1 = scene::SceneObj::new(&head_model)
        .scale(7.0, 7.0, 7.0)
        .rotate(0.0, 0.0, 0.0)
        .translate(-3.0, 0.0, -30.0);
    
    
    
    let cube_model = scene::WfObj::new(Arc::new(wfobj::new_wavefront_obj("models/cube.obj").unwrap()));
    let _cube_0 = scene::SceneObj::new(&cube_model)
        .scale(4.0, 4.0, 4.0)
        .rotate(45.0, 45.0, 0.0)
        .translate(5.0, 0.0, -30.0);
    let _cube_1 = scene::SceneObj::new(&cube_model)
        .scale(4.0, 4.0, 4.0)
        .rotate(45.0, 45.0, 0.0)
        .translate(-5.0, 0.0, -30.0);
    
    let triangle_model = scene::TriObj::new(Triangle::new(
        Point3d::from_coords(-1.0, 1.0, 0.0),
        Point3d::from_coords(0.0, -1.0, 0.0),
        Point3d::from_coords(1.0, 0.8, 0.0),
    ));
    let _tri_0 = scene::SceneObj::new(&triangle_model)
        .scale(10.0, 10.0, 10.0)
        .rotate(-45.0, 0.0, 0.0)
        .translate(0.0, 0.0, -40.0);
    
    Scene::new()
        .add_obj(head_0)
        .add_obj(head_1)
        //.add_obj(Box::new(cube_0))
        //.add_obj(Box::new(cube_1))
        //.add_obj(Box::new(tri_0))
        //.add_light(Light::new(Point3d::from_coords(-50.0, -50.0, 50.0), 0.5))
        //.add_light(Light::new(Point3d::from_coords(10.0, 200.0, 20.0), 0.5))
        .add_light(Light::new(Point3d::from_coords(1.0, 0.0, 10.0), 0.5))
        .to_mesh()
}

//type VtxShader = Box<dyn FnOnce(Point3d, Point3d, Vector3d, &Vec<Light>) -> f32 + Send + 'static>;


fn main() {
    //let mut thread_handles = vec![];
    
    //let (tx, rx) = mpsc::channel();
    
    let frame_width = FRAME_WIDTH;
    let frame_height = FRAME_HEIGHT;
    
    let aspect_ratio = (frame_width as f32) / (frame_height as f32);
    let fov_vert: f32 = 35.0;
    let fov_scaling_factor = (fov_vert / 2.0).to_radians().tan();
    
    // First scale from the viewport shape to NDC: [0; screen] -> [0; 2]
    let screen_to_world = Mat4f::from_rows(
        [2.0 * fov_scaling_factor * aspect_ratio / frame_width as f32, 0.0, 0.0, -fov_scaling_factor * aspect_ratio],
        [0.0, 2.0 * fov_scaling_factor / frame_height as f32, 0.0, -fov_scaling_factor],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    );
    
    let ray_orig = Point3d::from_coords(0.0, 0.0, 0.0);
    
    let recursion_depth = 4;
    
    loop {
        //let mesh_glob = Arc::new(create_scene_mesh());
        let mesh_glob = create_scene_mesh();
    
        let timer = Instant::now();
    
        let mut fbuf: Vec<[u8; 3]> = vec![[0, 0, 0]; (frame_width * frame_height) as usize];
        fbuf.par_iter_mut().enumerate().for_each(|(idx, pix)| {
            let x = idx as u32 % frame_width;
            let y = idx as u32 / frame_width;
            let ray_aim = Point3d::from(&screen_to_world * Point4d::from_coords(x as f32, y as f32, -1.0, 1.0));
            let ray_dir = ray_aim - ray_orig;
            let color = mesh_glob.cast_ray(
                &ray_orig,
                &ray_dir.normalize(),
                &|a, b, c, d| shading::phong(a, b, c, d),
                recursion_depth,
            );
            *pix = color;
        });
    
        println!("Elapsed time: {:.2?}", timer.elapsed());
        
        //let fbuf = fbuf.iter().flatten().map(|x| *x).collect();
        let fbuf = fbuf.iter().flat_map(|x| *x).collect();
    
        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_vec(frame_width, frame_height, fbuf).unwrap();
        image::imageops::flip_vertical_in_place(&mut img);
        img.save("myimg.png").unwrap();
        break;
    }
}
