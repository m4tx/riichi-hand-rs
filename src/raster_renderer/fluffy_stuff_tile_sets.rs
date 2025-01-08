use std::collections::HashMap;
use std::env;
use std::sync::LazyLock;

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

/// Tile set based on the FluffyStuff's tiles - Yellow version.
pub static YELLOW_FLUFFY_STUFF_TILE_SET: LazyLock<TwoPartTileSet> =
    LazyLock::new(|| load_tile_set!(concat!(env!("OUT_DIR"), "/tilesets/FluffyStuff/"), "Yellow"));

/// Tile set based on the FluffyStuff's tiles - Red version.
pub static RED_FLUFFY_STUFF_TILE_SET: LazyLock<TwoPartTileSet> =
    LazyLock::new(|| load_tile_set!(concat!(env!("OUT_DIR"), "/tilesets/FluffyStuff/"), "Red"));

/// Tile set based on the FluffyStuff's tiles - Black version.
pub static BLACK_FLUFFY_STUFF_TILE_SET: LazyLock<TwoPartTileSet> =
    LazyLock::new(|| load_tile_set!(concat!(env!("OUT_DIR"), "/tilesets/FluffyStuff/"), "Black"));
