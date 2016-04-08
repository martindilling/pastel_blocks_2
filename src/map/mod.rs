pub mod block;

use std::path::Path;
use image;
use image::GenericImage;
use map::block::Block;

pub type Cells = Vec<Vec<Block>>;

#[derive(Clone)]
pub enum BlockType {None, Wall, Removable, Dangerous, Changer}

#[derive(Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
}

pub struct Map {
    width: usize,
    height: usize,
    cells: Cells,
}

impl Map {
    pub fn from_bmp(path: &Path) -> Self {
        let img = image::open(path).unwrap();
        let (w, h) = img.dimensions();
        let w = w as usize;
        let h = h as usize;

        let mut cells = vec![];
        for row in img.raw_pixels().chunks(w as usize * 3) {
            let mut cell_row = vec![];
            for rgb in row.chunks(3) {
                cell_row.push(match rgb {
                    [0,     0,   0] => Block::wall(),
                    [255, 255, 255] => Block::none(),
                    [127, 127, 127] => Block::none(),
                    [255,   0,   0] => Block::removable(Color::Red),
                    [0,   255,   0] => Block::removable(Color::Green),
                    [0,     0, 255] => Block::removable(Color::Blue),
                    [255, 255,   0] => Block::removable(Color::Yellow),
                    [255,   0, 255] => Block::removable(Color::Magenta),
                    [0,   255, 255] => Block::removable(Color::Cyan),
                    [255, 200, 200] => Block::changer(Color::Red),
                    [200, 255, 200] => Block::changer(Color::Green),
                    [200, 200, 255] => Block::changer(Color::Blue),
                    [255, 255, 200] => Block::changer(Color::Yellow),
                    [255, 200, 255] => Block::changer(Color::Magenta),
                    [200, 255, 255] => Block::changer(Color::Cyan),
                    [127,   0,   0] => Block::dangerous(Color::Red),
                    [0,   127,   0] => Block::dangerous(Color::Green),
                    [0,     0, 127] => Block::dangerous(Color::Blue),
                    [127, 127,   0] => Block::dangerous(Color::Yellow),
                    [127,   0, 127] => Block::dangerous(Color::Magenta),
                    [0,   127, 127] => Block::dangerous(Color::Cyan),
                    _ => Block::none(),
                });
            }
            cells.push(cell_row);
        }

        Map {
            width: w,
            height: h,
            cells: cells,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_cells(&self) -> &Cells {
        &self.cells
    }

    pub fn switch(&mut self) {
        for rows in self.cells.iter_mut() {
            for cell in rows.iter_mut() {
                cell.switch();
            }
        }
    }
}
