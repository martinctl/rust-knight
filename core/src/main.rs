use crate::board::Board;
use crate::piece::{Piece, Pieces};

pub mod piece;
pub mod board;

fn main() {
    let piece = Piece::new(Pieces::BlackBishop);
    println!("Piece: {}", piece);

    let board = Board::new();
    println!("{:?}", board);

}