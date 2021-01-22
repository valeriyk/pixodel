extern crate image;

mod img_tiles;

use crate::img_tiles::img_tiles::{Tile, TilesLayout, TileGenerator, TilePixel};
use image::{GenericImage, ImageBuffer, RgbImage, RgbaImage};
use std::sync::mpsc;
use std::thread;





struct TileMsg {
    tile: Tile,
    sent_from: u32,
    is_last: bool,
}

//~ enum TileMsgType {
//~ REGULAR,
//~ LAST,
//~ }

const NUM_SLAVES: u32 = 2;

const FRAME_WIDTH: u32 = 80;
const FRAME_HEIGHT: u32 = FRAME_WIDTH;

const TILE_WIDTH: u32 = 8;
const TILE_HEIGHT: u32 = TILE_WIDTH;

fn slv_thread_proc(slave_idx: u32, num_slaves: u32, tx: mpsc::Sender<TileMsg>) {
    let layout = TilesLayout::new(FRAME_WIDTH, FRAME_HEIGHT, TILE_WIDTH, TILE_HEIGHT);

    let mut tiles = TileGenerator::new(slave_idx, num_slaves, &layout);
    for mut tile in &mut tiles {
        // tile.buf = ImageBuffer::from_pixel(
        //     tile.width,
        //     tile.height,
        //     image::Rgb([
        //         (tile.row_idx * (u8::MAX as u32 + 1) / &layout.num_tiles_in_row) as u8,
        //         0,
        //         (tile.col_idx * (u8::MAX as u32 + 1) / &layout.num_tiles_in_col) as u8,
        //     ]),
        // );
        let num_pixels: usize = (tile.width * tile.height) as usize;
        // for i in 0..num_pixels {
        //     tile.bbuf[i*3] = (tile.row_idx * (u8::MAX as u32 + 1) / &layout.num_tiles_in_row) as u8;
        //     tile.bbuf[i*3 + 1] = 0 as u8;
        //     tile.bbuf[i*3 + 2] = (tile.col_idx * (u8::MAX as u32 + 1) / &layout.num_tiles_in_col) as u8;
        // }
        for y in 0..tile.height as usize {
            for x in 0..tile.width as usize {
                tile.bbuf[x][y] = TilePixel {
                    rgba: [
                        (tile.row_idx * (u8::MAX as u32 + 1) / &layout.num_tiles_in_row) as u8,
                        u8::MIN,
                        (tile.col_idx * (u8::MAX as u32 + 1) / &layout.num_tiles_in_col) as u8,
                        u8::MAX],
                };
            }
        }
        for i in 0..num_pixels {
            tile.vbuf.push((tile.row_idx * (u8::MAX as u32 + 1) / &layout.num_tiles_in_row) as u8);
            tile.vbuf.push(u8::MIN);
            tile.vbuf.push((tile.col_idx * (u8::MAX as u32 + 1) / &layout.num_tiles_in_col) as u8);
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
        ImageBuffer::from_pixel(FRAME_WIDTH, FRAME_HEIGHT, image::Rgba([100u8, 80u8, 60u8, u8::MAX]));
    
    let mut img2 =
        ImageBuffer::from_pixel(FRAME_WIDTH, FRAME_HEIGHT, image::Rgb([100u8, 80u8, 60u8]));
    
    for received in &rx {
        if received.is_last == true {
            num_slaves_finished += 1;
        } else {
            let subimage_top_left_x = received.tile.row_idx * received.tile.width;
            let subimage_top_left_y = received.tile.col_idx * received.tile.height;
            //img.copy_from(&received.tile.buf, subimage_top_left_x, subimage_top_left_y);
            // let sliced_box: &[u8] = &received.tile.bbuf[..];
            // let tile_img = ImageBuffer::from_raw(received.tile.width, received.tile.height, sliced_box).unwrap();
            // img.copy_from(&tile_img, subimage_top_left_x, subimage_top_left_y);
            
            for y in 0..received.tile.height as usize {
                for x in 0..received.tile.width as usize {
                    img.put_pixel(subimage_top_left_x + x as u32, subimage_top_left_y + y as u32, unsafe { image::Rgba(received.tile.bbuf[x][y].rgba)})
                    //img.put_pixel(subimage_top_left_x + x as u32, subimage_top_left_y + y as u32, unsafe { image::Rgba([100u8, 0u8, 0u8, 0u8])})
                }
            }
    
            let tile_img: RgbImage = ImageBuffer::from_raw(received.tile.width, received.tile.height, received.tile.vbuf).unwrap();
            img2.copy_from(&tile_img, subimage_top_left_x, subimage_top_left_y);
            
        }

        if num_slaves_finished == NUM_SLAVES {
            img.save("myimg.png").unwrap();
            img2.save("myimg2.png").unwrap();
            break;
        }
    }
}

fn main() {
    let mut thread_handles = vec![];

    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_SLAVES {
        let tx_slv_to_fbuf = mpsc::Sender::clone(&tx);
        let handle = thread::spawn(move || slv_thread_proc(i, NUM_SLAVES, tx_slv_to_fbuf));

        thread_handles.push(handle);
    }

    let handle = thread::spawn(|| fbuf_thread_proc(rx));
    thread_handles.push(handle);

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
