//use lazy_static::lazy_static;
//use std::collections::HashMap;

pub struct OrientedPiece {
    pub pattern: [[char; 5]; 5],
    pub top_index: u8,
}

pub struct Piece {
  pub id : char,
  pub possile_pieces : Vec<OrientedPiece>
}

//lazy_static! {
//  let all_pieces : HashMap<char, Piece> = [];
//}

