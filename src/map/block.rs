use map::BlockType;
use map::Color;

pub struct Block {
    pub block_type: BlockType,
    pub color: Option<Color>,
}

impl Block {
    pub fn none() -> Self {
        Block {
            block_type: BlockType::None,
            color: None,
        }
    }

    pub fn is_none(&self) -> bool {
        match self.block_type {
            BlockType::None => true,
            _ => false,
        }
    }

    pub fn wall() -> Self {
        Block {
            block_type: BlockType::Wall,
            color: None,
        }
    }

    pub fn is_wall(&self) -> bool {
        match self.block_type {
            BlockType::Wall => true,
            _ => false,
        }
    }

    pub fn removable(color: Color) -> Self {
        Block {
            block_type: BlockType::Removable,
            color: Some(color),
        }
    }

    pub fn is_removable(&self) -> bool {
        match self.block_type {
            BlockType::Removable => true,
            _ => false,
        }
    }

    pub fn dangerous(color: Color) -> Self {
        Block {
            block_type: BlockType::Dangerous,
            color: Some(color),
        }
    }

    pub fn is_dangerous(&self) -> bool {
        match self.block_type {
            BlockType::Dangerous => true,
            _ => false,
        }
    }

    pub fn changer(color: Color) -> Self {
        Block {
            block_type: BlockType::Changer,
            color: Some(color),
        }
    }

    pub fn is_changer(&self) -> bool {
        match self.block_type {
            BlockType::Changer => true,
            _ => false,
        }
    }

    pub fn switch(&mut self) {
        match self {
            _ if self.is_removable() => {
                self.block_type = BlockType::Dangerous;
            }
            _ if self.is_dangerous() => {
                self.block_type = BlockType::Removable;
            }
            _ => {}
        }
    }
}
