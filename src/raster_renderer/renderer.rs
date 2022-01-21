use image::{imageops, GenericImage, ImageBuffer, Rgba, RgbaImage};

use crate::raster_renderer::tile_set::TileSet;
use crate::TilePlacement::{Normal, Rotated, RotatedAndShifted};
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

type ImageType = RgbaImage;

impl<'a, T: TileSet> RasterRenderer<'a, T> {
    #[inline]
    /// Renders given [Hand] instance using [TileSet] and [RenderOptions].
    pub fn render(hand: &Hand, tile_set: &'a T, options: RenderOptions) -> ImageType {
        Self::new(tile_set, options).render_internal(hand)
    }

    #[inline]
    fn new(tile_set: &'a T, options: RenderOptions) -> Self {
        Self { tile_set, options }
    }

    fn render_internal(&self, hand: &Hand) -> ImageType {
        let (width, height) = self.calculate_image_size(hand);
        let mut image = ImageBuffer::new(width, height);

        self.render_hand(hand, &mut image);

        image
    }

    fn render_hand<I: GenericImage<Pixel = Rgba<u8>>>(&self, hand: &Hand, image: &mut I) {
        let mut start_x = 0;
        for group in hand.groups() {
            let (width, height) = self.calculate_group_size(group);
            let mut sub_image =
                imageops::crop(image, start_x, image.height() - height, width, height);
            self.render_group(group, &mut sub_image);

            start_x += width + self.group_gap();
        }
    }

    fn render_group<I: GenericImage<Pixel = Rgba<u8>>>(&self, group: &HandGroup, image: &mut I) {
        let mut start_x = 0;
        let mut last_placement = Normal;
        for tile in group {
            let (width, height) = self.calculate_tile_size(tile);
            if last_placement == Rotated && tile.placement == RotatedAndShifted {
                start_x -= width + self.tile_gap();
            }

            let mut sub_image =
                imageops::crop(image, start_x, image.height() - height, width, height);
            self.render_tile(tile, &mut sub_image);

            last_placement = tile.placement;
            start_x += width + self.tile_gap();
        }
    }

    fn render_tile<I: GenericImage<Pixel = Rgba<u8>>>(&self, tile: &HandTile, image: &mut I) {
        let tile_image = self.tile_set.tile_image(tile);
        image::imageops::overlay(image, &tile_image, 0, 0);
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

#[cfg(test)]
#[cfg(feature = "fluffy-stuff-tile-sets")]
mod tests {
    use image::ImageFormat;

    use crate::raster_renderer::fluffy_stuff_tile_sets::YELLOW_FLUFFY_STUFF_TILE_SET;
    use crate::raster_renderer::renderer::{RasterRenderer, RenderOptions, TileWidthRatio};
    use crate::tiles::*;
    use crate::TilePlacement::{Normal, Rotated, RotatedAndShifted};
    use crate::{Hand, HandTile};

    #[test]
    fn should_render_hand() {
        let hand = Hand::new(vec![
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
        ]);

        println!("{:?}", hand);
        let buffer = RasterRenderer::render(
            &hand,
            &*YELLOW_FLUFFY_STUFF_TILE_SET,
            RenderOptions::new(TileWidthRatio(0.1), TileWidthRatio(0.5)),
        );

        let expected = image::load_from_memory_with_format(
            include_bytes!("expected_render.png"),
            ImageFormat::Png,
        )
        .expect("could not load expected image")
        .to_rgba8();

        // assert! instead of assert_eq! to avoid lengthy error messages containing diffs
        assert!(buffer == expected, "actual and expected images differ");
    }
}
