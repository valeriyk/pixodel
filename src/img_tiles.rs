
pub mod img_tiles {
	//use image::{RgbImage, ImageBuffer};
	use crate::{TILE_WIDTH, TILE_HEIGHT};
	
	#[derive(Copy, Clone)]
	pub union TilePixel {
		pub rgba: [u8; 4],
		pub rgb: [u8; 3],
		pub luma_alpha: [u8; 2],
		pub luma: [u8; 1],
	}
	
	#[derive(Copy, Clone)]
	pub struct TilesLayout {
		frame_width: u32,
		frame_height: u32,
		default_tile_width: u32,
		default_tile_height: u32,
		fringe_tile_width: u32,
		fringe_tile_height: u32,
		pub num_tiles_in_row: u32,
		pub num_tiles_in_col: u32,
		has_narrow_tiles: bool,
	}
	
	impl TilesLayout {
		pub fn new(frame_width: u32, frame_height: u32, tile_width: u32, tile_height: u32) -> TilesLayout {
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
	
	pub struct Tile {
		pub row_idx: u32,
		pub col_idx: u32,
		pub width: u32,
		pub height: u32,
		//pub buf: RgbImage,
		pub bbuf: Box<[[TilePixel; TILE_WIDTH as usize]; TILE_HEIGHT as usize]>,
		pub vbuf: Vec<u8>,
	}
	
	impl Tile {
		pub fn new(row_idx: u32, col_idx: u32, width: u32, height: u32) -> Tile {
			Tile {
				row_idx,
				col_idx,
				width,
				height,
				//buf: ImageBuffer::from_pixel(width, height, image::Rgb([0, 0, 0])),
				bbuf: Box::new([[TilePixel {rgba: [u8::MIN, u8::MIN, u8::MIN, u8::MAX]}; TILE_WIDTH as usize]; TILE_HEIGHT as usize]),
				vbuf: Vec::new(),
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
				.field("row", &self.row_idx)
				.field("col", &self.col_idx)
				.field("w", &self.width)
				.field("h", &self.height)
				.finish()
		}
	}
	
	pub struct TileGenerator {
		stride: u32,
		seq_idx: u32,
		layout: TilesLayout,
	}
	
	impl TileGenerator {
		pub fn new(initial_idx: u32, stride: u32, layout: &TilesLayout) -> TileGenerator {
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
				//let buf = ImageBuffer::from_pixel(width, height, image::Rgb([0, 0, 0]));
				//let buf2: [u32; TILE_WIDTH as usize * TILE_HEIGHT as usize] = [0; TILE_WIDTH as usize * TILE_HEIGHT as usize];
				//let mut vbuf = Vec::new();
				// for i in 0..width*height*3 {
				// 	vbuf.push(0 as u8);
				// }
				let t = Tile {
					row_idx,
					col_idx,
					width,
					height,
					//buf,
					bbuf: Box::new([[TilePixel {rgba: [u8::MIN, u8::MIN, u8::MIN, u8::MAX]}; TILE_WIDTH as usize]; TILE_HEIGHT as usize]),
					vbuf: Vec::with_capacity((width * height) as usize),
				};
				self.seq_idx += self.stride;
				Some(t)
			} else {
				None
			}
		}
	}
}


#[cfg(test)]
mod tests {
	//use crate::img_tiles::img_tiles::*;
	use super::img_tiles::*;
	
	#[test]
	fn iter_2x2() {
		let layout = TilesLayout::new(800, 800, 400, 400);
		let mut tiles = TileGenerator::new(0, 1, &layout);
		assert_eq!(tiles.next(), Some(Tile::new(0, 0, 400, 400)));
		assert_eq!(tiles.next(), Some(Tile::new(1, 0, 400, 400)));
		assert_eq!(tiles.next(), Some(Tile::new(0, 1, 400, 400)));
		assert_eq!(tiles.next(), Some(Tile::new(1, 1, 400, 400)));
		assert_eq!(tiles.next(), None);
	}
	
	#[test]
	fn iter_4x4() {
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
	}
	
	#[test]
	fn iter_1x2() {
		let layout = TilesLayout::new(100, 100, 100, 50);
		let mut tiles = TileGenerator::new(0, 1, &layout);
		assert_eq!(tiles.next(), Some(Tile::new(0, 0, 100, 50)));
		assert_eq!(tiles.next(), Some(Tile::new(0, 1, 100, 50)));
		assert_eq!(tiles.next(), None);
	}
	
	#[test]
	fn iter_2x2_narrow() {
		let layout = TilesLayout::new(100, 100, 60, 60);
		let mut tiles = TileGenerator::new(0, 1, &layout);
		assert_eq!(tiles.next(), Some(Tile::new(0, 0, 60, 60)));
		assert_eq!(tiles.next(), Some(Tile::new(1, 0, 40, 60)));
		assert_eq!(tiles.next(), Some(Tile::new(0, 1, 60, 40)));
		assert_eq!(tiles.next(), Some(Tile::new(1, 1, 40, 40)));
		assert_eq!(tiles.next(), None);
	}
	
	#[test]
	fn tile_larger_than_frame() {
		let layout = TilesLayout::new(100, 100, 150, 150);
		let mut tiles = TileGenerator::new(0, 1, &layout);
		assert_eq!(tiles.next(), Some(Tile::new(0, 0, 100, 100)));
		assert_eq!(tiles.next(), None);
	}
	
	#[test]
	fn no_tiles_for_a_slave() {
		let layout = TilesLayout::new(100, 100, 100, 100);
		let mut tiles = TileGenerator::new(0, 1, &layout);
		assert_eq!(tiles.next(), Some(Tile::new(0, 0, 100, 100)));
		assert_eq!(tiles.next(), None);
		let mut tiles = TileGenerator::new(1, 1, &layout);
		assert_eq!(tiles.next(), None);
	}
}
