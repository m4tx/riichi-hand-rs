#[cfg(all(not(feature = "raster-renderer"), feature = "fluffy-stuff-tile-sets"))]
compile_error!("feature \"fluffy-stuff-tile-sets\" must be used with \"raster-renderer\"");

#[cfg(all(not(feature = "raster-renderer"), feature = "martin-persson-tile-sets"))]
compile_error!("feature \"martin-persson-tile-sets\" must be used with \"raster-renderer\"");

fn main() {
    #[cfg(feature = "fluffy-stuff-tile-sets")]
    tile_set_render::render_tile_sets();

    println!("cargo:rerun-if-changed=build.rs");
}

#[cfg(feature = "fluffy-stuff-tile-sets")]
mod tile_set_render {
    use std::path::Path;
    use std::{env, fs};

    use image::RgbaImage;
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    const TILE_SETS: [&str; 3] = ["Yellow", "Red", "Black"];
    const TILE_NAMES: [&str; 37] = [
        "Man1",
        "Man2",
        "Man3",
        "Man4",
        "Man5-Dora",
        "Man5",
        "Man6",
        "Man7",
        "Man8",
        "Man9",
        "Pin1",
        "Pin2",
        "Pin3",
        "Pin4",
        "Pin5-Dora",
        "Pin5",
        "Pin6",
        "Pin7",
        "Pin8",
        "Pin9",
        "Sou1",
        "Sou2",
        "Sou3",
        "Sou4",
        "Sou5-Dora",
        "Sou5",
        "Sou6",
        "Sou7",
        "Sou8",
        "Sou9",
        "Ton",
        "Nan",
        "Pei",
        "Shaa",
        "Haku",
        "Hatsu",
        "Chun",
    ];

    const BACKGROUND_TILE_MARGIN: f32 = 0.0;
    const FOREGROUND_TILE_MARGIN: f32 = 0.05;

    pub fn render_tile_sets() {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        for tile_set in TILE_SETS {
            let tile_set_path = Path::new(&out_dir)
                .join("tilesets")
                .join("FluffyStuff")
                .join(tile_set);
            fs::create_dir_all(&tile_set_path).expect("could not create tile set output directory");

            render_and_save(tile_set, &tile_set_path, "Front", BACKGROUND_TILE_MARGIN);
            render_and_save(tile_set, &tile_set_path, "Back", BACKGROUND_TILE_MARGIN);

            TILE_NAMES.into_par_iter().for_each(|tile_name| {
                render_and_save(tile_set, &tile_set_path, tile_name, FOREGROUND_TILE_MARGIN);
            });
        }
    }

    fn render_and_save(tile_set: &str, tile_set_path: &Path, name: &str, margin: f32) {
        let back_tile = render_svg(
            &format!("tilesets/FluffyStuff/{}/{}.svg", tile_set, name),
            margin,
        );
        let dest_path = Path::new(&tile_set_path).join(format!("{}.png", name));
        back_tile.save(dest_path).expect("could not save file");
    }

    fn render_svg(path: &str, margin: f32) -> RgbaImage {
        println!("cargo:rerun-if-changed={}", path);

        let opt = usvg::Options::default();
        let svg_data = fs::read(path).unwrap();
        let rtree = {
            usvg::Tree::from_data(&svg_data, &opt).unwrap()
        };

        let pixmap_size = rtree.size().to_int_size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        resvg::render(
            &rtree,
            tiny_skia::Transform::from_scale(1.0 - 2.0 * margin, 1.0 - 2.0 * margin)
                .post_translate(
                    pixmap_size.width() as f32 * margin,
                    pixmap_size.height() as f32 * margin,
                ),
            &mut pixmap.as_mut(),
        );

        image::RgbaImage::from_raw(
            pixmap_size.width(),
            pixmap_size.height(),
            pixmap.data().to_vec(),
        )
        .expect("could not construct an image")
    }
}
