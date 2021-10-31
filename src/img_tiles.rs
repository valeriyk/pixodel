pub trait PixelTrait: Copy + Clone + PartialEq + core::fmt::Debug {
	fn zero() -> Self;
	fn set_rgb(r: u8, g: u8, b: u8) -> Self;
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct Rgb {
	pub(crate) pixel: [u8; 3],
}
impl PixelTrait for Rgb {
	fn zero() -> Rgb {
		Rgb {
			pixel: [0, 0, 0],
		}
	}
	fn set_rgb(r: u8, g: u8, b: u8) -> Self {
		Rgb {
			pixel: [r, g, b],
		}
	}
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Rgba {
	pixel: [u8; 4],
}
impl PixelTrait for Rgba {
	fn zero() -> Rgba {
		Rgba {
			pixel: [0, 0, 0, 0],
		}
	}
	
	fn set_rgb(r: u8, g: u8, b: u8) -> Self {
		Rgba {
			pixel: [r, g, b, 255],
		}
	}
}



pub struct TiledFrame<T: PixelTrait> {
    width: u32,
    height: u32,
    default_tile_width: u32,
    default_tile_height: u32,
    fringe_tile_width: u32,
    fringe_tile_height: u32,
    num_tiles_in_row: u32,
    num_tiles_in_col: u32,
    has_narrow_tiles: bool,
    pub(crate) pixels: Vec<T>,
}

impl<T: PixelTrait> TiledFrame<T> {
	pub fn new(
		frame_width: u32,
		frame_height: u32,
		tile_width: u32,
		tile_height: u32,
	) -> TiledFrame<T> {
		let num_tiles_in_row = ((frame_width as f32 / tile_width as f32).ceil()) as u32;
		let num_tiles_in_col = ((frame_height as f32 / tile_height as f32).ceil()) as u32;
		let fringe_tile_width = frame_width - (num_tiles_in_row - 1) * tile_width;
		let fringe_tile_height = frame_height - (num_tiles_in_col - 1) * tile_height;
		let has_narrow_tiles =
			(fringe_tile_width != tile_width) || (fringe_tile_height != tile_height);
		TiledFrame {
			width: frame_width,
			height: frame_height,
			default_tile_width: tile_width,
			default_tile_height: tile_height,
			fringe_tile_width,
			fringe_tile_height,
			num_tiles_in_row,
			num_tiles_in_col,
			has_narrow_tiles,
			//pixels: Vec::<T>::with_capacity((frame_width * frame_height) as usize),
			pixels: vec![T::zero(); (frame_width * frame_height) as usize],
		}
	}
	
	pub fn iter(&self) -> TiledFrameIter<T> {
		TiledFrameIter::<T>::new(self)
	}

	fn detach_tile(&self, tile_idx_hor: usize, tile_idx_vert: usize) -> Tile<T> {
		let mut width = self.default_tile_width;
		let mut height = self.default_tile_height;
		let origin_x = tile_idx_hor as u32 * self.default_tile_width;
		let origin_y = tile_idx_vert as u32 * self.default_tile_height;
		if (tile_idx_hor as u32 == self.num_tiles_in_row - 1) && self.has_narrow_tiles {
			width = self.fringe_tile_width;
		}
		if (tile_idx_vert as u32 == self.num_tiles_in_col - 1) && self.has_narrow_tiles {
			height = self.fringe_tile_height;
		}
		let mut tile = Tile::<T>::new(origin_x, origin_y, width, height);
		for y in origin_y..origin_y + height {
			for x in origin_x..origin_x + width {
				let tile_idx = (y - origin_y) * tile.width + (x - origin_x);
				let frame_idx = (y * self.width + x);
				tile.pixels[tile_idx as usize] = self.pixels[frame_idx as usize];
			}
		}
		tile
	}
	
	pub(crate) fn merge_tile(&mut self, tile: &Tile<T>) {
        for y in tile.origin_y..tile.origin_y + tile.height {
            for x in tile.origin_x..tile.origin_x + tile.width {
				let tile_idx = (y - tile.origin_y) * tile.width + (x - tile.origin_x);
				let frame_idx = (y * self.width + x);
                self.pixels[frame_idx as usize] = tile.pixels[tile_idx as usize];
            }
        }
    }
}

impl<T: PixelTrait> core::cmp::PartialEq for Tile<T> {
    fn eq(&self, other: &Self) -> bool {
		
		let mut res: bool = self.origin_x == other.origin_x;
        res &= self.origin_y == other.origin_y;
        res &= self.width == other.width;
        res &= self.height == other.height;
	
		if res {
			for y in 0..self.height {
				for x in 0..self.width {
					let idx: usize = (y * self.width + x) as usize;
					res &= self.pixels[idx] == other.pixels[idx]
				}
			}
		}
		res
    }
}

impl<T: PixelTrait> core::fmt::Debug for Tile<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tile")
            .field("origin_x", &self.origin_x)
            .field("origin_y", &self.origin_y)
            .field("w", &self.width)
            .field("h", &self.height)
			.field("pixels", &self.pixels)
            .finish()
    }
}



