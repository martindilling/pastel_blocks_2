use piston_window::*;
use texture_manager::TextureManager;
use map::BlockType;
use map::Color;
use map::Map;
use map::block::Block;

pub struct UI {
    cell_size: f64,
    textures: TextureManager,
}

impl UI {
    pub fn new(textures: TextureManager) -> UI {
        UI {
            cell_size: 32.0,
            textures: textures,
        }
    }

    pub fn draw(&mut self, window: &PistonWindow, map: &Map) {
        window.draw_2d(|context, graphics| {
            clear([0.8, 0.8, 0.8, 1.0], graphics);

            for (y, rows) in map.get_cells().iter().enumerate() {
                for (x, cell) in rows.iter().enumerate() {
                    let position = context.transform.trans(
                        self.cell_to_position(x),
                        self.cell_to_position(y)
                    );

                    let texture = match cell {
                        _ if cell.is_none() => Some(self.textures.get("bg_black")),
                        _ if cell.is_wall() => Some(self.textures.get("block_wall")),
                        &Block { ref block_type, color: Some(ref c) , .. } => Some(self.textures.get_block(block_type.clone(), c.clone())),
                        _ => None,
                    };

                    match texture {
                        Some(_) => image(texture.unwrap(), position, graphics),
                        None => {},
                    }
                }
            }
        });
    }

    pub fn cell_to_position(&self, cell: usize) -> f64 {
        self.cell_size * cell as f64
    }
}
