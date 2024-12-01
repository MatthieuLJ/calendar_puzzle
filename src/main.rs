pub mod board;
pub mod piece;

fn main() {
    let mut board = board::Board::new();
    let mut pieces = piece::all_pieces();

    board.set_date(4, 13, 8);

    let _result = try_placing_pieces(&mut board, &mut pieces);
    
}

fn try_placing_pieces<'a>(b: &'a mut board::Board, pieces: &mut Vec<char>) -> bool {
    if b.is_full() {
        if pieces.is_empty() {
            println!("Found a solution:\n{}", b);
            return true;
        } else {
            return false;
        }
    } else {
        for piece_index in 0..pieces.len() {
            let this_piece = pieces[piece_index];
            let orientations = piece::get_piece(pieces[piece_index])
                .expect("We should be able to get orientations")
                .possible_pieces;
            for oriented_piece in orientations {
                if b.place_piece_on_top_left(&oriented_piece) {
                    //println!("Now:\n {:?}", b.table);
                    
                    pieces.remove(piece_index);
                    if try_placing_pieces(b, pieces) {
                        return true;
                    } else {
                        pieces.insert(piece_index, this_piece);
                        b.remove_piece(this_piece);
                        //println!("Then:\n {:?}", b.table);
                    }
                }
            }
        }
        return false;
    }
}
