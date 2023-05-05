use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::raster_renderer::SimpleTileSet;
use crate::tiles::*;

use crate::raster_renderer::tile_set_util::*;

macro_rules! load_tile_set {
    ($path:literal, $tile_set_name:literal) => {{
        let map = load_tile_map!($path, $tile_set_name);
        SimpleTileSet::new(map).expect("could not create tile set")
    }};
}

lazy_static! {
    /// Tile set based on the Martin Persson's tiles
    pub static ref MARTIN_PERSSON_TILE_SET: SimpleTileSet = load_tile_set!("../../tilesets/", "MartinPersson");
}
