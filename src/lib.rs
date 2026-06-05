use raylib::prelude::*;

pub const I_PIECE_COLOR: Color = Color::DARKTURQUOISE;
pub const J_PIECE_COLOR: Color = Color::BLUE;
pub const L_PIECE_COLOR: Color = Color::ORANGE;
pub const O_PIECE_COLOR: Color = Color::YELLOW;
pub const S_PIECE_COLOR: Color = Color::GREEN;
pub const T_PIECE_COLOR: Color = Color::PURPLE;
pub const Z_PIECE_COLOR: Color = Color::RED;

/// The entire board. Including the already placed piecec, the active piece and the held piece
pub struct Board {
    active_piece: TetrisPiece,
    held_piece  : Option<Tetrimino>,
    bag         : Bag,
    grid        : [Option<Color>; 10*20],
}

impl Board {
    /// Pulls the next piece from the bag and sets it as the active piece
    pub fn next_piece(&mut self) {
        let next = TetrisPiece { 
            tetrimino: self.bag.next().unwrap(), 
            rotation: Rotation::None 
        };

        self.active_piece = next;
    }

    /// Hold the active piece and swaps with the current held piece
    pub fn hold_piece(&mut self) {
        let swap = match self.held_piece {
            Some(t) => t,
            None    => self.bag.next().unwrap(),
        };

        self.held_piece = Some(self.active_piece.tetrimino);

        self.active_piece = TetrisPiece { 
            tetrimino: swap, 
            rotation: Rotation::None 
        }
    }
}

/// Represents a tetris piece
#[derive(Debug, Clone)]
pub struct TetrisPiece {
    tetrimino   : Tetrimino,
    rotation    : Rotation, 
}

impl TetrisPiece {
    /// The cells in a 4x4 grid a piece occupies
    pub fn occupied_cells(&self) -> [[bool; 4]; 4] {
        match self.tetrimino {
            Tetrimino::I => Tetrimino::i_cells(self.rotation),
            Tetrimino::J => Tetrimino::j_cells(self.rotation),
            Tetrimino::L => Tetrimino::l_cells(self.rotation),
            Tetrimino::O => Tetrimino::o_cells(self.rotation),
            Tetrimino::S => Tetrimino::s_cells(self.rotation),
            Tetrimino::T => Tetrimino::t_cells(self.rotation),
            Tetrimino::Z => Tetrimino::z_cells(self.rotation),
        }
    }
}

/// All the types a [`TetrisPiece`] can be
#[derive(Debug, Clone, Copy)]
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
            Self::S => (Self::s_cells(rotation), S_PIECE_COLOR),
            Self::T => (Self::t_cells(rotation), T_PIECE_COLOR),
            Self::Z => (Self::z_cells(rotation), Z_PIECE_COLOR),
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
    fn i_cells(rotation: Rotation) -> [[bool; 4]; 4] {
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
    fn j_cells(rotation: Rotation) -> [[bool; 4]; 4] {
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

    /// The cells to draw an L-piece
    fn l_cells(rotation: Rotation) -> [[bool; 4]; 4] {
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

    /// The cells to draw an O-piece
    fn o_cells(rotation: Rotation) -> [[bool; 4]; 4] {
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

    /// The cells to draw an S-piece
    fn s_cells(rotation: Rotation) -> [[bool; 4]; 4] {
        let mut cells = [[false; 4]; 4];
        match rotation {
            Rotation::None => {
                cells[1][0] = true;
                cells[2][0] = true;
                cells[0][1] = true;
                cells[1][1] = true;
            }
            Rotation::Ninety => {
                cells[1][0] = true;
                cells[1][1] = true;
                cells[2][1] = true;
                cells[2][2] = true;
            }
            Rotation::OneEighty => {
                cells[1][1] = true;
                cells[2][1] = true;
                cells[0][2] = true;
                cells[1][2] = true;
            }
            Rotation::TwoSeventy => {
                cells[0][0] = true;
                cells[0][1] = true;
                cells[1][1] = true;
                cells[1][2] = true;
            }
        }

        cells
    }

    /// The cells to draw a T-piece
    fn t_cells(rotation: Rotation) -> [[bool; 4]; 4] {
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

    /// The cells to draw a Z-piece
    fn z_cells(rotation: Rotation) -> [[bool; 4]; 4] {
        let mut cells = [[false; 4]; 4];
        match rotation {
            Rotation::None => {
                cells[0][0] = true;
                cells[1][0] = true;
                cells[1][1] = true;
                cells[2][1] = true;
            }
            Rotation::Ninety => {
                cells[2][0] = true;
                cells[1][1] = true;
                cells[2][1] = true;
                cells[1][2] = true;
            }
            Rotation::OneEighty => {
                cells[0][1] = true;
                cells[1][1] = true;
                cells[1][2] = true;
                cells[2][2] = true;
            }
            Rotation::TwoSeventy => {
                cells[1][0] = true;
                cells[0][1] = true;
                cells[1][1] = true;
                cells[0][2] = true;
            }
        }

        cells
    }
}

/// Which way a [`TetrisPiece`] is rotated
#[derive(Debug, Copy, Clone)]
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

impl TryFrom<usize> for Rotation {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Ninety),
            2 => Ok(Self::OneEighty),
            3 => Ok(Self::TwoSeventy),
            _ => Err(()),
        }
    }
}

/// What which generates the next pieces
struct Bag {
    i       : usize,
    pieces  : [Tetrimino; 7]
}

impl Bag {
    fn new() -> Self {
        let mut new = Self {
            i: 0,
            pieces: [Tetrimino::I; 7]
        };

        new.regenerate();
        new
    }

    /// Regenerates the bag
    fn regenerate(&mut self) {
        use rand::random_range;
        self.i = 0;

        let mut new_bag = vec![
            Tetrimino::I,
            Tetrimino::J,
            Tetrimino::L,
            Tetrimino::O,
            Tetrimino::S,
            Tetrimino::T,
            Tetrimino::Z,
        ];

        for i in 0..self.pieces.len() {
            let new_i = random_range(0..new_bag.len());
            self.pieces[i] = new_bag.remove(new_i); 
        }
    }
}

impl Iterator for Bag {
    type Item = Tetrimino;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= 7 {
            self.regenerate();
        }

        let piece = self.pieces[self.i];
        self.i += 1;
        Some(piece)
    }
}