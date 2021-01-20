extern crate image;

use image::{GenericImage, ImageBuffer, RgbImage};
use std::sync::mpsc;
use std::thread;

#[derive(Copy, Clone)]
struct TilesLayout {
    frame_width: u32,
    frame_height: u32,
    default_tile_width: u32,
    default_tile_height: u32,
    fringe_tile_width: u32,
    fringe_tile_height: u32,
    num_tiles_in_row: u32,
    num_tiles_in_col: u32,
    has_narrow_tiles: bool,
}

impl TilesLayout {
    fn new(frame_width: u32, frame_height: u32, tile_width: u32, tile_height: u32) -> TilesLayout {
        let num_tiles_in_row = ((frame_width as f32 / tile_width as f32).ceil()) as u32;
        let num_tiles_in_col = ((frame_height as f32 / tile_height as f32).ceil()) as u32;
        let fringe_tile_width = frame_width - (num_tiles_in_row - 1) * tile_width;
        let fringe_tile_height = frame_height - (num_tiles_in_col - 1) * tile_height;
        let has_narrow_tiles =
            (fringe_tile_width != tile_width) || (fringe_tile_height != tile_height);
        TilesLayout {
            frame_width,
            frame_height,
            default_tile_width: tile_width,
            default_tile_height: tile_height,
            fringe_tile_width,
            fringe_tile_height,
            num_tiles_in_row,
            num_tiles_in_col,
            has_narrow_tiles,
        }
    }
}

struct Tile {
    row_idx: u32,
    col_idx: u32,
    width: u32,
    height: u32,
    buf: RgbImage,
    //buf2: [u32; TILE_WIDTH as usize * TILE_HEIGHT as usize],
}

impl Tile {
    fn new(row_idx: u32, col_idx: u32, width: u32, height: u32) -> Tile {
        Tile {
            row_idx,
            col_idx,
            width,
            height,
            buf: ImageBuffer::from_pixel(width, height, image::Rgb([0, 0, 0])),
        }
    }
}

impl core::cmp::PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        let mut res: bool = self.row_idx == other.row_idx;
        res &= self.col_idx == other.col_idx;
        res &= self.width == other.width;
        res &= self.height == other.height;
        res
    }
}

impl core::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tile")
            .field("row",&self.row_idx)
            .field("col", &self.col_idx)
            .field("w", &self.width)
            .field("h", &self.height)
            .finish()
    }
}

struct TileGenerator {
    stride: u32,
    seq_idx: u32,
    layout: TilesLayout,
}

impl TileGenerator {
    fn new(initial_idx: u32, stride: u32, layout: &TilesLayout) -> TileGenerator {
        TileGenerator {
            seq_idx: initial_idx,
            stride,
            layout: *layout,
        }
    }
}

impl Iterator for TileGenerator {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.seq_idx < self.layout.num_tiles_in_row * self.layout.num_tiles_in_col {
            let row_idx = self.seq_idx % self.layout.num_tiles_in_row;
            let col_idx = self.seq_idx / self.layout.num_tiles_in_row;
            let mut width = self.layout.default_tile_width;
            let mut height = self.layout.default_tile_height;
            if (row_idx == self.layout.num_tiles_in_row - 1) && self.layout.has_narrow_tiles {
                width = self.layout.fringe_tile_width;
            }
            if (col_idx == self.layout.num_tiles_in_col - 1) && self.layout.has_narrow_tiles {
                height = self.layout.fringe_tile_height;
            }
            let buf = ImageBuffer::from_pixel(width, height, image::Rgb([0, 0, 0]));
            //let buf2: [u32; TILE_WIDTH as usize * TILE_HEIGHT as usize] = [0; TILE_WIDTH as usize * TILE_HEIGHT as usize];
            let t = Tile {
                row_idx,
                col_idx,
                width,
                height,
                buf,
            };
            self.seq_idx += self.stride;
            Some(t)
        } else {
            None
        }
    }
}


