#[derive(Debug, Clone, Copy)]
pub enum Token {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

impl std::fmt::Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Self::Pawn => fmt.write_char('P'),
            Self::Knight => fmt.write_char('N'),
            Self::Rook => fmt.write_char('R'),
            Self::Bishop => fmt.write_char('B'),
            Self::Queen => fmt.write_char('Q'),
            Self::King => fmt.write_char('K'),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl std::fmt::Display for Color {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Self::Black => fmt.write_char('B'),
            Self::White => fmt.write_char('W'),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    color: Color,
    token: Token,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}{}", self.color, self.token)
    }
}

impl Cell {
    fn new(color: Color, token: Token) -> Self {
        Self { color, token }
    }

    pub fn color(self) -> Color {
        self.color
    }

    pub fn token(self) -> Token {
        self.token
    }
}

#[derive(Clone)]
pub struct Board {
    cells: [Option<Cell>; 64],
}

impl std::fmt::Display for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for (index, cell) in self.cells.iter().enumerate() {
            if let Some(cell) = cell {
                write!(fmt, "{}", cell)?;
            } else {
                write!(fmt, "  ")?;
            }

            if index > 0 && index % 8 == 0 {
                fmt.write_char('\n')?;
            }
        }
        Ok(())
    }
}

impl Board {
    pub fn new() -> Self {
        use Color::{Black, White};
        use Token::{Bishop, King, Knight, Pawn, Queen, Rook};
        Self {
            cells: [
                Some(Cell::new(Black, Rook)),
                Some(Cell::new(Black, Knight)),
                Some(Cell::new(Black, Bishop)),
                Some(Cell::new(Black, Queen)),
                Some(Cell::new(Black, King)),
                Some(Cell::new(Black, Bishop)),
                Some(Cell::new(Black, Knight)),
                Some(Cell::new(Black, Rook)),
                Some(Cell::new(Black, Pawn)),
                Some(Cell::new(Black, Pawn)),
                Some(Cell::new(Black, Pawn)),
                Some(Cell::new(Black, Pawn)),
                Some(Cell::new(Black, Pawn)),
                Some(Cell::new(Black, Pawn)),
                Some(Cell::new(Black, Pawn)),
                Some(Cell::new(Black, Pawn)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Cell::new(White, Pawn)),
                Some(Cell::new(White, Pawn)),
                Some(Cell::new(White, Pawn)),
                Some(Cell::new(White, Pawn)),
                Some(Cell::new(White, Pawn)),
                Some(Cell::new(White, Pawn)),
                Some(Cell::new(White, Pawn)),
                Some(Cell::new(White, Pawn)),
                Some(Cell::new(White, Rook)),
                Some(Cell::new(White, Knight)),
                Some(Cell::new(White, Bishop)),
                Some(Cell::new(White, Queen)),
                Some(Cell::new(White, King)),
                Some(Cell::new(White, Bishop)),
                Some(Cell::new(White, Knight)),
                Some(Cell::new(White, Rook)),
            ],
        }
    }

    pub fn cells(&self) -> &[Option<Cell>] {
        &self.cells
    }
}
