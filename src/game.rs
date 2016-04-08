use piston_window::*;
use map::Map;
use map::BlockType;
use map::Color;
use texture_manager::TextureManager;
use tilemap::Tilemap;
use image as im;
use ui::UI;


static TILEMAP: &'static [u8] = include_bytes!("../assets/tiles_32x32.png");


pub struct Game {
    map: Map,
    ui: UI,
    window: PistonWindow
}

impl Game {
    pub fn new(map: Map) -> Game {
        let window: PistonWindow = WindowSettings::new("Pastel Blocks", [1024, 640])
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut tilemap = Tilemap::new(im::load_from_memory(TILEMAP).unwrap(), 32, 32);


        let mut texture_manager = TextureManager::new();
        texture_manager.load_from_buffer("tilemap", TILEMAP, &window);
        texture_manager.load_from_image("bg_black", tilemap.get(2, 3), &window);
        texture_manager.load_from_image("block_wall", tilemap.get(0, 3), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Red, tilemap.get(0, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Red, tilemap.get(0, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Red, tilemap.get(0, 2), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Green, tilemap.get(1, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Green, tilemap.get(1, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Green, tilemap.get(1, 2), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Blue, tilemap.get(2, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Blue, tilemap.get(2, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Blue, tilemap.get(2, 2), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Yellow, tilemap.get(3, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Yellow, tilemap.get(3, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Yellow, tilemap.get(3, 2), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Magenta, tilemap.get(4, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Magenta, tilemap.get(4, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Magenta, tilemap.get(4, 2), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Cyan, tilemap.get(5, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Cyan, tilemap.get(5, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Cyan, tilemap.get(5, 2), &window);

        let ui = UI::new(texture_manager);

        Game {
            map: map,
            ui: ui,
            window: window,
        }
    }

    pub fn run(&mut self) {
        for e in self.window.clone() {
            match e.event {
                Some(Event::Render(ref render)) => {
                    self.ui.draw(&e, &self.map);
                }
                Some(Event::Input(ref input)) => {
                    self.handle_input(input);
                }
                _ => {}
            }
        }
    }

    pub fn handle_input(&mut self, input: &Input) {
        match input {
            &Input::Press(button) => {
                match button {
                    Button::Keyboard(Key::Space) => {
                        println!("Pressed Space");
                        self.map.switch();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
