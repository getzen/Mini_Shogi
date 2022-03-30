// Piece

use crate::GameLocation;
use crate::GameLocation::*;
use crate::piece::PieceKind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceKind {
    King,
    Gold,
    Silver,
    SilverPro,
    Pawn,
    PawnPro,
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
       let mut vectors =  match self.kind {
            King =>      vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1), (-1,-1), (1,-1)],
            Gold =>      vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1)],
            Silver =>    vec![(0,1), (1,1), (-1,1), (-1,-1), (1,-1)],
            SilverPro => vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1)], // same as Gold
            Pawn =>      vec![(0,1)],
            PawnPro =>   vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1)], // same as Gold
        };
        // Flip the y axis for player 1. Could optimize by hard-coding these values.
        if self.player == 1 {
            vectors.iter().for_each(|v| v.1 = -v.1);
        }
        vectors
    }

    pub fn promotion_kind(&self) -> Option<PieceKind> {
        match self.kind {
            King => None,
            Gold => None,
            Silver => Some(SilverPro),
            SilverPro => None,
            Pawn => Some(PawnPro),
            PawnPro => None,
        }
    }

    pub fn demotion_kind(&self) -> Option<PieceKind> {
        match self.kind {
            King => None,
            Gold => None,
            Silver => None,
            SilverPro => Some(Silver),
            Pawn => None,
            PawnPro => Some(Pawn),
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