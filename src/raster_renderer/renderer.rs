use std::error::Error;
use std::fmt::{Display, Formatter};

use image::{GenericImage, ImageBuffer, Rgba, RgbaImage, imageops};

use crate::TilePlacement::{Normal, Rotated, RotatedAndShifted};
use crate::raster_renderer::tile_set::{TileImageRetrieveError, TileSet};
use crate::{Hand, HandGroup, HandTile};

#[derive(Copy, Clone, Default, Debug)]
pub struct TileWidthRatio(pub f32);

#[derive(Copy, Clone, Debug)]
/// Rendering options used with [RasterRenderer].
pub struct RenderOptions {
    /// Gap between tiles, expressed as a fraction of tile width.
    pub tile_gap: TileWidthRatio,
    /// Gap between groups, expressed as a fraction of tile width.
    pub group_gap: TileWidthRatio,
}

impl RenderOptions {
    #[inline]
    /// Creates a new render options object instance.
    pub fn new(tile_gap: TileWidthRatio, group_gap: TileWidthRatio) -> Self {
        Self {
            tile_gap,
            group_gap,
        }
    }
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self::new(TileWidthRatio(0.0), TileWidthRatio(1.0 / 3.0))
    }
}

#[derive(Debug)]
/// Renders a [Hand] instance to a raster image.
pub struct RasterRenderer<'a, T: TileSet> {
    tile_set: &'a T,
    options: RenderOptions,
}

/// Alias for the return image type of [RasterRenderer::render].
pub type ImageType = RgbaImage;
/// Result of [RasterRenderer::render].
pub type HandRenderResult = Result<ImageType, HandRenderError>;

impl<'a, T: TileSet> RasterRenderer<'a, T> {
    #[inline]
    /// Renders given [Hand] instance using [TileSet] and [RenderOptions].
    pub fn render(hand: &Hand, tile_set: &'a T, options: RenderOptions) -> HandRenderResult {
        Self::new(tile_set, options).render_internal(hand)
    }

    #[inline]
    fn new(tile_set: &'a T, options: RenderOptions) -> Self {
        Self { tile_set, options }
    }

    fn render_internal(&self, hand: &Hand) -> HandRenderResult {
        let (width, height) = self.calculate_image_size(hand);
        let mut image = ImageBuffer::new(width, height);

        self.render_hand(hand, &mut image)?;

        Ok(image)
    }

    fn render_hand<I: GenericImage<Pixel = Rgba<u8>>>(
        &self,
        hand: &Hand,
        image: &mut I,
    ) -> Result<(), HandRenderError> {
        let mut start_x = 0;
        for group in hand.groups() {
            let (width, height) = self.calculate_group_size(group);
            let mut sub_image =
                imageops::crop(image, start_x, image.height() - height, width, height);
            self.render_group(group, &mut *sub_image)?;

            start_x += width + self.group_gap();
        }

        Ok(())
    }

    fn render_group<I: GenericImage<Pixel = Rgba<u8>>>(
        &self,
        group: &HandGroup,
        image: &mut I,
    ) -> Result<(), HandRenderError> {
        let mut start_x = 0;
        let mut last_placement = Normal;
        for tile in group {
            let (width, height) = self.calculate_tile_size(tile);
            if last_placement == Rotated && tile.placement == RotatedAndShifted {
                start_x -= width + self.tile_gap();
            }

            let mut sub_image =
                imageops::crop(image, start_x, image.height() - height, width, height);
            self.render_tile(tile, &mut *sub_image)?;

            last_placement = tile.placement;
            start_x += width + self.tile_gap();
        }

        Ok(())
    }

    fn render_tile<I: GenericImage<Pixel = Rgba<u8>>>(
        &self,
        tile: &HandTile,
        image: &mut I,
    ) -> Result<(), HandRenderError> {
        let tile_image = self.tile_set.tile_image(tile)?;
        imageops::overlay(image, &tile_image, 0, 0);

        Ok(())
    }

    fn calculate_image_size(&self, hand: &Hand) -> (u32, u32) {
        hand.groups()
            .iter()
            .map(|group| self.calculate_group_size(group))
            .reduce(|(w1, h1), (w2, h2)| (w1 + w2 + self.group_gap(), h1.max(h2)))
            .unwrap_or((0, 0))
    }

    fn calculate_group_size(&self, group: &HandGroup) -> (u32, u32) {
        group
            .iter()
            .map(|tile| (tile.placement, self.calculate_tile_size(tile)))
            .reduce(|(placement_1, (w1, h1)), (placement_2, (w2, h2))| {
                let width = if placement_1 == Rotated && placement_2 == RotatedAndShifted {
                    w1
                } else {
                    w1 + w2 + self.tile_gap()
                };
                (placement_2, (width, h1.max(h2)))
            })
            .unwrap_or((Normal, (0, 0)))
            .1
    }

    #[inline]
    fn calculate_tile_size(&self, tile: &HandTile) -> (u32, u32) {
        let width = self.tile_set.tile_width();
        let height = self.tile_set.tile_height();

        match tile.placement {
            Normal => (width, height),
            Rotated => (height, width),
            RotatedAndShifted => (height, 2 * width),
        }
    }

