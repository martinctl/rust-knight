use std::fmt::Display;

/// A chess piece
/// use std::fmt::Display;

/// Represents a chess piece.
///
/// This struct is used to represent a chess piece with its type and color.
pub struct Piece {
    piece: u8,
}

/// Represents the type of the chess piece.
///
/// This enum is used to represent the type of the chess piece.
/// It can be a Pawn, Knight, Bishop, Rook, Queen, King, or None.
#[repr(u8)]
pub enum PieceType {
    None = 0,
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

/// Represents the color of a chess piece.
///
/// This enum is used to represent the color of a chess piece.
/// It can be either White or Black.
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 8,
}

/// Represents the specific chess piece.
///
/// This enum is used to represent a specific chess piece with its type and color.
#[repr(u8)]
pub enum Pieces{
    None = 0,
    WhitePawn = 1,  // 0b0001
    WhiteKnight = 2,  // 0b0010
    WhiteBishop = 3,  // 0b0011
    WhiteRook = 4,  // 0b0100
    WhiteQueen = 5,  // 0b0101
    WhiteKing = 6,  // 0b0110
    BlackPawn = 9,  // 0b1001
    BlackKnight = 10,  // 0b1010
    BlackBishop = 11,  // 0b1011
    BlackRook = 12,  // 0b1100
    BlackQueen = 13,  // 0b1101
    BlackKing = 14,  // 0b1110
}

impl Piece {
    /// Creates a new chess piece.
    ///
    /// # Arguments
    ///
    /// * `piece` - A specific chess piece from the Pieces enum.
    ///
    /// # Returns
    ///
    /// * A new Piece object.
    pub fn new(piece: Pieces) -> Piece {
        Piece {
            piece: piece as u8,
        }
    }

    /// Gets the type of the chess piece.
    ///
    /// # Returns
    ///
    /// * The type of the chess piece from the PieceType enum.
    pub fn piece_type(&self) -> PieceType {
        match self.piece & !0b1000 {
            0 => PieceType::None,
            1 => PieceType::Pawn,
            2 => PieceType::Knight,
            3 => PieceType::Bishop,
            4 => PieceType::Rook,
            5 => PieceType::Queen,
            6 => PieceType::King,
            _ => panic!("Invalid piece type"),
        }
    }

    /// Gets the color of the chess piece.
    ///
    /// # Returns
    ///
    /// * The color of the chess piece from the Color enum.
    pub fn color(&self) -> Color {
        match self.piece >> 3 {
            0 => Color::White,
            1 => Color::Black,
            _ => panic!("Invalid color"),
        }
    }

    /// Gets the specific chess piece.
    ///
    /// # Returns
    ///
    /// * The specific chess piece from the Pieces enum.
    pub fn piece(&self) -> Pieces {
        match self.piece {
            0 => Pieces::None,
            1 => Pieces::WhitePawn,
            2 => Pieces::WhiteKnight,
            3 => Pieces::WhiteBishop,
            4 => Pieces::WhiteRook,
            5 => Pieces::WhiteQueen,
            6 => Pieces::WhiteKing,
            9 => Pieces::BlackPawn,
            10 => Pieces::BlackKnight,
            11 => Pieces::BlackBishop,
            12 => Pieces::BlackRook,
            13 => Pieces::BlackQueen,
            14 => Pieces::BlackKing,
            _ => panic!("Invalid piece"),
        }
    }

    /// Gets the symbol of the chess piece.
    ///
    /// # Returns
    ///
    /// * The symbol of the chess piece.
    pub fn get_symbol(&self) -> &'static str {
        match self.piece() {
            Pieces::None => " ",
            Pieces::WhitePawn => "P",
            Pieces::WhiteKnight => "N",
            Pieces::WhiteBishop => "B",
            Pieces::WhiteRook => "R",
            Pieces::WhiteQueen => "Q",
            Pieces::WhiteKing => "K",
            Pieces::BlackPawn => "p",
            Pieces::BlackKnight => "n",
            Pieces::BlackBishop => "b",
            Pieces::BlackRook => "r",
            Pieces::BlackQueen => "q",
            Pieces::BlackKing => "k",
        }
    }
}

impl Display for Piece {
    /// Formats the Piece for display.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a Formatter.
    ///
    /// # Returns
    ///
    /// * A Result that indicates whether the operation was successful.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_symbol())
    }
}

#[cfg(test)]
impl Piece {
    /// Creates a new chess piece for testing.
    ///
    /// # Arguments
    ///
    /// * `value` - create a new chess piece with the given value.
    ///
    /// # Returns
    ///
    /// * A new Piece object.
    pub fn new_for_test(value: u8) -> Piece {
        Piece {
            piece: value
        }
    }
}
