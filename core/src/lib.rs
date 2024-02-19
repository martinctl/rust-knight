#![allow(dead_code)]
mod piece;
mod board;

#[cfg(test)]
mod tests {
    use super::piece::*;

    const WHITE_PAWN: u8 = 1;

    #[test]
    fn test_piece_type() {
        let p = Piece::new(Pieces::WhitePawn);
        assert_eq!(p.piece_type() as u8, PieceType::Pawn as u8);
        let p2 = Piece::new(Pieces::WhiteKnight);
        assert_eq!(p2.piece_type() as u8, PieceType::Knight as u8);
        let p3 = Piece::new(Pieces::WhiteBishop);
        assert_eq!(p3.piece_type() as u8, PieceType::Bishop as u8);
        let p4 = Piece::new(Pieces::WhiteRook);
        assert_eq!(p4.piece_type() as u8, PieceType::Rook as u8);
        let p5 = Piece::new(Pieces::WhiteQueen);
        assert_eq!(p5.piece_type() as u8, PieceType::Queen as u8);
        let p6 = Piece::new(Pieces::WhiteKing);
        assert_eq!(p6.piece_type() as u8, PieceType::King as u8);
        let n = Piece::new(Pieces::None);
        assert_eq!(n.piece_type() as u8, PieceType::None as u8);
    }

    #[test]
    fn test_color() {
        let p = Piece::new(Pieces::WhitePawn);
        assert_eq!(p.color() as u8, Color::White as u8);
        let p2 = Piece::new(Pieces::BlackKnight);
        assert_eq!(p2.color() as u8, Color::Black as u8);
    }

    #[test]
    fn test_piece() {
        let p = Piece::new(Pieces::WhitePawn);
        assert_eq!(p.piece() as u8, Pieces::WhitePawn as u8);
    }

    #[test]
    fn test_display() {
        assert_eq!(Piece::new(Pieces::WhitePawn).to_string(), "P");
        assert_eq!(Piece::new(Pieces::WhiteKnight).to_string(), "N");
        assert_eq!(Piece::new(Pieces::WhiteBishop).to_string(), "B");
        assert_eq!(Piece::new(Pieces::WhiteRook).to_string(), "R");
        assert_eq!(Piece::new(Pieces::WhiteQueen).to_string(), "Q");
        assert_eq!(Piece::new(Pieces::WhiteKing).to_string(), "K");
        assert_eq!(Piece::new(Pieces::BlackPawn).to_string(), "p");
        assert_eq!(Piece::new(Pieces::BlackKnight).to_string(), "n");
        assert_eq!(Piece::new(Pieces::BlackBishop).to_string(), "b");
        assert_eq!(Piece::new(Pieces::BlackRook).to_string(), "r");
        assert_eq!(Piece::new(Pieces::BlackQueen).to_string(), "q");
        assert_eq!(Piece::new(Pieces::BlackKing).to_string(), "k");
        assert_eq!(Piece::new(Pieces::None).to_string(), " ");
    }

    #[test]
    #[should_panic (expected = "Invalid piece type")]
    fn test_panic_piece_type() {
        Piece::new_for_test(17).piece_type();
    }

    #[test]
    #[should_panic (expected = "Invalid color")]
    fn test_panic_color() {
        Piece::new_for_test(17).color();
    }

    #[test]
    #[should_panic (expected = "Invalid piece")]
    fn test_panic_piece() {
        Piece::new_for_test(17).piece();
    }
}