    fn group_gap(&self) -> u32 {
        (self.options.group_gap.0 * self.tile_set.tile_width() as f32) as u32
    }

    fn tile_gap(&self) -> u32 {
        (self.options.tile_gap.0 * self.tile_set.tile_width() as f32) as u32
    }
}

#[derive(Clone, Debug)]
/// An error that occurs when calling [RasterRenderer::render].
pub enum HandRenderError {
    /// Error occurred when retrieving a tile image..
    TileImageRetrieveError(TileImageRetrieveError),
}

impl Error for HandRenderError {}

impl Display for HandRenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TileImageRetrieveError(inner_error) => {
                write!(f, "could not retrieve tile image: {}", inner_error)
            }
        }
    }
}

impl From<TileImageRetrieveError> for HandRenderError {
    fn from(error: TileImageRetrieveError) -> Self {
        Self::TileImageRetrieveError(error)
    }
}

#[cfg(test)]
mod tests {
    use image::{ImageFormat, RgbaImage};

    use crate::TilePlacement::{Normal, Rotated, RotatedAndShifted};
    #[cfg(feature = "fluffy-stuff-tile-sets")]
    use crate::raster_renderer::fluffy_stuff_tile_sets::YELLOW_FLUFFY_STUFF_TILE_SET;
    #[cfg(feature = "martin-persson-tile-sets")]
    use crate::raster_renderer::martin_persson_tile_sets::MARTIN_PERSSON_TILE_SET;
    use crate::raster_renderer::renderer::{RasterRenderer, RenderOptions, TileWidthRatio};
    use crate::tiles::*;
    use crate::{Hand, HandTile};

    #[cfg(feature = "fluffy-stuff-tile-sets")]
    #[test]
    fn should_render_hand_with_fluffy_stuff_tile_set() {
        let buffer = RasterRenderer::render(
            &get_test_hand(),
            &*YELLOW_FLUFFY_STUFF_TILE_SET,
            RenderOptions::new(TileWidthRatio(0.1), TileWidthRatio(0.5)),
        )
        .unwrap();

        let expected = load_expected_image(include_bytes!("expected_render_fluffy_stuff.png"));

        // assert! instead of assert_eq! to avoid lengthy error messages containing
        // diffs
        assert!(buffer == expected, "actual and expected images differ");
    }

    #[cfg(feature = "martin-persson-tile-sets")]
    #[test]
    fn should_render_hand_with_martin_persson_tile_set() {
        let buffer = RasterRenderer::render(
            &get_test_non_rotated_hand(),
            &*MARTIN_PERSSON_TILE_SET,
            RenderOptions::new(TileWidthRatio(0.1), TileWidthRatio(0.5)),
        )
        .unwrap();

        let expected = load_expected_image(include_bytes!("expected_render_martin_persson.png"));

        // assert! instead of assert_eq! to avoid lengthy error messages containing
        // diffs
        assert!(buffer == expected, "actual and expected images differ");
    }

    #[cfg(feature = "martin-persson-tile-sets")]
    #[test]
    fn should_fail_render_hand_with_martin_persson_tile_set() {
        let error = RasterRenderer::render(
            &get_test_hand(),
            &*MARTIN_PERSSON_TILE_SET,
            RenderOptions::new(TileWidthRatio(0.1), TileWidthRatio(0.5)),
        )
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "could not retrieve tile image: tile rotated Ryan man not supported: this tile set does not support rotated tiles"
        );
    }

    fn load_expected_image(expected_file: &[u8]) -> RgbaImage {
        image::load_from_memory_with_format(expected_file, ImageFormat::Png)
            .expect("could not load expected image")
            .to_rgba8()
    }

    fn get_test_hand() -> Hand {
        Hand::new(vec![
            vec![
                HandTile::new(II_MAN, Normal),
                HandTile::new(RYAN_MAN, Rotated),
                HandTile::new(SAN_MAN, RotatedAndShifted),
                HandTile::new(SUU_MAN, Normal),
            ],
            vec![
                HandTile::new(ANY, Normal),
                HandTile::new(ANY, Rotated),
                HandTile::new(II_PIN, Normal),
                HandTile::new(II_SOU, RotatedAndShifted),
                HandTile::new(TON, RotatedAndShifted),
                HandTile::new(NAN, Normal),
            ],
            vec![],
            vec![HandTile::new(UU_MAN, Normal)],
        ])
    }

    fn get_test_non_rotated_hand() -> Hand {
        Hand::new(vec![
            vec![
                HandTile::new(II_MAN, Normal),
                HandTile::new(RYAN_MAN, Normal),
                HandTile::new(SAN_MAN, Normal),
                HandTile::new(SUU_MAN, Normal),
            ],
            vec![
                HandTile::new(ANY, Normal),
                HandTile::new(ANY, Normal),
                HandTile::new(II_PIN, Normal),
                HandTile::new(II_SOU, Normal),
                HandTile::new(TON, Normal),
                HandTile::new(NAN, Normal),
            ],
            vec![],
            vec![HandTile::new(UU_MAN, Normal)],
        ])
    }
}
