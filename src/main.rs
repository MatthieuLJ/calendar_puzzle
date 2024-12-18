use chrono::Datelike;
use dateparser::parse;
use std::io;
use std::time::Instant;
use termion;

mod board;
mod piece;

fn main() {
    let mut board = board::Board::new();
    let mut pieces = piece::all_pieces();
    let mut solutions: Vec<board::Board> = Vec::new();

    let (week_day, day, month) = get_date();

    board.set_date(week_day, day, month);

    let now = Instant::now();

    let _ = try_placing_pieces(&mut board, &mut pieces, &mut solutions);

    let elapsed = now.elapsed();
    
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    println!("Got {} solutions", solutions.len());
    for s in solutions {
        println!("{s}");
    }
    println!("Solved in {} ms", elapsed.as_millis());
}

fn get_date() -> (u8, u8, u8) {
    let mut input: String = String::new();
    loop {
        println!("What day would you like to solve for?");
        io::stdin().read_line(&mut input).unwrap_or_else(|error| {
            panic!("Could not read from input {error}");
        });
        let result = match parse(&input.trim()) {
            Ok(d) => {
                println!("That was a {}", d.weekday());
                (
                    d.weekday().number_from_monday() as u8,
                    d.day() as u8,
                    d.month() as u8,
                )
            }
            Err(error) => {
                println!(
                    "Could not parse the data {}, got [{}]",
                    error.to_string(),
                    input
                );
                continue;
            }
        };
        return result;
    }
}

const CHECK_FOR_SOLVABILITY_THRESH: usize = 5;

fn try_placing_pieces<'a>(
    b: &'a mut board::Board,
    pieces: &mut Vec<char>,
    solutions: &mut Vec<board::Board>,
) {
    print!("{}{}{b}", termion::clear::All, termion::cursor::Goto(1, 1));
    if b.is_full() {
        return;
    } else {
        for piece_index in 0..pieces.len() {
            let this_piece = pieces[piece_index];
            let orientations = piece::get_piece(pieces[piece_index])
                .expect("We should be able to get orientations")
                .possible_pieces;
            for oriented_piece in orientations {
                if b.place_piece_on_top_left(&oriented_piece) {

                    if b.is_full() {
                        solutions.push(b.clone());
                        b.remove_piece(this_piece);
                        continue;
                    }

                    if pieces.len() <= CHECK_FOR_SOLVABILITY_THRESH && !b.is_solvable() {
                        b.remove_piece(this_piece);
                        continue;
                    }

                    pieces.remove(piece_index);
                    try_placing_pieces(b, pieces, solutions);

                    pieces.insert(piece_index, this_piece);
                    b.remove_piece(this_piece);
                }
            }
        }
        return;
    }
}
