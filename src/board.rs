use std::fmt;

use crate::piece::OrientedPiece;

const NUM_LINES: usize = 6;
const NUM_COLUMNS: usize = 9;

#[derive(Debug, Clone)]
pub struct Board {
    table: [[char; NUM_COLUMNS]; NUM_LINES],
}

impl Board {
    pub fn new() -> Board {
        let mut new_board = Board {
            table: [['0'; NUM_COLUMNS]; NUM_LINES],
        };
        new_board.table[NUM_LINES - 1][NUM_COLUMNS - 1] = 'X';

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

    pub fn place_piece_on_top_left(&self, piece: &OrientedPiece) -> Option<Board> {
        // first find the top-left empty space on the board
        let mut top_left: (usize, usize) = (0, 0);

        '_outer : for i in 0..NUM_LINES {
            for j in usize::from(piece.top_index)..NUM_COLUMNS {
                if self.table[i][j] == '0' {
                    top_left.0 = i;
                    top_left.1 = j;
                    break '_outer;
                }
            }
        }

        let offset: (usize, usize) = (top_left.0 - usize::from(piece.top_index), top_left.1);

        let piece_rows: usize = piece.pattern.len();
        let piece_cols = piece.pattern[0].len();
        for i in 0..piece_rows {
            for j in 0..piece_cols {
                if self.table[offset.0 + i][offset.1 + j] != '0' && piece.pattern[i][j] != '0' {
                    return None;
                }
            }
        }

        let mut new_board = self.clone();
        for i in 0..piece_rows {
            for j in 0..piece_cols {
                if piece.pattern[i][j] != '0' {
                    new_board.table[offset.0 + i][offset.1 + j] = piece.pattern[i][j];
                }
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // first is top line going clockwise
        // 0 = nothing
        // 1 = thin line
        // 2 = thick line
        let mut lines: [[[usize; 4]; NUM_COLUMNS + 1]; NUM_LINES + 1] =
            [[[0; 4]; NUM_COLUMNS + 1]; NUM_LINES + 1];

        // neighbords start with top left going clockwise
        fn get_lines(neighbors: (char, char, char, char)) -> [usize; 4] {
            let mut result: [usize; 4] = [0; 4];

            if neighbors.0 != neighbors.1 {
                if neighbors.0 == 'X' || neighbors.1 == 'X' {
                    result[0] = 2;
                } else {
                    result[0] = 1;
                }
            }
            if neighbors.1 != neighbors.2 {
                if neighbors.1 == 'X' || neighbors.2 == 'X' {
                    result[1] = 2;
                } else {
                    result[1] = 1;
                }
            }
            if neighbors.2 != neighbors.3 {
                if neighbors.2 == 'X' || neighbors.3 == 'X' {
                    result[2] = 2;
                } else {
                    result[2] = 1;
                }
            }
            if neighbors.3 != neighbors.0 {
                if neighbors.3 == 'X' || neighbors.0 == 'X' {
                    result[3] = 2;
                } else {
                    result[3] = 1;
                }
            }

            result
        }

        for i in 0..NUM_LINES + 1 {
            for j in 0..NUM_COLUMNS + 1 {
                match (i, j) {
                    (0, 0) => { // top left
                        lines[i][j] = get_lines(('X', 'X', self.table[i][j],'X'));
                    }
                    (0, NUM_COLUMNS) => { // top right
                        lines[i][j] = get_lines(('X', 'X', 'X',self.table[i][j-1]));
                    }
                    (NUM_LINES, 0) => { // bottom left
                        lines[i][j] = get_lines(('X', self.table[i - 1][j], 'X', 'X'));
                    }
                    (NUM_LINES, NUM_COLUMNS) => { //bottom right
                        lines[i][j] = get_lines((self.table[i - 1][j - 1], 'X', 'X', 'X'));
                    }
                    (NUM_LINES, _) => { // bottom
                        lines[i][j] =
                            get_lines((self.table[i - 1][j - 1], self.table[i - 1][j], 'X', 'X'));
                    }
                    (0, _) => { // top
                        lines[i][j] = get_lines(('X', 'X', self.table[i][j], self.table[i][j - 1]));
                    }
                    (_, 0) => { // left
                        lines[i][j] = get_lines(('X', self.table[i - 1][j], self.table[i][j], 'X'));
                    }
                    (_, NUM_COLUMNS) => { // right
                        lines[i][j] =
                            get_lines((self.table[i - 1][j - 1], 'X', 'X', self.table[i][j - 1]));
                    }
                    _ => {
                        lines[i][j] = get_lines((
                            self.table[i - 1][j - 1],
                            self.table[i - 1][j],
                            self.table[i][j],
                            self.table[i][j - 1],
                        ));
                    }
                }
            }
        }
        // https://en.wikipedia.org/wiki/Box-drawing_characters

        //          0	1	2	3	4	5	6	7	8	9	A	B	C	D	E	F
        // U+250x	─	━	│	┃	┄	┅	┆	┇	┈	┉	┊	┋	┌	┍	┎	┏
        // U+251x	┐	┑	┒	┓	└	┕	┖	┗	┘	┙	┚	┛	├	┝	┞	┟
        // U+252x	┠	┡	┢	┣	┤	┥	┦	┧	┨	┩	┪	┫	┬	┭	┮	┯
        // U+253x	┰	┱	┲	┳	┴	┵	┶	┷	┸	┹	┺	┻	┼	┽	┾	┿
        // U+254x	╀	╁	╂	╃	╄	╅	╆	╇	╈	╉	╊	╋
        // U+257x	╰	╱	╲	╳	╴	╵	╶	╷	╸	╹	╺	╻	╼	╽	╾	╿

        const BOX_CHARS: [[[[char; 3]; 3]; 3]; 3] = [
            [
                // top empty
                [[' ', '╴', '╸'], ['╷', '┐', '┑'], ['╻', '┒', '┓']], // right empty
                [['╶', '─', '╾'], ['┌', '┬', '┭'], ['┎', '┰', '┱']], // right small
                [['╺', '╼', '━'], ['┍', '┮', '┯'], ['┏', '┲', '┳']], // right heavy
            ],
            [
                // top small
                [['╵', '┘', '┙'], ['│', '┤', '┥'], ['╽', '┧', '┪']], // right empty
                [['└', '┴', '┵'], ['├', '┼', '┽'], ['┟', '╁', '╅']], // right small
                [['┕', '┶', '┷'], ['┝', '┾', '┿'], ['┢', '╆', '╈']], // right heavy
            ],
            [
                // top heavy
                [['╹', '┚', '┛'], ['╿', '┦', '┩'], ['┃', '┨', '┫']], // right empty
                [['┖', '┸', '┹'], ['┞', '╀', '╃'], ['┠', '╂', '╉']], // right small
                [['┗', '┺', '┻'], ['┡', '╄', '╇'], ['┣', '╊', '╋']], // right heavy
            ],
        ];

        let mut drawing: String = String::new();
        for i in 0..NUM_LINES + 1 {
            for j in 0..NUM_COLUMNS + 1 {
                drawing.push(
                    BOX_CHARS[lines[i][j][0]][lines[i][j][1]][lines[i][j][2]][lines[i][j][3]],
                );
            }
            drawing.push('\n');
        }

        write!(f, "{}", drawing)
    }
}
