use crate::piece::{Piece, Pieces};

pub mod piece;

fn main() {
    let piece = Piece::new(Pieces::BlackBishop);
    println!("Piece: {}", piece);
}