use image::DynamicImage;
use image::GenericImage;
use image::RgbaImage;
use image::SubImage;

pub struct Tilemap {
    tilemap: DynamicImage,
    tile_width: u32,
    tile_height: u32,
}

impl Tilemap {
    pub fn new(tilemap: DynamicImage, tile_width: u32, tile_height: u32) -> Self {
        Tilemap {
            tilemap: tilemap,
            tile_width: tile_width,
            tile_height: tile_height,
        }
    }

    pub fn get(&mut self, x: u32, y: u32) -> DynamicImage {
        DynamicImage::ImageRgba8(
            self.tilemap.sub_image(
                x * self.tile_width,
                y * self.tile_width,
                self.tile_width,
                self.tile_height
            ).to_image()
        )
    }
}
