use image::{ImageFormat, RgbaImage};

macro_rules! load_tile_map {
    ($path:expr, $tile_set_name:literal) => {{
        let mut map = HashMap::with_capacity(ALL_TILES.len());
        load_tile!(map, AKADORA_MAN, $path, $tile_set_name, "Man5-Dora");
        load_tile!(map, II_MAN, $path, $tile_set_name, "Man1");
        load_tile!(map, RYAN_MAN, $path, $tile_set_name, "Man2");
        load_tile!(map, SAN_MAN, $path, $tile_set_name, "Man3");
        load_tile!(map, SUU_MAN, $path, $tile_set_name, "Man4");
        load_tile!(map, UU_MAN, $path, $tile_set_name, "Man5");
        load_tile!(map, ROU_MAN, $path, $tile_set_name, "Man6");
        load_tile!(map, CHII_MAN, $path, $tile_set_name, "Man7");
        load_tile!(map, PAA_MAN, $path, $tile_set_name, "Man8");
        load_tile!(map, KYUU_MAN, $path, $tile_set_name, "Man9");
        load_tile!(map, AKADORA_PIN, $path, $tile_set_name, "Pin5-Dora");
        load_tile!(map, II_PIN, $path, $tile_set_name, "Pin1");
        load_tile!(map, RYAN_PIN, $path, $tile_set_name, "Pin2");
        load_tile!(map, SAN_PIN, $path, $tile_set_name, "Pin3");
        load_tile!(map, SUU_PIN, $path, $tile_set_name, "Pin4");
        load_tile!(map, UU_PIN, $path, $tile_set_name, "Pin5");
        load_tile!(map, ROU_PIN, $path, $tile_set_name, "Pin6");
        load_tile!(map, CHII_PIN, $path, $tile_set_name, "Pin7");
        load_tile!(map, PAA_PIN, $path, $tile_set_name, "Pin8");
        load_tile!(map, KYUU_PIN, $path, $tile_set_name, "Pin9");
        load_tile!(map, AKADORA_SOU, $path, $tile_set_name, "Sou5-Dora");
        load_tile!(map, II_SOU, $path, $tile_set_name, "Sou1");
        load_tile!(map, RYAN_SOU, $path, $tile_set_name, "Sou2");
        load_tile!(map, SAN_SOU, $path, $tile_set_name, "Sou3");
        load_tile!(map, SUU_SOU, $path, $tile_set_name, "Sou4");
        load_tile!(map, UU_SOU, $path, $tile_set_name, "Sou5");
        load_tile!(map, ROU_SOU, $path, $tile_set_name, "Sou6");
        load_tile!(map, CHII_SOU, $path, $tile_set_name, "Sou7");
        load_tile!(map, PAA_SOU, $path, $tile_set_name, "Sou8");
        load_tile!(map, KYUU_SOU, $path, $tile_set_name, "Sou9");
        load_tile!(map, TON, $path, $tile_set_name, "Ton");
        load_tile!(map, NAN, $path, $tile_set_name, "Nan");
        load_tile!(map, SHAA, $path, $tile_set_name, "Shaa");
        load_tile!(map, PEI, $path, $tile_set_name, "Pei");
        load_tile!(map, HAKU, $path, $tile_set_name, "Haku");
        load_tile!(map, HATSU, $path, $tile_set_name, "Hatsu");
        load_tile!(map, CHUN, $path, $tile_set_name, "Chun");
        load_tile!(map, ANY, $path, $tile_set_name, "Back");
        map
    }};
}

macro_rules! load_tile {
    ($map:expr, $tile:expr, $path:expr, $tile_set_name:literal, $tile_name:literal) => {
        $map.insert($tile, load_tile_image!($path, $tile_set_name, $tile_name));
    };
}

macro_rules! load_tile_image {
    ($path:expr, $tile_set_name:literal, $tile_name:literal) => {
        load_png_from_memory(include_bytes!(concat!(
            $path,
            $tile_set_name,
            "/",
            $tile_name,
            ".png"
        )))
    };
}

pub(super) fn load_png_from_memory(buf: &[u8]) -> RgbaImage {
    image::load_from_memory_with_format(buf, ImageFormat::Png)
        .expect("could not load image")
        .to_rgba8()
}

pub(super) use {load_tile, load_tile_image, load_tile_map};