pub struct Tile<T: PixelTrait> {
    origin_x: u32,
    origin_y: u32,
    width: u32,
    height: u32,
    pixels: Vec<T>,
}

impl<T: PixelTrait> Tile<T> {
    fn new(origin_x: u32, origin_y: u32, width: u32, height: u32) -> Tile<T> {
        Tile {
            origin_x,
            origin_y,
            width,
            height,
            //pixels: Vec::with_capacity((width * height) as usize),
			pixels: vec![T::zero(); (width * height) as usize],
        }
    }
	
	pub fn iter_mut(&mut self) -> TileIter<T> {
		TileIter::<T>::new(self)
	}
}

pub struct TileIter<'a, T: PixelTrait> {
	tile: &'a Tile<T>,
	pixel_idx_hor: usize,
	pixel_idx_vert: usize,
}

impl<'a, T: PixelTrait> TileIter<'a, T> {
	fn new(tile: &'a Tile<T>) -> TileIter<T> {
		TileIter {
			tile,
			pixel_idx_hor: 0,
			pixel_idx_vert: 0,
		}
	}
}

impl <'a, T: PixelTrait> Iterator for TileIter<'a, T> {
	type Item = (usize, usize, T);
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.pixel_idx_vert < self.tile.height as usize &&
			self.pixel_idx_hor < self.tile.width as usize
		{
			let pixel_idx = self.pixel_idx_vert * self.tile.width as usize + self.pixel_idx_vert;
			let pixel = self.tile.pixels[pixel_idx as usize];
			let result = (self.pixel_idx_hor, self.pixel_idx_vert, pixel);
			if self.pixel_idx_hor == (self.tile.width - 1) as usize {
				self.pixel_idx_hor = 0;
				self.pixel_idx_vert += 1;
			}
			else {
				self.pixel_idx_hor += 1;
			}
			Some(result)
		}
		else {
			None
		}
	}
}
///////////////////////////////////////////////////////////////

pub struct TiledFrameIter<'a, T: PixelTrait> {
    frame: &'a TiledFrame<T>,
    tile_idx_hor: usize,
	tile_idx_vert: usize,
}

impl<'a, T: PixelTrait> TiledFrameIter<'a, T> {
	fn new(frame: &'a TiledFrame<T>) -> TiledFrameIter<T> {
		TiledFrameIter {
			frame,
			tile_idx_hor: 0,
			tile_idx_vert: 0,
		}
	}
}

