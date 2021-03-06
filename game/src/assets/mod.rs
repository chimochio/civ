use allegro::{Bitmap, BitmapLike, Display, Flag, MemoryBitmap, SubBitmap};
use allegro::bitmap_like::MEMORY_BITMAP;
use std::collections::HashMap;
use std::rc::Weak;

static mut TILES: Option<Tilemap> = None;

struct Tilemap {
    pub tiles: HashMap<i32, Weak<SubBitmap>>,
    pub tile_width: i32,
    pub tile_height: i32,
    #[allow(dead_code)] source: Bitmap,
}

fn tilemap() -> &'static Tilemap {
    unsafe {
        match TILES {
            Some(ref x) => &(*x),
            None => panic!("attempted to get tilemap before they were initialized"),
        }
    }
}

pub fn draw_tile(core: &::allegro::Core, id: i32, dx: i32, dy: i32, flags: Option<::allegro::BitmapDrawingFlags>) {
    let tile = tilemap().tiles.get(&id).unwrap().upgrade().unwrap();

    let sx = 0.0;
    let sy = 0.0;
    let sw = tile_width() as f32;
    let sh = tile_height() as f32;
    let dx = dx as f32;
    let dy = dy as f32;
    let dw = sw;
    let dh = sh;
    let flags = flags.unwrap_or(::allegro::BitmapDrawingFlags::zero());
    core.draw_scaled_bitmap(&(*tile), sx, sy, sw, sh, dx, dy, dw, dh, flags);
}

pub fn tile_width() -> i32 {
    tilemap().tile_width
}

pub fn tile_height() -> i32 {
    tilemap().tile_height
}

/// This method should be called on a background thread to load bitmaps into memory.
pub fn load_tilemap(core: &::allegro::Core) -> MemoryBitmap {
    const PATH: &'static str = "../game/src/assets/tiles/fantasyhextiles_v2.png";
    core.set_new_bitmap_flags(MEMORY_BITMAP);
    let source = ::allegro::Bitmap::load(core, PATH).expect("failed to load tilemap source");
    core.set_new_bitmap_flags(::allegro::BitmapFlags::zero());
    source.into_memory_bitmap().unwrap_or_else(|_| panic!("failed to convert tilemap to memory bitmap"))
}

/// This method should be called on the main thread to initialize the TILES variable.
pub fn init_tilemap(display: &Display, bmp: MemoryBitmap) {
    const SRC_WIDTH: i32 = 8;
    const SRC_HEIGHT: i32 = 5;

    let bmp = display.convert_bitmap(&bmp.into_bitmap()).unwrap();
    let tile_width = bmp.get_width() / SRC_WIDTH;
    let tile_height = bmp.get_height() / SRC_HEIGHT;
    println!("Tile dimensions: {}x{}", tile_width, tile_height);
    let mut tiles = HashMap::with_capacity((SRC_WIDTH * SRC_HEIGHT) as usize);
    for y in 0..SRC_HEIGHT {
        for x in 0..SRC_WIDTH {
            let id = x + (y * SRC_WIDTH);
            let tile = (&bmp).create_sub_bitmap(x * tile_width, y * tile_height, tile_width, tile_height).expect("failed to create sub bitmap");
            tiles.insert(id, tile);
        }
    }
    unsafe {
        TILES = Some(Tilemap{
            source: bmp,
            tile_width: tile_width,
            tile_height: /*tile_height as f32*/ 30, // TODO: figure this out
            tiles: tiles,
        })
    }
}

#[cfg(test)]
mod tests {
    use allegro::BitmapLike;

    #[test]
    fn it_works() {
        println!("dir: {:?}", ::std::env::current_dir().unwrap().as_os_str());
    }
}
