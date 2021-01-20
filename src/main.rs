extern crate image;

use image::{RgbImage, ImageBuffer, GenericImage};
use std::sync::mpsc;
use std::thread;

//type Tile = [[u16; 32]; 32];

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
		let fringe_tile_width = frame_width % tile_width;
		let fringe_tile_height = frame_height % tile_height;
		let has_narrow_tiles = (tile_width != fringe_tile_width) || (tile_height != fringe_tile_height);
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
	fn new() -> Tile {
		Tile {
			row_idx: 0,
			col_idx: 0,
			width: 0,
			height: 0,
			buf: ImageBuffer::from_pixel(1, 1, image::Rgb([100, 100, 100])),
		}
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
			//~ if (row_idx == self.layout.num_tiles_in_row - 1) && self.layout.has_narrow_tiles {
				//~ width = self.layout.fringe_tile_width;
			//~ }
			//~ if (col_idx == self.layout.num_tiles_in_col - 1) && self.layout.has_narrow_tiles {
				//~ height = self.layout.fringe_tile_height;
			//~ }
			let buf = ImageBuffer::from_pixel(width, height, image::Rgb([100, 100, 100]));
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
	let layout_0 = TilesLayout::new(800, 800, 400, 400);
	let mut tiles = TileGenerator::new(0, 1, layout_0);
	//let mut tiles_iter = tiles.iter();
	assert_eq!(tiles.next(), Some((0, 0)));
	assert_eq!(tiles.next(), Some((1, 0)));
	assert_eq!(tiles.next(), Some((0, 1)));
	assert_eq!(tiles.next(), Some((1, 1)));
	assert_eq!(tiles.next(), None);
	
	let layout_1 = TilesLayout::new(800, 800, 200, 200);
	let mut tiles = TileGenerator::new(0, 2, layout_1);
	assert_eq!(tiles.next(), Some((0, 0)));
	assert_eq!(tiles.next(), Some((2, 0)));
	assert_eq!(tiles.next(), Some((0, 1)));
	assert_eq!(tiles.next(), Some((2, 1)));
	assert_eq!(tiles.next(), Some((0, 2)));
	assert_eq!(tiles.next(), Some((2, 2)));
	assert_eq!(tiles.next(), Some((0, 3)));
	assert_eq!(tiles.next(), Some((2, 3)));
	assert_eq!(tiles.next(), None);
	let mut tiles = TileGenerator::new(1, 2, layout_1);
	assert_eq!(tiles.next(), Some((1, 0)));
	assert_eq!(tiles.next(), Some((3, 0)));
	assert_eq!(tiles.next(), Some((1, 1)));
	assert_eq!(tiles.next(), Some((3, 1)));
	assert_eq!(tiles.next(), Some((1, 2)));
	assert_eq!(tiles.next(), Some((3, 2)));
	assert_eq!(tiles.next(), Some((1, 3)));
	assert_eq!(tiles.next(), Some((3, 3)));
	assert_eq!(tiles.next(), None);
	let mut tiles = TileGenerator::new(0, 3, layout_1);
	assert_eq!(tiles.next(), Some((0, 0)));
	assert_eq!(tiles.next(), Some((3, 0)));
	assert_eq!(tiles.next(), Some((2, 1)));
	assert_eq!(tiles.next(), Some((1, 2)));
	assert_eq!(tiles.next(), Some((0, 3)));
	assert_eq!(tiles.next(), Some((3, 3)));
	assert_eq!(tiles.next(), None);
}



struct TileMsg {
	tile: Tile,
	sent_from: u32,
	description: String,
	is_last: bool,
}

//~ enum TileMsgType {
	//~ REGULAR,
	//~ LAST,
//~ }

//~ struct Framebuffer {
	//~ framebuffer: Box<u32>,
//~ }

//~ impl Framebuffer {
	//~ fn get_tile (idx: usize) -> Tile {
		//~ [[0; 32]; 32]
	//~ }
//~ }

const NUM_SLAVES: u32 = 2;

const FRAME_WIDTH: u32 = 800;
const FRAME_HEIGHT: u32 = FRAME_WIDTH;

const TILE_WIDTH: u32 = 100;
const TILE_HEIGHT: u32 = TILE_WIDTH;

//const NUM_TILES: u32 = (FRAME_WIDTH / TILE_WIDTH) * (FRAME_HEIGHT / TILE_HEIGHT);



/*
//fn mst_thread_proc (run: mpsc::Receiver<bool>, tx: mpsc::Sender<String>) {
fn mst_thread_proc (tx: &mut [mpsc::SyncSender<String>]) {
	//for i in tx.iter() {
	for i in 0..NUM_SLAVES {
		//~ println!("hi number {} from the spawned thread!", i);
		//~ thread::sleep(Duration::from_millis(1));
		
		
		let mut val = String::from("hi from master to ");
		val.push_str(&i.to_string());
		tx[i].send(val).unwrap();
        println!("Mst just sent smth to {}", i);
	}
}
*/

