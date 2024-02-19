use std::fmt::Display;

/// A chess piece
/// use std::fmt::Display;

/// Represents a chess piece.
///
/// This struct is used to represent a chess piece with its type and color.
#[derive(Debug)]
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
        match self.piece & 0b111 {
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

    /// Gets the value of the chess piece.
    ///
    /// The value is based on the relative value of the chess pieces.
    /// See [Chess piece relative value](https://en.wikipedia.org/wiki/Chess_piece_relative_value)
    ///
    /// # Returns
    ///
    /// * The value of the chess piece.
    pub fn value(&self) -> i32 {
        match self.piece() {
            Pieces::None => 0,
            Pieces::WhitePawn | Pieces::BlackPawn => 100,
            Pieces::WhiteKnight | Pieces::BlackKnight => 320,
            Pieces::WhiteBishop | Pieces::BlackBishop => 330,
            Pieces::WhiteRook | Pieces::BlackRook => 500,
            Pieces::WhiteQueen | Pieces::BlackQueen => 900,
            Pieces::WhiteKing | Pieces::BlackKing => 20000,
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

    /// Gets the unicode symbol of the chess piece.
    ///
    /// # Returns
    ///
    /// * The unicode symbol of the chess piece.
    pub fn get_unicode_symbol(&self) -> &'static str {
        match self.piece() {
            Pieces::None => " ",
            Pieces::WhitePawn => "♙",
            Pieces::WhiteKnight => "♘",
            Pieces::WhiteBishop => "♗",
            Pieces::WhiteRook => "♖",
            Pieces::WhiteQueen => "♕",
            Pieces::WhiteKing => "♔",
            Pieces::BlackPawn => "♟",
            Pieces::BlackKnight => "♞",
            Pieces::BlackBishop => "♝",
            Pieces::BlackRook => "♜",
            Pieces::BlackQueen => "♛",
            Pieces::BlackKing => "♚",
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