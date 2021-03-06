extern crate image;

use std::sync::{Arc, mpsc};
use std::thread;

use image::{GenericImage, ImageBuffer};

use scene::light::Light;
use scene::tracing;

use crate::geometry::{Point3d, Mat4f, Point4d};
use crate::geometry::triangle::Triangle;
use crate::img_tiles::{Tile, TileGenerator, TilesLayout};
use crate::scene::{Mesh, objects::TraceableObject, Scene};
use crate::scene::objects::wfobj;

mod img_tiles;
mod geometry;
mod scene;

const NUM_SLAVES: u32 = 8;

const FRAME_WIDTH: u32 = 640;
const FRAME_HEIGHT: u32 = 640;

const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = TILE_WIDTH;


struct TileMsg {
    tile: Tile,
    sent_from: u32,
    is_last: bool,
}


fn slv_thread_proc(slave_idx: u32, num_slaves: u32, tx: mpsc::Sender<TileMsg>, scene: Arc<Mesh>) {
    let layout = TilesLayout::new(FRAME_WIDTH, FRAME_HEIGHT, TILE_WIDTH, TILE_HEIGHT);

    let mut tiles = TileGenerator::new(slave_idx, num_slaves, &layout);
    for mut tile in &mut tiles {
        let aspect_ratio = (layout.frame_width as f32) / (layout.frame_height as f32);
        let fov_vert: f32 = 35.0;
        let fov_scaling_factor = (fov_vert / 2.0).to_radians().tan();
        
        // First scale from the viewport shape to NDC: [0; screen] -> [0; 2]
        let screen_to_world = Mat4f::from_rows(
            [2.0 * fov_scaling_factor * aspect_ratio / layout.frame_width as f32, 0.0, 0.0, -fov_scaling_factor * aspect_ratio],
            [0.0, 2.0 * fov_scaling_factor / layout.frame_height as f32, 0.0, -fov_scaling_factor],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        );

        for y in tile.col_idx * tile.height..tile.col_idx * tile.height + tile.height {
            for x in tile.row_idx * tile.width..tile.row_idx * tile.width + tile.width {
                let ray_aim = Point3d::from(&screen_to_world * Point4d::from_coords(x as f32, y as f32, -1.0, 1.0));
                let ray_orig = Point3d::from_coords(0.0, 0.0, 0.0);
                let ray_dir = ray_aim - ray_orig;

                let color = tracing::cast_ray(&ray_orig, &ray_dir.normalize(), &scene);

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

    let head_model = Arc::new(wfobj::new_wavefront_obj("models/african_head.obj").unwrap());
    let mut head_0 = scene::WfObj::new(Arc::clone(&head_model));
    let mut head_1 = scene::WfObj::new(Arc::clone(&head_model));
    head_0.scale(7.0, 7.0, 7.0);
    head_1.scale(7.0, 7.0, 7.0);
    head_0.rotate(0.0, 0.0, 0.0);
    head_1.rotate(0.0, 0.0, 0.0);
    head_0.translate(3.0, 0.0, -30.0);
    head_1.translate(-3.0, 0.0, -30.0);
    scene.add_obj(Box::new(head_0));
    scene.add_obj(Box::new(head_1));
    
    let cube_model = Arc::new(wfobj::new_wavefront_obj("models/cube.obj").unwrap());
    let mut cube_0 = scene::WfObj::new(Arc::clone(&cube_model));
    let mut cube_1 = scene::WfObj::new(Arc::clone(&cube_model));
    cube_0.scale(4.0, 4.0, 4.0);
    cube_1.scale(4.0, 4.0, 4.0);
    cube_0.rotate(45.0, 45.0, 0.0);
    cube_1.rotate(45.0, 45.0, 0.0);
    cube_0.translate(5.0, 0.0, -30.0);
    cube_1.translate(-5.0, 0.0, -30.0);
    //scene.add_obj(Box::new(cube_0));
    //scene.add_obj(Box::new(cube_1));
    
    let mut tri_0 = scene::TriObj::new(Triangle::new(
        Point3d::from_coords(-1.0, 1.0, 0.0),
        Point3d::from_coords(0.0, -1.0, 0.0),
        Point3d::from_coords(1.0, 0.8, 0.0),
    ));
    tri_0.scale(10.0, 10.0, 10.0);
    tri_0.rotate(-45.0, 0.0, 0.0);
    tri_0.translate(0.0, 0.0, -40.0);
    //scene.add_obj(Box::new(tri_0));
    
    //scene.add_light(Light::new(Point3d::from_coords(-50.0, -50.0, 50.0), 0.5));
    //scene.add_light(Light::new(Point3d::from_coords(10.0, 200.0, 20.0), 0.5));
    scene.add_light(Light::new(Point3d::from_coords(1.0, 0.0, 10.0), 0.5));

    scene
}

fn main() {
    let mut thread_handles = vec![];

    let (tx, rx) = mpsc::channel();

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