fn slv_thread_proc (slave_idx: u32, num_slaves: u32, tx: mpsc::Sender<TileMsg>) {
	
	let layout = TilesLayout::new(FRAME_WIDTH, FRAME_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
	
	let mut tiles = TileGenerator::new(slave_idx, num_slaves, &layout);
	for mut tile in &mut tiles {
		//println!("tile iter of slave {} returned row{}(width={}):col{}(height={})", slave_idx, tile.row_idx, tile.width, tile.col_idx, tile.height);
		let mut val = String::from("tile row");
		val.push_str(&tile.row_idx.to_string());
		val.push_str(":col");
		val.push_str(&tile.col_idx.to_string());
		val.push_str(" from slave ");
		val.push_str(&slave_idx.to_string());
        
        tile.buf = ImageBuffer::from_pixel(tile.width, tile.height, image::Rgb([(tile.row_idx * 256 / &layout.num_tiles_in_row) as u8, 0, (tile.col_idx * 256 / &layout.num_tiles_in_col) as u8]));
        let msg = TileMsg {
			description: val,
			tile,
			sent_from: slave_idx,
			is_last: false,
		};
        tx.send(msg).unwrap();
        //println!("Slv {}: tile {}", slave_idx, tile_idx);
	}
	
	let mut val = String::from("Last msg from slave ");
	val.push_str(&slave_idx.to_string());
	let t = Tile::new();
	let msg = TileMsg {
		description: val,
		tile: t,
		sent_from: slave_idx,
		is_last: true,
	};
	tx.send(msg).unwrap();
	println!("Slv {}: finishing", slave_idx);
        
}

fn fbuf_thread_proc (rx: mpsc::Receiver<TileMsg>) {

	let mut num_slaves_finished: u32 = 0;
	
	let mut img: RgbImage = ImageBuffer::from_pixel(FRAME_WIDTH, FRAME_HEIGHT, image::Rgb([100, 80, 60]));
	
	for received in &rx {
	
		if received.is_last == true {
			num_slaves_finished += 1;
			println!("Last msg detected, num_slaves_finished = {}", num_slaves_finished);
		} else {
			println!("Fbuf: received {}", received.description);
			let subimage_top_left_x = received.tile.row_idx * received.tile.width;
			let subimage_top_left_y = received.tile.col_idx * received.tile.height;
			img.copy_from(&received.tile.buf, subimage_top_left_x, subimage_top_left_y);
		}
		
		if num_slaves_finished == NUM_SLAVES {
			println!("Fbuf done!");
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
		let handle = thread::spawn(move || {slv_thread_proc(i, NUM_SLAVES, tx_slv_to_fbuf)});
		
		thread_handles.push(handle);
	}
	
	let handle = thread::spawn(|| {fbuf_thread_proc(rx)});
	thread_handles.push(handle);
	 
    for handle in thread_handles {
        handle.join().unwrap();
    }
}



/*
type Handler = fn() -> bool;

struct InPort {
	ch : Receiver,
	handler : Handler,
};

struct OutPort {
	ch : Sender,
};

struct Core {
	name : String,
	//~ ports_num : usize,
	//~ ports_used : usize,
	//~ //port_arr
	inputs_arr : Vec<InPort>,
	outputs_arr : Vec<OutPort>,
	//event
	mutex : thread::mutex;
	//thread
};

impl Core {
	fn new (name: String) -> Core {
		Core {
			.name,
			.ports_num (0),
			.ports_used (0),
			.inputs_vec (Vec::new()),
			.outputs_vec (Vec::new()),
		}
	}
	
	fn connect_slv_to(name: String, h: Handler, mst : Module) {
		inputs_arr.add();
		let (tx, rx) = mpsc::channel();
		hw.outputs_arr.add();
	}
	
	fn run() {
		foreach i in inputs_arr {
			thread::spawn(i.handler);
        }
	}
}

fn mem_ctrl_proc () -> bool {
	true
}

fn main() {
    println!("Hello, world!");
    
    let cpu_core0 = new_module( String::from("CPU0"));
	let cpu_core1 = new_module( String::from("CPU1"));
	let mem_ctrl  = new_module( String::from("MEM_CTRL"));
    
    cpu_core0.connect_slv_to(String::from("CORE0_RUN_REQ"), core0_run_proc, run_ctrl);
    cpu_core1.connect_slv_to(String::from("CORE1_RUN_REQ"), core1_run_proc, run_ctrl);
    cpu_core1.connect_slv_to(String::from("CORE0_TO_CORE1"), core1_irq_proc, cpu_core0);
    mem_ctrl.connect_slv_to(String::from("MEM_BUS"), mem_ctrl_proc, cpu_core0, cpu_core1);
    //~ new_link( cpu_core0, mem_ctrl, String::from("CORE0_TO_MEM_BUS"), core0_bus);
    //~ new_link( cpu_core1, mem_ctrl, String::from("CORE1_TO_MEM_BUS"), core1_bus);
    //~ //new_link( cpu_core0, cpu_core1, signal_t, "MY_IRQ");
    
		
	};
    cpu_core0.run();
    cpu_core1.run();
    mem_ctrl.run();
//    mem_ctrl.powerup();
}



//~ use std::sync::mpsc;

//~ fn main() {
    //~ let (tx, rx) = mpsc::channel();
//~ }


//~ fn cpu_core() {
    //~ let (tx, rx) = mpsc::channel();
//~ }

//~ fn mem_ctrl() {
    //~ let (tx, rx) = mpsc::channel();
//~ }

//~ thread::spawn(|| {
            //~ handle_connection(stream);
        //~ });

*/



