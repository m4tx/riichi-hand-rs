use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use image::{ImageBuffer, RgbaImage};

use crate::tiles::{ALL_TILES, ANY};
use crate::TilePlacement::Normal;
use crate::{HandTile, Tile};

/// Set of tile images that can be used to render a hand using
/// [RasterRenderer](super::RasterRenderer).
pub trait TileSet {
    /// Returns an image of given tile, taking into account the tile placement (rotation).
    ///
    /// The returned image should always have dimensions `W x H` for tiles that are not rotated, and
    /// `H x W` for the rotated versions (where W and H are the return values of `tile_width()` and
    /// `tile_height()`, respectively)
    fn tile_image(&self, hand_tile: &HandTile) -> RgbaImage;

    /// Returns tile width, in pixels. Must be the same for all images.
    fn tile_width(&self) -> u32;

    /// Returns tile height, in pixels. Must be the same for all images.
    fn tile_height(&self) -> u32;
}

#[derive(Debug)]
/// An implementation of [TileSet] that expects a hash map of tile foregrounds and a single
/// background image.
///
/// This implementation automatically combines background and foreground on the fly. Also, it
/// assumes "realistic" light for the rendered tiles (i.e. for rotated tiles, it mirrors the
/// background, so it always seems like the light is coming from once source).
pub struct SimpleTileSet {
    front: RgbaImage,
    tile_map: HashMap<Tile, RgbaImage>,
    tile_width: u32,
    tile_height: u32,
}

#[derive(Copy, Clone, Debug)]
/// An error that occurs when creating a [SimpleTileSet].
pub enum TileSetCreationError {
    /// There is a tile missing in the image foreground map.
    TileMissing(Tile),
    /// Images passed (both background and foregrounds) have different dimensions.
    ImagesDoNotHaveEqualDimensions,
}

impl Error for TileSetCreationError {}

impl Display for TileSetCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileSetCreationError::TileMissing(tile) => {
                write!(f, "tile foreground missing: {}", tile)
            }
            TileSetCreationError::ImagesDoNotHaveEqualDimensions => write!(
                f,
                "images (backgrounds and foregrounds) do not have equal dimensions"
            ),
        }
    }
}

impl SimpleTileSet {
    /// Creates a new [SimpleTileSet] instance using given background image and a map of tile
    /// foregrounds.
    pub fn new(
        front: RgbaImage,
        tile_map: HashMap<Tile, RgbaImage>,
    ) -> Result<Self, TileSetCreationError> {
        Self::validate_tile_map(&front, &tile_map)?;

        let tile_width = tile_map[&ANY].width();
        let tile_height = tile_map[&ANY].height();

        Ok(Self {
            front,
            tile_map,
            tile_width,
            tile_height,
        })
    }

    fn validate_tile_map(
        front: &RgbaImage,
        tile_map: &HashMap<Tile, RgbaImage>,
    ) -> Result<(), TileSetCreationError> {
        for tile in ALL_TILES {
            if !tile_map.contains_key(&tile) {
                return Err(TileSetCreationError::TileMissing(tile));
            }
        }

        let tile_width = tile_map[&ANY].width();
        let tile_height = tile_map[&ANY].height();
        let same_dimensions = tile_map
            .values()
            .chain([front])
            .all(|image| image.width() == tile_width && image.height() == tile_height);
        if !same_dimensions {
            return Err(TileSetCreationError::ImagesDoNotHaveEqualDimensions);
        }

        Ok(())
    }

    fn hand_tile_foreground(&self, hand_tile: &HandTile) -> RgbaImage {
        if hand_tile.tile == ANY {
            return ImageBuffer::new(0, 0);
        }

        let buffer = &self.tile_map[&hand_tile.tile];

        if hand_tile.placement == Normal {
            buffer.clone()
        } else {
            image::imageops::rotate90(buffer)
        }
    }

    fn hand_tile_background(&self, hand_tile: &HandTile) -> RgbaImage {
        let background = if hand_tile.tile == ANY {
            &self.tile_map[&ANY]
        } else {
            &self.front
        };

        if hand_tile.placement == Normal {
            background.clone()
        } else {
            let mut new_background = image::imageops::rotate90(background);
            image::imageops::flip_horizontal_in_place(&mut new_background);
            new_background
        }
    }
}

impl TileSet for SimpleTileSet {
    #[inline]
    fn tile_image(&self, hand_tile: &HandTile) -> RgbaImage {
        let mut background = self.hand_tile_background(hand_tile);
        let foreground = self.hand_tile_foreground(hand_tile);
        image::imageops::overlay(&mut background, &foreground, 0, 0);

        background
    }

    #[inline]
    fn tile_width(&self) -> u32 {
        self.tile_width
    }

    #[inline]
    fn tile_height(&self) -> u32 {
        self.tile_height
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use image::ImageBuffer;

    use crate::raster_renderer::{SimpleTileSet, TileSetCreationError};
    use crate::tiles::{ALL_TILES, ANY};

    #[test]
    fn should_return_tile_missing_error() {
        let result = SimpleTileSet::new(ImageBuffer::new(16, 16), HashMap::new());
        assert!(result.is_err());
        match result.err().unwrap() {
            TileSetCreationError::TileMissing(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn should_return_image_dimensions_error() {
        let buffer1 = ImageBuffer::new(16, 16);
        let buffer2 = ImageBuffer::new(32, 32);
        let mut map = HashMap::new();
        for tile in ALL_TILES {
            map.insert(tile, buffer1.clone());
        }
        map.insert(ANY, buffer2);

        let result = SimpleTileSet::new(buffer1, map);
        assert!(result.is_err());
        match result.err().unwrap() {
            TileSetCreationError::ImagesDoNotHaveEqualDimensions => assert!(true),
            _ => assert!(false),
        }

        let buffer1 = ImageBuffer::new(16, 16);
        let buffer2 = ImageBuffer::new(32, 32);
        let mut map = HashMap::new();
        for tile in ALL_TILES {
            map.insert(tile, buffer1.clone());
        }

        let result = SimpleTileSet::new(buffer2, map);
        assert!(result.is_err());
        match result.err().unwrap() {
            TileSetCreationError::ImagesDoNotHaveEqualDimensions => assert!(true),
            _ => assert!(false),
        }
    }
}
