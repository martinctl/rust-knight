use std::fmt::{Debug, Display};

use crate::piece::{Piece, Pieces};

pub struct Board {
    board: [[Piece; 8]; 8],
}


impl Board {
    fn new_empty() -> Board {
        Board {
            board: [[Piece::new(Pieces::None); 8]; 8],
        }
    }

    pub fn new() -> Board {
        let mut board = Board::new_empty();
        board.set_starting_position();
        board
    }

    fn set_starting_position(&mut self) {
        self.board[0][0] = Piece::new(Pieces::WhiteRook);
        self.board[0][1] = Piece::new(Pieces::WhiteKnight);
        self.board[0][2] = Piece::new(Pieces::WhiteBishop);
        self.board[0][3] = Piece::new(Pieces::WhiteQueen);
        self.board[0][4] = Piece::new(Pieces::WhiteKing);
        self.board[0][5] = Piece::new(Pieces::WhiteBishop);
        self.board[0][6] = Piece::new(Pieces::WhiteKnight);
        self.board[0][7] = Piece::new(Pieces::WhiteRook);
        self.board[1][0] = Piece::new(Pieces::WhitePawn);
        self.board[1][1] = Piece::new(Pieces::WhitePawn);
        self.board[1][2] = Piece::new(Pieces::WhitePawn);
        self.board[1][3] = Piece::new(Pieces::WhitePawn);
        self.board[1][4] = Piece::new(Pieces::WhitePawn);
        self.board[1][5] = Piece::new(Pieces::WhitePawn);
        self.board[1][6] = Piece::new(Pieces::WhitePawn);
        self.board[1][7] = Piece::new(Pieces::WhitePawn);
        self.board[6][0] = Piece::new(Pieces::BlackPawn);
        self.board[6][1] = Piece::new(Pieces::BlackPawn);
        self.board[6][2] = Piece::new(Pieces::BlackPawn);
        self.board[6][3] = Piece::new(Pieces::BlackPawn);
        self.board[6][4] = Piece::new(Pieces::BlackPawn);
        self.board[6][5] = Piece::new(Pieces::BlackPawn);
        self.board[6][6] = Piece::new(Pieces::BlackPawn);
        self.board[6][7] = Piece::new(Pieces::BlackPawn);
        self.board[7][0] = Piece::new(Pieces::BlackRook);
        self.board[7][1] = Piece::new(Pieces::BlackKnight);
        self.board[7][2] = Piece::new(Pieces::BlackBishop);
        self.board[7][3] = Piece::new(Pieces::BlackQueen);
        self.board[7][4] = Piece::new(Pieces::BlackKing);
        self.board[7][5] = Piece::new(Pieces::BlackBishop);
        self.board[7][6] = Piece::new(Pieces::BlackKnight);
        self.board[7][7] = Piece::new(Pieces::BlackRook);
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..8 {
            for j in 0..8 {
                s.push_str(&self.board[i][j].to_string());
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}