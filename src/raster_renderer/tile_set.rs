use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use image::{ImageBuffer, RgbaImage};

use crate::TilePlacement::Normal;
use crate::tiles::{ALL_TILES, ANY};
use crate::{HandTile, Tile};

/// Result of [TileSet::tile_image].
pub type TileImageResult = Result<RgbaImage, TileImageRetrieveError>;

/// Set of tile images that can be used to render a hand using
/// [RasterRenderer](super::RasterRenderer).
pub trait TileSet {
    /// Returns an image of given tile, taking into account the tile placement
    /// (rotation).
    ///
    /// The returned image should always have dimensions `W x H` for tiles that
    /// are not rotated, and `H x W` for the rotated versions (where W and H
    /// are the return values of `tile_width()` and `tile_height()`,
    /// respectively)
    fn tile_image(&self, hand_tile: &HandTile) -> TileImageResult;

    /// Returns tile width, in pixels. Must be the same for all images.
    fn tile_width(&self) -> u32;

    /// Returns tile height, in pixels. Must be the same for all images.
    fn tile_height(&self) -> u32;
}

impl<T: TileSet + ?Sized> TileSet for &T {
    fn tile_image(&self, hand_tile: &HandTile) -> TileImageResult {
        T::tile_image(self, hand_tile)
    }

    fn tile_width(&self) -> u32 {
        T::tile_width(self)
    }

    fn tile_height(&self) -> u32 {
        T::tile_height(self)
    }
}

impl<T: TileSet + ?Sized> TileSet for Box<T> {
    fn tile_image(&self, hand_tile: &HandTile) -> TileImageResult {
        T::tile_image(self, hand_tile)
    }

    fn tile_width(&self) -> u32 {
        T::tile_width(self)
    }

    fn tile_height(&self) -> u32 {
        T::tile_height(self)
    }
}

#[derive(Clone, Debug)]
/// An error that occurs when calling [TileSet::tile_image].
pub enum TileImageRetrieveError {
    /// This specific hand tile is not supported.
    TileNotSupported(HandTile, String),
}

impl Error for TileImageRetrieveError {}

impl Display for TileImageRetrieveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TileNotSupported(tile, message) => {
                write!(f, "tile {} not supported: {}", tile, message)
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
/// An error that occurs when creating a [TwoPartTileSet] or a [SimpleTileSet].
pub enum TileSetCreationError {
    /// There is a tile missing in the image foreground map.
    TileMissing(Tile),
    /// Images passed (both background and foregrounds) have different
    /// dimensions.
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

#[derive(Debug)]
/// An implementation of [TileSet] that expects a hash map of tile images.
///
/// This implementation does not support rotated tiles. This just returns the
/// tiles as is, returning an error if an unsupported tile is requested.
pub struct SimpleTileSet {
    tile_map: HashMap<Tile, RgbaImage>,
    tile_width: u32,
    tile_height: u32,
}

impl SimpleTileSet {
    /// Creates a new [TwoPartTileSet] instance using given map of tile images.
    pub fn new(tile_map: HashMap<Tile, RgbaImage>) -> Result<Self, TileSetCreationError> {
        Self::validate_tile_map(&tile_map)?;

        let tile_width = tile_map[&ANY].width();
        let tile_height = tile_map[&ANY].height();

        Ok(Self {
            tile_map,
            tile_width,
            tile_height,
        })
    }

    fn validate_tile_map(tile_map: &HashMap<Tile, RgbaImage>) -> Result<(), TileSetCreationError> {
        for tile in ALL_TILES {
            if !tile_map.contains_key(&tile) {
                return Err(TileSetCreationError::TileMissing(tile));
            }
        }

        let tile_width = tile_map[&ANY].width();
        let tile_height = tile_map[&ANY].height();
        let same_dimensions = tile_map
            .values()
            .all(|image| image.width() == tile_width && image.height() == tile_height);
        if !same_dimensions {
            return Err(TileSetCreationError::ImagesDoNotHaveEqualDimensions);
        }

        Ok(())
    }
}

impl TileSet for SimpleTileSet {
    #[inline]
    fn tile_image(&self, hand_tile: &HandTile) -> TileImageResult {
        if hand_tile.placement == Normal {
            Ok(self.tile_map[&hand_tile.tile].clone())
        } else {
            Err(TileImageRetrieveError::TileNotSupported(
                *hand_tile,
                "this tile set does not support rotated tiles".to_string(),
            ))
        }
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

#[derive(Debug)]
/// An implementation of [TileSet] that expects a hash map of tile foregrounds
/// and a single background image.
///
/// This implementation automatically combines background and foreground on the
/// fly. Also, it assumes "realistic" light for the rendered tiles (i.e. for
/// rotated tiles, it mirrors the background, so it always seems like the light
/// is coming from once source).
pub struct TwoPartTileSet {
    front: RgbaImage,
    tile_map: HashMap<Tile, RgbaImage>,
    tile_width: u32,
    tile_height: u32,
}

impl TwoPartTileSet {
    /// Creates a new [TwoPartTileSet] instance using given background image and
    /// a map of tile foregrounds.
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

impl TileSet for TwoPartTileSet {
    #[inline]
    fn tile_image(&self, hand_tile: &HandTile) -> TileImageResult {
        let mut background = self.hand_tile_background(hand_tile);
        let foreground = self.hand_tile_foreground(hand_tile);
        image::imageops::overlay(&mut background, &foreground, 0, 0);

        Ok(background)
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

    use crate::HandTile;
    use crate::TilePlacement::Normal;
    use crate::raster_renderer::{TileSet, TileSetCreationError, TwoPartTileSet};
    use crate::tiles::{ALL_TILES, ANY, II_PIN};

    #[test]
    fn should_return_tile_missing_error() {
        let result = TwoPartTileSet::new(ImageBuffer::new(16, 16), HashMap::new());
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            TileSetCreationError::TileMissing(_)
        ));
    }

    #[test]
    fn should_work_with_boxes() {
        let buffer1 = ImageBuffer::new(16, 16);
        let mut map = HashMap::new();
        for tile in ALL_TILES {
            map.insert(tile, buffer1.clone());
        }

        let result: Box<dyn TileSet> = Box::new(TwoPartTileSet::new(buffer1, map).unwrap());
        assert_eq!(TileSet::tile_height(&result), 16);
        assert_eq!(TileSet::tile_width(&result), 16);
        assert!(TileSet::tile_image(&result, &HandTile::new(II_PIN, Normal)).is_ok());
    }

    #[test]
    fn should_work_with_boxed_references() {
        let buffer1 = ImageBuffer::new(16, 16);
        let mut map = HashMap::new();
        for tile in ALL_TILES {
            map.insert(tile, buffer1.clone());
        }

        let result = TwoPartTileSet::new(buffer1, map).unwrap();
        let result: Box<&dyn TileSet> = Box::new(&result);
        assert_eq!(TileSet::tile_height(&result), 16);
        assert_eq!(TileSet::tile_width(&result), 16);
        assert!(TileSet::tile_image(&result, &HandTile::new(II_PIN, Normal)).is_ok());
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

        let result = TwoPartTileSet::new(buffer1, map);
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            TileSetCreationError::ImagesDoNotHaveEqualDimensions
        ));

        let buffer1 = ImageBuffer::new(16, 16);
        let buffer2 = ImageBuffer::new(32, 32);
        let mut map = HashMap::new();
        for tile in ALL_TILES {
            map.insert(tile, buffer1.clone());
        }

        let result = TwoPartTileSet::new(buffer2, map);
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            TileSetCreationError::ImagesDoNotHaveEqualDimensions
        ));
    }
}