#[test]
fn iter_demo() {
    let layout = TilesLayout::new(800, 800, 400, 400);
    let mut tiles = TileGenerator::new(0, 1, &layout);
    assert_eq!(tiles.next(), Some(Tile::new(0, 0, 400, 400)));
    assert_eq!(tiles.next(), Some(Tile::new(1, 0, 400, 400)));
    assert_eq!(tiles.next(), Some(Tile::new(0, 1, 400, 400)));
    assert_eq!(tiles.next(), Some(Tile::new(1, 1, 400, 400)));
    assert_eq!(tiles.next(), None);

    let layout = TilesLayout::new(800, 800, 200, 200);
    let mut tiles = TileGenerator::new(0, 2, &layout);
    assert_eq!(tiles.next(), Some(Tile::new(0, 0, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(2, 0, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(0, 1, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(2, 1, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(0, 2, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(2, 2, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(0, 3, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(2, 3, 200, 200)));
    assert_eq!(tiles.next(), None);
    let mut tiles = TileGenerator::new(1, 2, &layout);
    assert_eq!(tiles.next(), Some(Tile::new(1, 0, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(3, 0, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(1, 1, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(3, 1, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(1, 2, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(3, 2, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(1, 3, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(3, 3, 200, 200)));
    assert_eq!(tiles.next(), None);
    let mut tiles = TileGenerator::new(0, 3, &layout);
    assert_eq!(tiles.next(), Some(Tile::new(0, 0, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(3, 0, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(2, 1, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(1, 2, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(0, 3, 200, 200)));
    assert_eq!(tiles.next(), Some(Tile::new(3, 3, 200, 200)));
    assert_eq!(tiles.next(), None);
    
    let layout = TilesLayout::new(100, 100, 100, 50);
    let mut tiles = TileGenerator::new(0, 1, &layout);
    assert_eq!(tiles.next(), Some(Tile::new(0, 0, 100, 50)));
    assert_eq!(tiles.next(), Some(Tile::new(0, 1, 100, 50)));
    assert_eq!(tiles.next(), None);
    
    let layout = TilesLayout::new(100, 100, 60, 60);
    let mut tiles = TileGenerator::new(0, 1, &layout);
    assert_eq!(tiles.next(), Some(Tile::new(0, 0, 60, 60)));
    assert_eq!(tiles.next(), Some(Tile::new(1, 0, 40, 60)));
    assert_eq!(tiles.next(), Some(Tile::new(0, 1, 60, 40)));
    assert_eq!(tiles.next(), Some(Tile::new(1, 1, 40, 40)));
    assert_eq!(tiles.next(), None);
    
    let layout = TilesLayout::new(100, 100, 150, 150);
    let mut tiles = TileGenerator::new(0, 1, &layout);
    assert_eq!(tiles.next(), Some(Tile::new(0, 0, 100, 100)));
    assert_eq!(tiles.next(), None);
}


struct TileMsg {
    tile: Tile,
    sent_from: u32,
    is_last: bool,
}

//~ enum TileMsgType {
//~ REGULAR,
//~ LAST,
//~ }

const NUM_SLAVES: u32 = 22;

const FRAME_WIDTH: u32 = 800;
const FRAME_HEIGHT: u32 = FRAME_WIDTH;

const TILE_WIDTH: u32 = 10;
const TILE_HEIGHT: u32 = TILE_WIDTH;

fn slv_thread_proc(slave_idx: u32, num_slaves: u32, tx: mpsc::Sender<TileMsg>) {
    let layout = TilesLayout::new(FRAME_WIDTH, FRAME_HEIGHT, TILE_WIDTH, TILE_HEIGHT);

    let mut tiles = TileGenerator::new(slave_idx, num_slaves, &layout);
    for mut tile in &mut tiles {
        tile.buf = ImageBuffer::from_pixel(
            tile.width,
            tile.height,
            image::Rgb([
                (tile.row_idx * 256 / &layout.num_tiles_in_row) as u8,
                0,
                (tile.col_idx * 256 / &layout.num_tiles_in_col) as u8,
            ]),
        );
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

    let mut img: RgbImage =
        ImageBuffer::from_pixel(FRAME_WIDTH, FRAME_HEIGHT, image::Rgb([100, 80, 60]));

    for received in &rx {
        if received.is_last == true {
            num_slaves_finished += 1;
        } else {
            let subimage_top_left_x = received.tile.row_idx * received.tile.width;
            let subimage_top_left_y = received.tile.col_idx * received.tile.height;
            img.copy_from(&received.tile.buf, subimage_top_left_x, subimage_top_left_y);
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