impl <'a, T: PixelTrait> Iterator for TiledFrameIter<'a, T> {
    type Item = Tile<T>;
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.tile_idx_vert < self.frame.num_tiles_in_col as usize &&
			self.tile_idx_hor < self.frame.num_tiles_in_row as usize
		{
			let tile = self.frame.detach_tile(self.tile_idx_hor, self.tile_idx_vert);
			if self.tile_idx_hor == (self.frame.num_tiles_in_row - 1) as usize {
				self.tile_idx_hor = 0;
				self.tile_idx_vert += 1;
			}
			else {
				self.tile_idx_hor += 1;
			}
			Some(tile)
		}
		else {
			None
		}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_2x2() {
        let frame = TiledFrame::<Rgb>::new(800, 800, 400, 400);
        let mut tiles = frame.iter();
		assert_eq!(tiles.next(), Some(Tile::<Rgb>::new(0, 0, 400, 400)));
        assert_eq!(tiles.next(), Some(Tile::<Rgb>::new(400, 0, 400, 400)));
        assert_eq!(tiles.next(), Some(Tile::<Rgb>::new(0, 400, 400, 400)));
        assert_eq!(tiles.next(), Some(Tile::<Rgb>::new(400, 400, 400, 400)));
        assert_eq!(tiles.next(), None);
    }

