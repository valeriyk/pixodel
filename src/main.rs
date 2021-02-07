extern crate image;

mod img_tiles;
mod light;
mod math;
mod primitives;
mod scene;

use std::sync::{mpsc, Arc};
use std::thread;

use image::{GenericImage, ImageBuffer};

use crate::img_tiles::{Tile, TileGenerator, TilesLayout};
use crate::light::Light;
use crate::math::{Point3d, Vector3d};
use crate::primitives::Traceable;
use crate::scene::{Scene, Mesh, TraceableObj};

const NUM_SLAVES: u32 = 8;

const FRAME_WIDTH: u32 = 300;
const FRAME_HEIGHT: u32 = 400;

const TILE_WIDTH: u32 = 20;
const TILE_HEIGHT: u32 = TILE_WIDTH;

fn get_phong_illumination(
    surface_pt: Point3d,
    camera_pt: Point3d,
    surface_normal: Vector3d,
    lights: &Vec<Light>,
) -> f32 {
    let shininess: f32 = 20.0;
    let diffuse_reflection: f32 = 1.0;
    let specular_reflection: f32 = 0.5;
    let ambient_reflection: f32 = 0.05;

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

struct TileMsg {
    tile: Tile,
    sent_from: u32,
    is_last: bool,
}

fn cast_ray(ray_orig: Point3d, ray_dir: Vector3d, mesh: &Mesh) -> u8 {
    let mut distance_to_nearest_obj = f32::MAX;
    let mut nearest_obj_idx: Option<usize> = None;

    const BG_COLOR: u8 = 30u8;

    for (idx, triangle) in mesh.triangles.iter().enumerate() {
        let distance_to_obj = triangle.get_distance_to(ray_orig, ray_dir);
        match distance_to_obj {
            Some(dist) if dist < distance_to_nearest_obj => {
                distance_to_nearest_obj = dist;
                nearest_obj_idx = Some(idx);
            }
            _ => (),
        }
    }

    if let Some(idx) = nearest_obj_idx {
        let surface_pt = (ray_orig + ray_dir) * distance_to_nearest_obj;
        let norm_to_surface: Vector3d = mesh.triangles[idx].get_normal(surface_pt);
        let mut illumination =
            get_phong_illumination(surface_pt, ray_orig, norm_to_surface, &mesh.lights);
        if illumination > 1.0 {
            illumination = 1.0
        }
        (illumination * u8::MAX as f32) as u8
    } else {
        BG_COLOR
    }
}

fn slv_thread_proc(slave_idx: u32, num_slaves: u32, tx: mpsc::Sender<TileMsg>, scene: Arc<Mesh>) {
    let layout = TilesLayout::new(FRAME_WIDTH, FRAME_HEIGHT, TILE_WIDTH, TILE_HEIGHT);

    let mut tiles = TileGenerator::new(slave_idx, num_slaves, &layout);
    for mut tile in &mut tiles {
        let aspect_ratio = (layout.frame_width as f32) / (layout.frame_height as f32);
        let fov_vert: f32 = 35.0;
        let fov_scaling_factor = (fov_vert / 2.0).to_radians().tan();

        for y in tile.col_idx * tile.height..tile.col_idx * tile.height + tile.height {
            for x in tile.row_idx * tile.width..tile.row_idx * tile.width + tile.width {
                let ray_x: f32 = ((x as f32 * 2.0) / layout.frame_width as f32 - 1.0)
                    * fov_scaling_factor
                    * aspect_ratio;
                let ray_y: f32 =
                    ((y as f32 * 2.0) / layout.frame_height as f32 - 1.0) * fov_scaling_factor;
                let ray_z = -1.0;
                let ray_dir = Vector3d::from_coords(ray_x, ray_y, ray_z);

                let color = cast_ray(Point3d::new(), ray_dir.normalize(), &scene);

                tile.vbuf.push(color);
                tile.vbuf.push(u8::MIN);
                tile.vbuf.push(u8::MIN);
            }
        }
        let msg = TileMsg {
            tile,
            sent_from: slave_idx,
            is_last: false,
        };
        tx.send(msg).unwrap();
    }

    let t = Tile::new(0, 0, 0, 0);
    let msg = TileMsg {
        tile: t,
        sent_from: slave_idx,
        is_last: true,
    };
    tx.send(msg).unwrap();
}

fn fbuf_thread_proc(rx: mpsc::Receiver<TileMsg>) {
    let mut num_slaves_finished: u32 = 0;

    let mut img =
        ImageBuffer::from_pixel(FRAME_WIDTH, FRAME_HEIGHT, image::Rgb([100u8, 80u8, 60u8]));

    for received in &rx {
        if received.is_last == true {
            num_slaves_finished += 1;
        } else {
            let subimage_top_left_x = received.tile.row_idx * received.tile.width;
            let subimage_top_left_y = received.tile.col_idx * received.tile.height;

            let tile_img = ImageBuffer::from_raw(
                received.tile.width,
                received.tile.height,
                received.tile.vbuf,
            )
            .unwrap();
            img.copy_from(&tile_img, subimage_top_left_x, subimage_top_left_y)
                .unwrap();
        }

        if num_slaves_finished == NUM_SLAVES {
            image::imageops::flip_vertical_in_place(&mut img);
            img.save("myimg.png").unwrap();
            break;
        }
    }
}

//use std::io::Read;
//use std::fs::File;
//use wavefront_obj::obj::{self, ObjSet};

// fn wavefront_obj_reader(path: &str) -> ObjSet {
//     let file_content = {
//         let mut f = File::open(path).unwrap();
//         let mut content = String::new();
//         f.read_to_string(&mut content);
//         content
//     };
//
//     obj::parse(file_content).unwrap()
// }

fn create_scene() -> Scene {
    let mut scene = Scene::new();

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

    let head_model = Arc::new(scene::new_wavefront_obj("models/african_head.obj").unwrap());
    let mut head_0 = scene::WfObj::new(Arc::clone(&head_model));
    let mut head_1 = scene::WfObj::new(Arc::clone(&head_model));
    head_0.scale(0.5, 0.5, 1.0);
    head_1.scale(0.5, 0.5, 1.0);
    head_0.rotate(0.0, 0.0, 0.0);
    head_1.rotate(0.0, 0.0, 0.0);
    head_0.translate(2.0, 0.0, -30.0);
    head_1.translate(-2.0, 0.0, -30.0);
    scene.add_obj(head_0);
    scene.add_obj(head_1);

    //scene.add_light(Light::new(Vec3f::new(-50.0, -50.0, -10.0), 1.0));
    //scene.add_light(Light::new(Vec3f::new(50.0, -50.0, -10.0), 1.0));
    //scene.add_light(Light::new(Vec3f::new(0.0, -200.0, -1000.0), 1.0));
    scene.add_light(Light::new(Point3d::from_coords(0.0, 200.0, 20.0), 1.0));

    scene
}

fn main() {
    let mut thread_handles = vec![];

    let (tx, rx) = mpsc::channel();

    //let scene_glob = Arc::new(create_scene().refresh());
    let mesh_glob = Arc::new(create_scene().to_mesh());

    for i in 0..NUM_SLAVES {
        let tx_slv_to_fbuf = mpsc::Sender::clone(&tx);
        let mesh = mesh_glob.clone();

        let mut thread_name = String::from("slv_thread_proc");
        thread_name.push_str(&i.to_string());
        let builder = thread::Builder::new()
            .name(thread_name)
            .stack_size(32000000);
        let handle = builder
            .spawn(move || slv_thread_proc(i, NUM_SLAVES, tx_slv_to_fbuf, mesh))
            .unwrap();

        thread_handles.push(handle);
    }

    let handle = thread::spawn(|| fbuf_thread_proc(rx));
    thread_handles.push(handle);

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
