use crate::piece::OrientedPiece;

#[derive(Debug, Clone)]
pub struct Board {
    table: [[char; 9]; 6],
}

impl Board {
    pub fn new() -> Board {
        let mut new_board = Board {
            table: [['0'; 9]; 6],
        };
        new_board.table[5][8] = 'X';

        new_board
    }

    pub fn is_full(&self) -> bool {
        for row in self.table {
            for space in row {
                if space == '0' {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn place_piece_on_top_left(&self, piece: &OrientedPiece, piece_id: char) -> Option<Board> {
        // first find the top-left empty space on the board
        let mut top_left: (usize, usize) = (0, 0);
        let num_rows: usize = self.table.len();
        let num_cols: usize = self.table[0].len();

        for i in 0..num_rows {
            for j in usize::from(piece.top_index)..num_cols {
                if self.table[j][i] == '0' {
                    top_left.0 = i;
                    top_left.1 = j;
                }
            }
        }

        let offset: (usize, usize) = (top_left.0 - usize::from(piece.top_index), top_left.1);

        let piece_rows: usize = piece.pattern.len();
        let piece_cols = piece.pattern[0].len();
        for i in 0..piece_rows {
            for j in 0..piece_cols {
                if self.table[offset.0 + j][offset.1 + i] != '0' && piece.pattern[j][i] != '0' {
                    return None;
                }
            }
        }

        let mut new_board = self.clone();
        for i in 0..piece_rows {
            for j in 0..piece_cols {
                new_board.table[offset.0 + j][offset.1 + i] = piece_id;
            }
        }

        Some(new_board)
    }

    pub fn remove_piece(&mut self, piece_id: char) {
        let num_rows: usize = self.table.len();
        let num_cols: usize = self.table[0].len();

        for i in 0..num_rows {
            for j in 0..num_cols {
                if self.table[j][i] == piece_id {
                    self.table[j][i] = '0';
                }
            }
        }
    }
}