    #[test]
    fn iter_4x4() {
		let frame = TiledFrame::<Rgb>::new(800, 800, 200, 200);
		let mut tiles = frame.iter();
        assert_eq!(tiles.next(), Some(Tile::new(0, 0, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(200, 0, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(400, 0, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(600, 0, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(0, 200, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(200, 200, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(400, 200, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(600, 200, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(0, 400, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(200, 400, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(400, 400, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(600, 400, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(0, 600, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(200, 600, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(400, 600, 200, 200)));
        assert_eq!(tiles.next(), Some(Tile::new(600, 600, 200, 200)));
        assert_eq!(tiles.next(), None);
    }
	
    #[test]
    fn iter_1x2() {
        let frame = TiledFrame::<Rgb>::new(100, 100, 100, 50);
		let mut tiles = frame.iter();
        assert_eq!(tiles.next(), Some(Tile::new(0, 0, 100, 50)));
        assert_eq!(tiles.next(), Some(Tile::new(0, 50, 100, 50)));
        assert_eq!(tiles.next(), None);
    }
	
    #[test]
    fn iter_2x2_narrow() {
		let frame = TiledFrame::<Rgb>::new(100, 100, 60, 60);
		let mut tiles = frame.iter();
        assert_eq!(tiles.next(), Some(Tile::new(0, 0, 60, 60)));
        assert_eq!(tiles.next(), Some(Tile::new(60, 0, 40, 60)));
        assert_eq!(tiles.next(), Some(Tile::new(0, 60, 60, 40)));
        assert_eq!(tiles.next(), Some(Tile::new(60, 60, 40, 40)));
        assert_eq!(tiles.next(), None);
    }
	
	#[test]
	fn tile_equal_to_frame() {
		let frame = TiledFrame::<Rgb>::new(100, 100, 100, 100);
		let mut tiles = frame.iter();
		assert_eq!(tiles.next(), Some(Tile::new(0, 0, 100, 100)));
		assert_eq!(tiles.next(), None);
	}
	
    #[test]
    fn tile_larger_than_frame_xy() {
		let frame = TiledFrame::<Rgb>::new(100, 100, 150, 150);
		let mut tiles = frame.iter();
        assert_eq!(tiles.next(), Some(Tile::new(0, 0, 100, 100)));
        assert_eq!(tiles.next(), None);
    }
	
	#[test]
	fn tile_x_larger_y_smaller() {
		let frame = TiledFrame::<Rgb>::new(100, 100, 150, 90);
		let mut tiles = frame.iter();
		assert_eq!(tiles.next(), Some(Tile::new(0, 0, 100, 90)));
		assert_eq!(tiles.next(), Some(Tile::new(0, 90, 100, 10)));
		assert_eq!(tiles.next(), None);
	}
	
	#[test]
	fn tile_x_smaller_y_larger() {
		let frame = TiledFrame::<Rgb>::new(100, 100, 90, 150);
		let mut tiles = frame.iter();
		assert_eq!(tiles.next(), Some(Tile::new(0, 0, 90, 100)));
		assert_eq!(tiles.next(), Some(Tile::new(90, 0, 10, 100)));
		assert_eq!(tiles.next(), None);
	}
	
	impl PixelTrait for u8 {
		fn zero() -> u8 {
			0
		}
		fn set_rgb(r: u8, g: u8, b: u8) -> Self {
			(r as u32 + g as u32 + b as u32 / 3) as u8
		}
	}
	
	const test_frame: [u8; 64] = [
		0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0,
		0, 1, 1, 1, 1, 1, 0, 0,
		0, 1, 1, 1, 1, 1, 0, 0,
		0, 1, 1, 1, 1, 1, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0,
	];
	const test_tiles:[[u8; 16]; 4] = [
		[	0, 0, 0, 0,
			0, 0, 0, 0,
			0, 0, 0, 0,
			0, 1, 1, 1,
		],
		[
			0, 0, 0, 0,
			0, 0, 0, 0,
			0, 0, 0, 0,
			1, 1, 0, 0,
		],
		[
			0, 1, 1, 1,
			0, 1, 1, 1,
			0, 0, 0, 0,
			0, 0, 0, 0,
		],
		[
			1, 1, 0, 0,
			1, 1, 0, 0,
			0, 0, 0, 0,
			0, 0, 0, 0,
		],
	];
	
	#[test]
	fn tile_detach_top_left() {
		let mut frame = TiledFrame::<u8>::new(8, 8, 4, 4);
		frame.pixels = test_frame.to_vec();
		let reference = Tile {
			origin_x: 0,
			origin_y: 0,
			width: 4,
			height: 4,
			pixels: test_tiles[0].to_vec(),
		};
		let from_frame = frame.detach_tile(0, 0);
		assert_eq!(from_frame, reference);
	}
	
	#[test]
	fn tile_detach_bot_right() {
		let mut frame = TiledFrame::<u8>::new(8, 8, 4, 4);
		frame.pixels = test_frame.to_vec();
		let reference = Tile {
			origin_x: 4,
			origin_y: 4,
			width: 4,
			height: 4,
			pixels: test_tiles[3].to_vec(),
		};
		let from_frame = frame.detach_tile(1, 1);
		assert_eq!(from_frame, reference);
	}
	
	#[test]
	fn tile_detach_from_iter() {
		let mut frame = TiledFrame::<u8>::new(8, 8, 4, 4);
		frame.pixels = test_frame.to_vec();
		let mut tiles = frame.iter();
		assert_eq!(tiles.next().unwrap().pixels, test_tiles[0].to_vec());
		assert_eq!(tiles.next().unwrap().pixels, test_tiles[1].to_vec());
		assert_eq!(tiles.next().unwrap().pixels, test_tiles[2].to_vec());
		assert_eq!(tiles.next().unwrap().pixels, test_tiles[3].to_vec());
		assert_eq!(tiles.next(), None);
	}
	
	#[test]
	fn tile_merge() {
		let reference_tl = Tile {
			origin_x: 0,
			origin_y: 0,
			width: 4,
			height: 4,
			pixels: test_tiles[0].to_vec(),
		};
		let reference_tr = Tile {
			origin_x: 4,
			origin_y: 0,
			width: 4,
			height: 4,
			pixels: test_tiles[1].to_vec(),
		};
		let reference_bl = Tile {
			origin_x: 0,
			origin_y: 4,
			width: 4,
			height: 4,
			pixels: test_tiles[2].to_vec(),
		};
		let reference_br = Tile {
			origin_x: 4,
			origin_y: 4,
			width: 4,
			height: 4,
			pixels: test_tiles[3].to_vec(),
		};
		
		let mut frame = TiledFrame::<u8>::new(8, 8, 4, 4);
		frame.merge_tile(&reference_tl);
		frame.merge_tile(&reference_tr);
		frame.merge_tile(&reference_bl);
		frame.merge_tile(&reference_br);
		assert_eq!(frame.pixels, test_frame);
		
		// and now in reverse order:
		let mut frame = TiledFrame::<u8>::new(8, 8, 4, 4);
		frame.merge_tile(&reference_br);
		frame.merge_tile(&reference_bl);
		frame.merge_tile(&reference_tr);
		frame.merge_tile(&reference_tl);
		assert_eq!(frame.pixels, test_frame);
	}
}
