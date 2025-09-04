use std::collections::HashMap;
use std::sync::LazyLock;

use crate::raster_renderer::SimpleTileSet;
use crate::raster_renderer::tile_set_util::*;
use crate::tiles::*;

macro_rules! load_tile_set {
    ($path:literal, $tile_set_name:literal) => {{
        let map = load_tile_map!($path, $tile_set_name);
        SimpleTileSet::new(map).expect("could not create tile set")
    }};
}

/// Tile set based on the Martin Persson's tiles
pub static MARTIN_PERSSON_TILE_SET: LazyLock<SimpleTileSet> =
    LazyLock::new(|| load_tile_set!("../../tilesets/", "MartinPersson"));
