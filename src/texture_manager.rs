use std::collections::HashMap;
use std::path::PathBuf;
use piston_window::*;
use gfx_device_gl::Resources;
use find_folder::Search;
use image as im;
use image::DynamicImage;
use map::BlockType;
use map::Color;

pub struct TextureManager {
    textures: HashMap<String, Texture<Resources>>,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    pub fn load_from_image(&mut self, name: &str, img: DynamicImage, window: &PistonWindow) {
        let asset = Texture::from_image(
            &mut *window.factory.borrow_mut(),
            &img.to_rgba(),
            &TextureSettings::new()).unwrap();

        self.textures.insert(name.to_string(), asset);
    }

    pub fn load_block_from_image(&mut self, blockType: BlockType, color: Color, img: DynamicImage, window: &PistonWindow) {
        let name = format!("block_{}_{}", blockType as i32, color as i32);

        let asset = Texture::from_image(
            &mut *window.factory.borrow_mut(),
            &img.to_rgba(),
            &TextureSettings::new()).unwrap();

        self.textures.insert(name, asset);
    }

    pub fn load_from_buffer(&mut self, name: &str, buffer: &[u8], window: &PistonWindow) {
        let img = im::load_from_memory(buffer).unwrap();
        let asset = Texture::from_image(
            &mut *window.factory.borrow_mut(),
            &img.to_rgba(),
            &TextureSettings::new()).unwrap();

        self.textures.insert(name.to_string(), asset);
    }

    pub fn get(&self, name: &str) -> &Texture<Resources> {
        self.textures.get(&name.to_string()).unwrap()
    }

    pub fn get_block(&self, blockType: BlockType, color: Color) -> &Texture<Resources> {
        let name = format!("block_{}_{}", blockType as i32, color as i32);

        self.textures.get(&name).unwrap()
    }
}
