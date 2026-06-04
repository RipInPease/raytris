use raylib::prelude::*;

pub const I_PIECE_COLOR: Color = Color::DARKTURQUOISE;
pub const J_PIECE_COLOR: Color = Color::BLUE;
pub const L_PIECE_COLOR: Color = Color::ORANGE;
pub const O_PIECE_COLOR: Color = Color::YELLOW;
pub const S_PIECE_COLOR: Color = Color::GREEN;
pub const T_PIECE_COLOR: Color = Color::PURPLE;
pub const Z_PIECE_COLOR: Color = Color::RED;

/// Represents a tetris piece
pub struct TetrisPiece {

}

/// All the types a [`TetrisPiece`] can be
pub enum Tetrimino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z
}

impl Tetrimino {
    /// Draws a tetrimino using raylib
    pub fn draw<D: RaylibDraw>
    (
        &self,
        d: &mut D,
        x_pos: i32,
        y_pos: i32,
        tile_width: i32,
        tile_height: i32,
        rotation: Rotation
    )
    {
        let (cells, color) = match self {
            Self::I => (Self::i_cells(rotation), I_PIECE_COLOR),
            Self::J => (Self::j_cells(rotation), J_PIECE_COLOR),
            Self::L => (Self::l_cells(rotation), L_PIECE_COLOR),
            Self::O => (Self::o_cells(rotation), O_PIECE_COLOR),
            Self::T => (Self::t_cells(rotation), T_PIECE_COLOR),
            _ => ([[false; 4]; 4], I_PIECE_COLOR)
        };

        for x in 0..cells.len() {
            for y in 0..cells[x].len() {
                if cells[x][y] {
                    d.draw_rectangle(
                        x_pos + x as i32 * tile_width, 
                        y_pos + y as i32 * tile_height, 
                        tile_width, 
                        tile_height, 
                        color,
                    );
                }
            }
        }
    }

    /// The cells to draw an I-piece
    fn i_cells
    (rotation: Rotation) -> [[bool; 4]; 4] {
        let mut cells = [[false; 4]; 4];
        match rotation {
            Rotation::None => {
                cells[0][1] = true;
                cells[1][1] = true;
                cells[2][1] = true;
                cells[3][1] = true;
            }
            Rotation::Ninety => {
                cells[2][0] = true;
                cells[2][1] = true;
                cells[2][2] = true;
                cells[2][3] = true;
            }
            Rotation::OneEighty => {
                cells[0][2] = true;
                cells[1][2] = true;
                cells[2][2] = true;
                cells[3][2] = true;
            }
            Rotation::TwoSeventy => {
                cells[1][0] = true;
                cells[1][1] = true;
                cells[1][2] = true;
                cells[1][3] = true;
            }
        }

        cells
    }

    /// The cells to draw a J-piece
    fn j_cells
    (rotation: Rotation) -> [[bool; 4]; 4] {
        let mut cells = [[false; 4]; 4];
        match rotation {
            Rotation::None => {
                cells[0][0] = true;
                cells[0][1] = true;
                cells[1][1] = true;
                cells[2][1] = true;
            }
            Rotation::Ninety => {
                cells[1][0] = true;
                cells[2][0] = true;
                cells[1][1] = true;
                cells[1][2] = true;
            }
            Rotation::OneEighty => {
                cells[0][1] = true;
                cells[1][1] = true;
                cells[2][1] = true;
                cells[2][2] = true;
            }
            Rotation::TwoSeventy => {
                cells[1][0] = true;
                cells[1][1] = true;
                cells[1][2] = true;
                cells[0][2] = true;
            }
        }

        cells
    }

    /// The cells to draw a L-piece
    fn l_cells
    (rotation: Rotation) -> [[bool; 4]; 4] {
        let mut cells = [[false; 4]; 4];
        match rotation {
            Rotation::None => {
                cells[0][1] = true;
                cells[1][1] = true;
                cells[2][1] = true;
                cells[2][0] = true;
            }
            Rotation::Ninety => {
                cells[1][0] = true;
                cells[1][1] = true;
                cells[1][2] = true;
                cells[2][2] = true;
            }
            Rotation::OneEighty => {
                cells[0][1] = true;
                cells[1][1] = true;
                cells[2][1] = true;
                cells[0][2] = true;
            }
            Rotation::TwoSeventy => {
                cells[0][0] = true;
                cells[1][0] = true;
                cells[1][1] = true;
                cells[1][2] = true;
            }
        }

        cells
    }

    /// The cells to draw a O-piece
    fn o_cells
    (rotation: Rotation) -> [[bool; 4]; 4] {
        let mut cells = [[false; 4]; 4];
        match rotation {
            _ => {
                cells[1][0] = true;
                cells[2][0] = true;
                cells[1][1] = true;
                cells[2][1] = true;
            }
        }

        cells
    }

    /// The cells to draw a T-piece
    fn t_cells
    (rotation: Rotation) -> [[bool; 4]; 4] {
        let mut cells = [[false; 4]; 4];
        match rotation {
            Rotation::None => {
                cells[1][0] = true;
                cells[0][1] = true;
                cells[1][1] = true;
                cells[2][1] = true;
            }
            Rotation::Ninety => {
                cells[1][0] = true;
                cells[1][1] = true;
                cells[2][1] = true;
                cells[1][2] = true;
            }
            Rotation::OneEighty => {
                cells[0][1] = true;
                cells[1][1] = true;
                cells[2][1] = true;
                cells[1][2] = true;
            }
            Rotation::TwoSeventy => {
                cells[1][0] = true;
                cells[0][1] = true;
                cells[1][1] = true;
                cells[1][2] = true;
            }
        }

        cells
    }
}

/// Which way a [`TetrisPiece`] is rotated
pub enum Rotation {
    None,
    Ninety,
    OneEighty,
    TwoSeventy,
}

impl Rotation {
    /// Rotates ninety degrees clockwise
    pub fn rotate_cw(&mut self) {
        match self {
            Self::None       => *self = Self::Ninety,
            Self::Ninety     => *self = Self::OneEighty,
            Self::OneEighty  => *self = Self::TwoSeventy,
            Self::TwoSeventy => *self = Self::None,
        }
    }

    /// Rotates ninety degrees counter-clockwise
    pub fn rotate_ccw(&mut self) {
        match self {
            Self::None       => *self = Self::TwoSeventy,
            Self::TwoSeventy => *self = Self::OneEighty,
            Self::OneEighty  => *self = Self::Ninety,
            Self::Ninety     => *self = Self::None,
        }
    }
}