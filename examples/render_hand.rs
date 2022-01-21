use riichi_hand::parser::HandParser;
use riichi_hand::raster_renderer::fluffy_stuff_tile_sets::YELLOW_FLUFFY_STUFF_TILE_SET;
use riichi_hand::raster_renderer::{RasterRenderer, RenderOptions};
use std::io;

fn main() {
    println!("Loading the tile set...");
    let tile_set = &*YELLOW_FLUFFY_STUFF_TILE_SET;

    println!("Please provide hand string representation (e.g. 123m456p789sEEES):");
    let hand = HandParser::parse(read_line().trim()).expect("could not parse hand");

    println!("Where do you want to save the image?");
    let path = read_line();

    println!("Rendering the hand...");
    let image = RasterRenderer::render(&hand, tile_set, RenderOptions::default());
    image.save(path.trim()).expect("could not save image");
    println!("Successfully rendered the hand");
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input
}
