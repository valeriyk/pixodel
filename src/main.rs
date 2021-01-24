extern crate image;

use std::process::Output;
use std::sync::{Arc, mpsc};
use std::thread;

use image::{GenericImage, ImageBuffer};

use crate::img_tiles::img_tiles::{Tile, TileGenerator, TilesLayout};
use crate::math::math::Vec3f;
mod img_tiles;
mod math;

struct TileMsg {
    tile: Tile,
    sent_from: u32,
    is_last: bool,
}

//~ enum TileMsgType {
//~ REGULAR,
//~ LAST,
//~ }

const NUM_SLAVES: u32 = 1;

const FRAME_WIDTH: u32 = 800;
const FRAME_HEIGHT: u32 = FRAME_WIDTH;

const TILE_WIDTH: u32 = 100;
const TILE_HEIGHT: u32 = TILE_WIDTH;

struct Sphere {
    center: Vec3f,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32) -> Sphere {
        Sphere {center, radius}
    }
    
    pub fn ray_intersect(&self, ray_origin: Vec3f, ray_dir: Vec3f) -> (bool, f32) {
        let l = self.center - ray_origin;
        let tca = l * ray_dir;
        let d_squared = l * l - tca * tca;
        if d_squared > (self.radius * self.radius) {
            return (false, f32::MAX)
        }
        let thc = (self.radius * self.radius - d_squared).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 >= 0.0 {
            (true, t0)
        } else if t0 < 0.0 && t1 >= 0.0 {
            (true, t1)
        } else {
            (false, f32::MAX)
        }
    }
}

fn cast_ray (ray_orig: Vec3f, ray_dir: Vec3f, scene: &Scene) -> u8 {
    let mut distance_to_nearest_sphere = f32::MAX;
    let mut color = 30u8;
    
    for s in &scene.spheres {
        let (does_intersect, distance_to_sphere) = s.ray_intersect(ray_orig, ray_dir);
        if does_intersect == true && distance_to_sphere < distance_to_nearest_sphere {
            distance_to_nearest_sphere = distance_to_sphere;
            for l in &scene.lights {
                let surface_pt: Vec3f = ray_dir * distance_to_sphere;
            
                let norm_to_surface: Vec3f = (surface_pt - s.center).normalize();
                let light_dir: Vec3f = (surface_pt - l.position).normalize();
                let mut intensity = (light_dir * norm_to_surface);
                if intensity < 0.0 {
                    intensity = 0.0;
                }
                assert!(intensity <= 1.0);
                color = (intensity * u8::MAX as f32) as u8;
            }
        }
    }
    color
}

struct Light {
    position: Vec3f,
    intensity: f32,
}

struct Scene {
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
}

fn slv_thread_proc(slave_idx: u32, num_slaves: u32, tx: mpsc::Sender<TileMsg>, scene: Arc<Scene>) {
    let layout = TilesLayout::new(FRAME_WIDTH, FRAME_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
    
    let mut tiles = TileGenerator::new(slave_idx, num_slaves, &layout);
    for mut tile in &mut tiles {
        // let num_pixels: usize = (tile.width * tile.height) as usize;
        // for _ in 0..num_pixels {
        
        for y in tile.col_idx*tile.height..tile.col_idx*tile.height+tile.height {
            for x in tile.row_idx*tile.width..tile.row_idx*tile.width+tile.width {
            
                let ray_x: f32 = (x as f32 * 2.0) / layout.frame_width as f32 - 1.0;
                let ray_y: f32 = (y as f32 * 2.0) / layout.frame_height as f32 - 1.0;
                let ray_z = -1.0;
                let ray_dir = Vec3f::new(ray_x, ray_y, ray_z);
                let color = cast_ray(Vec3f::new(0.0, 0.0, 0.0), ray_dir.normalize(), &scene);
                
                tile.vbuf.push(color);
                tile.vbuf.push(u8::MIN);
                tile.vbuf.push(u8::MIN);
                
                // tile.vbuf.push((tile.row_idx * (u8::MAX as u32 + 1) / &layout.num_tiles_in_row) as u8);
                // tile.vbuf.push(u8::MIN);
                // tile.vbuf.push((tile.col_idx * (u8::MAX as u32 + 1) / &layout.num_tiles_in_col) as u8);
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
            
            let tile_img = ImageBuffer::from_raw(received.tile.width, received.tile.height, received.tile.vbuf).unwrap();
            img.copy_from(&tile_img, subimage_top_left_x, subimage_top_left_y).unwrap();
        }

        if num_slaves_finished == NUM_SLAVES {
            img.save("myimg.png").unwrap();
            break;
        }
    }
}

fn main() {
    let mut thread_handles = vec![];

    let (tx, rx) = mpsc::channel();
    
    //let (tx2, rx2) = mpsc::channel();
    let mut scene = Scene {spheres: Vec::new(), lights: Vec::new()};
    
    let sphere = Sphere {
        center: Vec3f::new(0.0, 0.0, -2.0),
        radius: 1.0,
    };
    scene.spheres.push(sphere);
    
    let sphere = Sphere {
        center: Vec3f::new(-2.0, 0.0, -4.0),
        radius: 1.0,
    };
    scene.spheres.push(sphere);
    
    let light = Light {
        position: Vec3f::new(-5.0, -5.0, 0.0),
        intensity: 1.0,
    };
    scene.lights.push(light);
    
    let scene_glob = Arc::new(scene);
    
    for i in 0..NUM_SLAVES {
        let tx_slv_to_fbuf = mpsc::Sender::clone(&tx);
        let scene = scene_glob.clone();
        let handle = thread::spawn(move || slv_thread_proc(i, NUM_SLAVES, tx_slv_to_fbuf, scene));

        thread_handles.push(handle);
    }

    let handle = thread::spawn(|| fbuf_thread_proc(rx));
    thread_handles.push(handle);

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
