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

    // week_day is 1 for Monday, 7 for Sunday
    pub fn set_date(&mut self, week_day: u8, day: u8, month: u8) {
        let week_coords: (usize, usize) = match week_day {
            1 => (7, 0),
            2 => (8, 0),
            3 => (7, 1),
            4 => (7, 2),
            5 => (7, 3),
            6 => (8, 3),
            7 => (8, 4),
            _ => {
                panic!("week_day should be from 1 (Monday) to 7 (Sunday)")
            }
        };
        self.table[week_coords.0][week_coords.1] = 'X';

        let day_coords: (usize, usize) = match day {
            1 => (4, 0),
            2 => (5, 0),
            3 => (6, 0),
            4 => (1, 1),
            5 => (2, 1),
            6 => (3, 1),
            7 => (4, 1),
            8 => (5, 1),
            9 => (6, 1),
            10 => (1, 2),
            11 => (2, 2),
            12 => (3, 2),
            13 => (4, 2),
            14 => (7, 5),
            15 => (6, 2),
            16 => (1, 3),
            17 => (2, 3),
            18 => (3, 3),
            19 => (4, 3),
            20 => (5, 3),
            21 => (6, 3),
            22 => (1, 4),
            23 => (2, 4),
            24 => (3, 4),
            25 => (4, 4),
            26 => (5, 4),
            27 => (6, 4),
            28 => (4, 5),
            29 => (5, 5),
            30 => (6, 5),
            31 => (5, 2),
            _ => {
                panic!("day should be between 1 and 31")
            }
        };
        self.table[day_coords.0][day_coords.1] = 'X';

        let month_coords: (usize, usize) = match month {
            1 => (0, 0),
            2 => (1, 0),
            3 => (2, 0),
            4 => (3, 0),
            5 => (0, 1),
            6 => (0, 2),
            7 => (0, 3),
            8 => (0, 4),
            9 => (0, 5),
            10 => (1, 5),
            11 => (2, 5),
            12 => (3, 5),
            _ => {
                panic!("month should be between 1 and 12")
            }
        };
        self.table[month_coords.0][month_coords.1] = 'X';
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

        '_outer: for i in 0..NUM_LINES {
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
                    (0, 0) => {
                        // top left
                        lines[i][j] = get_lines(('X', 'X', self.table[i][j], 'X'));
                    }
                    (0, NUM_COLUMNS) => {
                        // top right
                        lines[i][j] = get_lines(('X', 'X', 'X', self.table[i][j - 1]));
                    }
                    (NUM_LINES, 0) => {
                        // bottom left
                        lines[i][j] = get_lines(('X', self.table[i - 1][j], 'X', 'X'));
                    }
                    (NUM_LINES, NUM_COLUMNS) => {
                        //bottom right
                        lines[i][j] = get_lines((self.table[i - 1][j - 1], 'X', 'X', 'X'));
                    }
                    (NUM_LINES, _) => {
                        // bottom
                        lines[i][j] =
                            get_lines((self.table[i - 1][j - 1], self.table[i - 1][j], 'X', 'X'));
                    }
                    (0, _) => {
                        // top
                        lines[i][j] = get_lines(('X', 'X', self.table[i][j], self.table[i][j - 1]));
                    }
                    (_, 0) => {
                        // left
                        lines[i][j] = get_lines(('X', self.table[i - 1][j], self.table[i][j], 'X'));
                    }
                    (_, NUM_COLUMNS) => {
                        // right
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

#[cfg(test)]
mod test {

}