use super::board::{Board, Color, Coordinate, Token};

pub fn valid_moves(coordinate: Coordinate, board: &Board) -> Vec<Coordinate> {
    if let Some(ref cell) = board.get(coordinate) {
        match cell.token() {
            Token::Pawn => match cell.color() {
                Color::Black => {
                    if coordinate.row() == 7 {
                        vec![]
                    } else if coordinate.row() == 1 {
                        vec![coordinate.add_row(1), coordinate.add_row(2)]
                    } else {
                        vec![coordinate.add_row(1)]
                    }
                }
                Color::White => {
                    if coordinate.row() == 0 {
                        vec![]
                    } else if coordinate.row() == 6 {
                        vec![coordinate.add_row(-1), coordinate.add_row(-2)]
                    } else {
                        vec![coordinate.add_row(-1)]
                    }
                }
            },
            _ => vec![],
        }
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::super::board::Cell;
    use super::valid_moves;
    use super::{Board, Color, Coordinate, Token};

    mod pawn {
        use super::valid_moves;
        use super::{Board, Cell, Color, Coordinate, Token};

        #[test]
        fn normal_move() {
            let mut board = Board::new();
            board.set(
                Coordinate::new(3, 2),
                Some(Cell::new(Color::Black, Token::Pawn)),
            );
            board.set(
                Coordinate::new(5, 5),
                Some(Cell::new(Color::White, Token::Pawn)),
            );

            assert_eq!(
                valid_moves(Coordinate::new(3, 2), &board),
                vec![Coordinate::new(3, 3)]
            );

            assert_eq!(
                valid_moves(Coordinate::new(5, 5), &board),
                vec![Coordinate::new(5, 4)]
            );
        }

        #[test]
        fn initial_move() {
            let board = Board::new();

            assert_eq!(
                valid_moves(Coordinate::new(3, 1), &board),
                vec![Coordinate::new(3, 2), Coordinate::new(3, 3)]
            );

            assert_eq!(
                valid_moves(Coordinate::new(5, 6), &board),
                vec![Coordinate::new(5, 5), Coordinate::new(5, 4)]
            );
        }

        #[test]
        fn final_move() {
            let mut board = Board::new();
            board.set(
                Coordinate::new(3, 7),
                Some(Cell::new(Color::Black, Token::Pawn)),
            );
            board.set(
                Coordinate::new(5, 0),
                Some(Cell::new(Color::White, Token::Pawn)),
            );

            assert_eq!(valid_moves(Coordinate::new(3, 7), &board), vec![]);
            assert_eq!(valid_moves(Coordinate::new(5, 0), &board), vec![]);
        }
    }
}
