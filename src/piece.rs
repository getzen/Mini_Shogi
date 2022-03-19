// Piece

use crate::game::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceKind {
    King,
    Rook,
    Bishop,
    Pawn,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Piece {
    pub id: usize,
    pub kind: PieceKind,
    pub player: usize,
    pub coord: Option<Coord>,
}

impl Piece {
    pub fn new(id: usize, kind: PieceKind, player: usize) -> Self {
        Self {
            id, kind, player,
            coord: None,
        }
    }

    pub fn move_vectors(&self) -> Vec<(i8, i8)> {
        match self.kind {
            PieceKind::King => vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1), (-1,-1), (1,-1)],

            PieceKind::Rook => vec![(1,0), (0,1), (-1,0), (0,-1)],

            PieceKind::Bishop => vec![(1,1), (-1,1), (-1,-1), (1,-1)],

            PieceKind::Pawn => {
                if self.player == 0 {
                    vec![(0,1)]
                } else {
                    vec![(0,-1)]
                }
            },
        }
    }

    #[allow(dead_code)]
    pub fn string_rep(&self) -> &str {
        match self.kind {
            PieceKind::King => "K",
            PieceKind::Rook => "R",
            PieceKind::Bishop => "B",
            PieceKind::Pawn => "P",
        }
    }
}