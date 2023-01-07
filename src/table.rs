use crate::args::Color;

mod draw;

pub use draw::draw_table;

#[derive(Debug)]
pub struct DrawOptions {
    pub color: Color,
    pub flip: bool,
}

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub max_width: usize,
}

#[derive(Debug)]
pub struct Cell {
    pub content: String,
    pub cell_type: CellType,
}

#[derive(Debug)]
pub enum CellType {
    Null,
    Bool,
    Number,
    String,
    Collapsed,
}

impl Cell {
    pub fn null(content: String) -> Self {
        Self {
            content,
            cell_type: CellType::Null,
        }
    }

    pub fn bool(content: String) -> Self {
        Self {
            content,
            cell_type: CellType::Bool,
        }
    }

    pub fn number(content: String) -> Self {
        Self {
            content,
            cell_type: CellType::Number,
        }
    }

    pub fn string(content: String) -> Self {
        Self {
            content,
            cell_type: CellType::String,
        }
    }

    pub fn collapsed(content: String) -> Self {
        Self {
            content,
            cell_type: CellType::Collapsed,
        }
    }
}
