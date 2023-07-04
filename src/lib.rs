//! A collection of utilities for working with Riichi Mahjong player hands.
//!
//! This currently includes:
//! * Hand representation object
//! * Parser that allows to quickly create a hand using human-readable string,
//!   such as `123m456p_7*77z`
//! * Renderer that allows to draw a hand to a raster image (along with a few
//!   ready-to-use sets of tile images)
//!
//! ## Feature flags
//! By default, all the crate features are enabled. You can disable some of them
//! on demand to save on the compilation time and resulting library size.
//!
//! * `raster-renderer` - renderer for hands that outputs raster images
//! * `fluffy-stuff-tile-sets` - ready-to-use tile sets based on FluffyStuff's
//!   work. Greatly increases build time
//! * `martin-persson-tile-sets` - ready-to-use tile sets based on Martin
//!   Persson's work
//!
//! ## Example
//! ```
//! use riichi_hand::parser::HandParser;
//! use riichi_hand::raster_renderer::fluffy_stuff_tile_sets::YELLOW_FLUFFY_STUFF_TILE_SET;
//! use riichi_hand::raster_renderer::{RasterRenderer, RenderOptions};
//!
//! let hand = HandParser::parse("123m123p123sEESS").unwrap();
//! let image = RasterRenderer::render(&hand, &*YELLOW_FLUFFY_STUFF_TILE_SET, RenderOptions::default()).unwrap();
//! image.save("/tmp/hand.png").unwrap();
//! ```

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

pub use hand::*;

mod hand;

/// Hand parser that is able to convert string representation of a hand (e.g.
/// `123m456p_7*77z`) into [Hand] instance
pub mod parser;

#[cfg(feature = "raster-renderer")]
/// Module that renders [Hand] instance into raster images
pub mod raster_renderer;

/// Constant objects that represent all valid tiles
pub mod tiles;
