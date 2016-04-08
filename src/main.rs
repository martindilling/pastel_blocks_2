#![feature(slice_patterns)]

extern crate piston_window;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate image;

mod map;
mod game;
mod tilemap;
mod texture_manager;
mod ui;

use std::env;
use std::path::Path;
use std::path::PathBuf;
use map::Map;
use game::Game;

fn main() {
    let level_file_name = env::args().nth(1).unwrap_or("level_1".to_string());
    let path = Path::new("maps").join(level_file_name).with_extension("bmp");

    if !path.exists() {
        panic!("Couldn't find map: {:?}", path);
    }

    println!("Loading map: {:?}", path);
    let map = Map::from_bmp(&path);
    let mut game = Game::new(map);

    game.run();
}
