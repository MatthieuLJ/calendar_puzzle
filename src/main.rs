pub mod board;
pub mod piece;

fn main() {
    let board = board::Board::new();

    println!("Got a new board:");
    board.display();
}
