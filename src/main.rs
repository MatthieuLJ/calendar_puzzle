use piece::OrientedPiece;

pub mod board;
pub mod piece;

fn main() {
    let mut board = board::Board::new();

    let t: OrientedPiece = OrientedPiece {
        pattern: [
            ['t', '0', '0', '0', '0'],
            ['t', 't', 't', '0', '0'],
            ['t', '0', '0', '0', '0'],
            ['0', '0', '0', '0', '0'],
            ['0', '0', '0', '0', '0'],
        ],
        top_index: 0,
    };
    let v: OrientedPiece = OrientedPiece {
        pattern: [
            ['v', 'v', 'v', '0', '0'],
            ['0', '0', 'v', '0', '0'],
            ['0', '0', 'v', '0', '0'],
            ['0', '0', '0', '0', '0'],
            ['0', '0', '0', '0', '0'],
        ],
        top_index: 0,
    };

    board = board.place_piece_on_top_left(&t).expect("This should work");
    board = board.place_piece_on_top_left(&v).expect("This should work");


    println!("Got a new board:");
    println!("{}", board);
}
