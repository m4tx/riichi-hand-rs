riichi-hand-rs
==============

[![crates.io](https://img.shields.io/crates/v/riichi_hand.svg)](https://crates.io/crates/riichi_hand)
[![Documentation](https://docs.rs/riichi_hand/badge.svg)](https://docs.rs/riichi_hand)
[![Build Status](https://github.com/m4tx/riichi-hand-rs/workflows/Rust%20CI/badge.svg)](https://github.com/m4tx/riichi-hand-rs/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/m4tx/riichi-hand-rs/blob/master/LICENSE)

A collection of utilities for working with Riichi Mahjong player hands to use
with Rust programming language.

This currently includes:

* Hand representation object
* Parser that allows to quickly create a hand using human-readable string, such
  as `123m456p_7*77z`
* Renderer that allows to draw a hand to a raster image (along with a few
  ready-to-use sets of tile images)

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
riichi_hand = "0.2.0"
```

On the feature flags overview, please refer to
the [crate documentation](http://docs.rs/riichi_hand/).

## Example

```rust
use riichi_hand::parser::HandParser;
use riichi_hand::raster_renderer::fluffy_stuff_tile_sets::YELLOW_FLUFFY_STUFF_TILE_SET;
use riichi_hand::raster_renderer::{RasterRenderer, RenderOptions};

fn main() {
    let hand = HandParser::parse("123m123p123sEESS").unwrap();
    let image = RasterRenderer::render(&hand, &*YELLOW_FLUFFY_STUFF_TILE_SET, RenderOptions::default());
    image.save("/tmp/hand.png").unwrap();
}
```

## Uses

* [chombot](https://github.com/riichi/chombot) - Discord bot for Krakow Chombo
  Club's Discord server

## License

The project is licensed under the [MIT license](LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the project by you shall be licensed as MIT, without any
additional terms or conditions.

## Attribution

This project uses modified
[riichi-mahjong-tiles](https://github.com/FluffyStuff/riichi-mahjong-tiles)
by [FluffyStuff](https://github.com/FluffyStuff), licensed
under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).
