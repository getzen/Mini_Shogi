// Piece

use crate::GameLocation;
use crate::GameLocation::*;
use crate::piece::PieceKind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceKind {
    King,
    Rook,
    Bishop,
    Pawn,
    Samurai,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Piece {
    pub id: usize,
    pub kind: PieceKind,
    pub player: usize,
    pub location: GameLocation, 
    pub location_index: usize, // grid or reserve location
}

impl Piece {
    pub fn new(id: usize, kind: PieceKind, player: usize) -> Self {
        Self {
            id, kind, player,
            location: OutOfGame,
            location_index: usize::MAX,
        }
    }

    pub fn move_vectors(&self) -> Vec<(i8, i8)> {
        match self.kind {
            King => vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1), (-1,-1), (1,-1)],

            Rook => vec![(1,0), (0,1), (-1,0), (0,-1)],

            Bishop => vec![(1,1), (-1,1), (-1,-1), (1,-1)],

            Pawn => {
                if self.player == 0 {
                    vec![(0,1)]
                } else {
                    vec![(0,-1)]
                }
            },

            Samurai => vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1),],
        }
    }

    pub fn promotion_kind(&self) -> Option<PieceKind> {
        match self.kind {
            King => None,
            Rook => None,
            Bishop => None,
            Pawn => Some(Samurai),
            Samurai => None,
        }
    }

    pub fn demotion_kind(&self) -> Option<PieceKind> {
        match self.kind {
            King => None,
            Rook => None,
            Bishop => None,
            Pawn => None,
            Samurai => Some(Pawn),
        }
    }

    #[allow(dead_code)]
    pub fn string_rep(&self) -> &str {
        match self.kind {
            King => "K",
            Rook => "R",
            Bishop => "B",
            Pawn => "P",
            Samurai => "S",
        }
    }
}