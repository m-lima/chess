const SIZE: u8 = 8;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Coordinate {
    col: u8,
    row: u8,
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "r: {}, c: {}", self.row, self.col)
    }
}

impl std::convert::From<(u8, u8)> for Coordinate {
    fn from(pair: (u8, u8)) -> Self {
        Self::from_pair(pair)
    }
}

impl std::convert::From<u8> for Coordinate {
    fn from(index: u8) -> Self {
        Self::from_index(index)
    }
}

impl std::convert::Into<(u8, u8)> for Coordinate {
    fn into(self) -> (u8, u8) {
        self.into_pair()
    }
}

impl std::convert::Into<u8> for Coordinate {
    fn into(self) -> u8 {
        self.into_index()
    }
}

impl Coordinate {
    pub fn new(col: u8, row: u8) -> Self {
        assert!(col < SIZE && row < SIZE);
        Self { col, row }
    }

    pub fn from_pair(pair: (u8, u8)) -> Self {
        Self::new(pair.0, pair.1)
    }

    pub fn from_index(index: u8) -> Self {
        assert!(index < SIZE * SIZE);
        Self {
            row: index / SIZE,
            col: index % SIZE,
        }
    }

    pub fn into_pair(self) -> (u8, u8) {
        (self.col, self.row)
    }

    pub fn into_index(self) -> u8 {
        self.col + self.row * 8
    }

    pub fn col(self) -> u8 {
        self.col
    }

    pub fn row(self) -> u8 {
        self.row
    }

    pub fn add_col(self, amount: i8) -> Self {
        let col = self.col as i8 + amount;
        assert!(col >= 0 && col < SIZE as i8);
        Self::new(col as u8, self.row)
    }

    pub fn add_row(self, amount: i8) -> Self {
        let row = self.row as i8 + amount;
        assert!(row >= 0 && row < SIZE as i8);
        Self::new(self.col, row as u8)
    }

    pub fn with_col(self, col: u8) -> Self {
        Self::new(col, self.row)
    }

    pub fn with_row(self, row: u8) -> Self {
        Self::new(self.col, row)
    }
}

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
    pub fn new(color: Color, token: Token) -> Self {
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

    pub fn get(&self, coordinate: Coordinate) -> Option<Cell> {
        self.cells[usize::from(coordinate.into_index())]
    }

    pub fn set(&mut self, coordinate: Coordinate, cell: Option<Cell>) {
        self.cells[usize::from(coordinate.into_index())] = cell
    }
}

#[cfg(test)]
mod tests {
    mod coordinate {
        use super::super::Coordinate;
        #[test]
        fn from_pair() {
            assert_eq!(Coordinate::from((3, 7)), Coordinate::new(3, 7));
        }

        #[test]
        fn from_index() {
            assert_eq!(Coordinate::from(27), Coordinate::new(3, 3));
        }

        #[test]
        fn into_pair() {
            let pair: (u8, u8) = Coordinate::new(2, 7).into();
            assert_eq!(pair, (2, 7));
        }

        #[test]
        fn into_index() {
            let index: u8 = Coordinate::new(3, 3).into();
            assert_eq!(index, 27);
        }

        #[test]
        fn pair_roundtrip() {
            let pair = (3, 7);
            let other: (u8, u8) = Coordinate::from(pair).into();
            assert_eq!(pair, other);
        }

        #[test]
        fn index_roundtrip() {
            let index = 27;
            let other: u8 = Coordinate::from(index).into();
            assert_eq!(index, other);
        }

        #[test]
        fn with() {
            let c = Coordinate::new(0, 0);
            assert_eq!(c.with_col(1), Coordinate::new(1, 0));
            assert_eq!(c.with_row(1), Coordinate::new(0, 1));
        }

        #[test]
        fn add() {
            let c = Coordinate::new(0, 0);
            assert_eq!(c.add_col(1), Coordinate::new(1, 0));
            assert_eq!(c.add_row(1), Coordinate::new(0, 1));
        }

        mod bounds {
            use super::Coordinate;
            #[test]
            #[should_panic]
            fn from_index() {
                Coordinate::from(64);
            }

            #[test]
            #[should_panic]
            fn from_large_col() {
                Coordinate::new(8, 0);
            }

            #[test]
            #[should_panic]
            fn from_large_row() {
                Coordinate::new(0, 8);
            }

            #[test]
            #[should_panic]
            fn add_col() {
                let c = Coordinate::new(0, 0);
                c.add_col(8);
            }

            #[test]
            #[should_panic]
            fn add_row() {
                let c = Coordinate::new(0, 0);
                c.add_row(8);
            }

            #[test]
            #[should_panic]
            fn with_col() {
                let c = Coordinate::new(0, 0);
                c.with_col(8);
            }

            #[test]
            #[should_panic]
            fn with_row() {
                let c = Coordinate::new(0, 0);
                c.with_row(8);
            }
        }
    }
}
