use std::collections::HashMap;
use std::env;

use image::{ImageFormat, RgbaImage};
use lazy_static::lazy_static;

use crate::raster_renderer::SimpleTileSet;
use crate::tiles::*;

macro_rules! load_tile_set {
    ($tile_set_name:literal) => {{
        let mut map = HashMap::with_capacity(ALL_TILES.len());
        let front = load_tile_image!($tile_set_name, "Front");
        load_tile!(map, AKADORA_MAN, $tile_set_name, "Man5-Dora");
        load_tile!(map, II_MAN, $tile_set_name, "Man1");
        load_tile!(map, RYAN_MAN, $tile_set_name, "Man2");
        load_tile!(map, SAN_MAN, $tile_set_name, "Man3");
        load_tile!(map, SUU_MAN, $tile_set_name, "Man4");
        load_tile!(map, UU_MAN, $tile_set_name, "Man5");
        load_tile!(map, ROU_MAN, $tile_set_name, "Man6");
        load_tile!(map, CHII_MAN, $tile_set_name, "Man7");
        load_tile!(map, PAA_MAN, $tile_set_name, "Man8");
        load_tile!(map, KYUU_MAN, $tile_set_name, "Man9");
        load_tile!(map, AKADORA_PIN, $tile_set_name, "Pin5-Dora");
        load_tile!(map, II_PIN, $tile_set_name, "Pin1");
        load_tile!(map, RYAN_PIN, $tile_set_name, "Pin2");
        load_tile!(map, SAN_PIN, $tile_set_name, "Pin3");
        load_tile!(map, SUU_PIN, $tile_set_name, "Pin4");
        load_tile!(map, UU_PIN, $tile_set_name, "Pin5");
        load_tile!(map, ROU_PIN, $tile_set_name, "Pin6");
        load_tile!(map, CHII_PIN, $tile_set_name, "Pin7");
        load_tile!(map, PAA_PIN, $tile_set_name, "Pin8");
        load_tile!(map, KYUU_PIN, $tile_set_name, "Pin9");
        load_tile!(map, AKADORA_SOU, $tile_set_name, "Sou5-Dora");
        load_tile!(map, II_SOU, $tile_set_name, "Sou1");
        load_tile!(map, RYAN_SOU, $tile_set_name, "Sou2");
        load_tile!(map, SAN_SOU, $tile_set_name, "Sou3");
        load_tile!(map, SUU_SOU, $tile_set_name, "Sou4");
        load_tile!(map, UU_SOU, $tile_set_name, "Sou5");
        load_tile!(map, ROU_SOU, $tile_set_name, "Sou6");
        load_tile!(map, CHII_SOU, $tile_set_name, "Sou7");
        load_tile!(map, PAA_SOU, $tile_set_name, "Sou8");
        load_tile!(map, KYUU_SOU, $tile_set_name, "Sou9");
        load_tile!(map, TON, $tile_set_name, "Ton");
        load_tile!(map, NAN, $tile_set_name, "Nan");
        load_tile!(map, SHAA, $tile_set_name, "Shaa");
        load_tile!(map, PEI, $tile_set_name, "Pei");
        load_tile!(map, HAKU, $tile_set_name, "Haku");
        load_tile!(map, HATSU, $tile_set_name, "Hatsu");
        load_tile!(map, CHUN, $tile_set_name, "Chun");
        load_tile!(map, ANY, $tile_set_name, "Back");
        SimpleTileSet::new(front, map).expect("could not create tile set")
    }};
}

macro_rules! load_tile {
    ($map:expr, $tile:expr, $tile_set_name:literal, $tile_name:literal) => {
        $map.insert($tile, load_tile_image!($tile_set_name, $tile_name));
    };
}

macro_rules! load_tile_image {
    ($tile_set_name:literal, $tile_name:literal) => {
        load_png_from_memory(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/tilesets/",
            $tile_set_name,
            "/",
            $tile_name,
            ".png"
        )))
    };
}

lazy_static! {
    /// Tile set based on the FluffyStuff's tiles - Yellow version.
    pub static ref YELLOW_FLUFFY_STUFF_TILE_SET: SimpleTileSet = load_tile_set!("Yellow");

    /// Tile set based on the FluffyStuff's tiles - Red version.
    pub static ref RED_FLUFFY_STUFF_TILE_SET: SimpleTileSet = load_tile_set!("Red");

    /// Tile set based on the FluffyStuff's tiles - Black version.
    pub static ref BLACK_FLUFFY_STUFF_TILE_SET: SimpleTileSet = load_tile_set!("Black");
}

fn load_png_from_memory(buf: &[u8]) -> RgbaImage {
    image::load_from_memory_with_format(buf, ImageFormat::Png)
        .expect("could not load image")
        .to_rgba8()
}
