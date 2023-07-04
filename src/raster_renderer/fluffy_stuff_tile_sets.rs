use std::collections::HashMap;
use std::env;

use lazy_static::lazy_static;

use crate::raster_renderer::tile_set_util::*;
use crate::raster_renderer::TwoPartTileSet;
use crate::tiles::*;

macro_rules! load_tile_set {
    ($path:expr, $tile_set_name:literal) => {{
        let map = load_tile_map!($path, $tile_set_name);
        let front = load_tile_image!($path, $tile_set_name, "Front");
        TwoPartTileSet::new(front, map).expect("could not create tile set")
    }};
}

lazy_static! {
    /// Tile set based on the FluffyStuff's tiles - Yellow version.
    pub static ref YELLOW_FLUFFY_STUFF_TILE_SET: TwoPartTileSet =
        load_tile_set!(concat!(env!("OUT_DIR"), "/tilesets/FluffyStuff/"), "Yellow");

    /// Tile set based on the FluffyStuff's tiles - Red version.
    pub static ref RED_FLUFFY_STUFF_TILE_SET: TwoPartTileSet =
        load_tile_set!(concat!(env!("OUT_DIR"), "/tilesets/FluffyStuff/"), "Red");

    /// Tile set based on the FluffyStuff's tiles - Black version.
    pub static ref BLACK_FLUFFY_STUFF_TILE_SET: TwoPartTileSet =
        load_tile_set!(concat!(env!("OUT_DIR"), "/tilesets/FluffyStuff/"), "Black");
}
