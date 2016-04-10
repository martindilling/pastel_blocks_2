use piston_window::*;
use map::Map;
use map::BlockType;
use map::Color;
use texture_manager::TextureManager;
use tilemap::Tilemap;
use image as im;
use ui::UI;


static TILEMAP_32: &'static [u8] = include_bytes!("../assets/tiles_32x32.png");
static TILEMAP_16: &'static [u8] = include_bytes!("../assets/tiles_16x16.png");


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

        let mut tilemap32 = Tilemap::new(im::load_from_memory(TILEMAP_32).unwrap(), 32, 32);
        let mut tilemap16 = Tilemap::new(im::load_from_memory(TILEMAP_16).unwrap(), 16, 16);


        let mut texture_manager = TextureManager::new();
        texture_manager.load_from_image("ball_white", tilemap16.get(0, 0), &window);

        texture_manager.load_from_image("bg_black", tilemap32.get(2, 3), &window);
        texture_manager.load_from_image("block_wall", tilemap32.get(0, 3), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Red, tilemap32.get(0, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Red, tilemap32.get(0, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Red, tilemap32.get(0, 2), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Green, tilemap32.get(1, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Green, tilemap32.get(1, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Green, tilemap32.get(1, 2), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Blue, tilemap32.get(2, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Blue, tilemap32.get(2, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Blue, tilemap32.get(2, 2), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Yellow, tilemap32.get(3, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Yellow, tilemap32.get(3, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Yellow, tilemap32.get(3, 2), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Magenta, tilemap32.get(4, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Magenta, tilemap32.get(4, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Magenta, tilemap32.get(4, 2), &window);
        texture_manager.load_block_from_image(BlockType::Removable, Color::Cyan, tilemap32.get(5, 0), &window);
        texture_manager.load_block_from_image(BlockType::Dangerous, Color::Cyan, tilemap32.get(5, 1), &window);
        texture_manager.load_block_from_image(BlockType::Changer, Color::Cyan, tilemap32.get(5, 2), &window);

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
                Some(Event::Update(ref update)) => {
                    self.handle_update(update, &e);
                }
                _ => {}
            }
        }
    }

    pub fn handle_update(&mut self, update: &UpdateArgs, window: &PistonWindow) {
        self.map.update(update.dt);
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
