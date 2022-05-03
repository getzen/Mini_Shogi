// Piece

use crate::game::GameLocation;
use crate::piece::PieceKind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceKind {
    King,
    Gold,
    Silver,
    SilverPro,
    Rook,
    RookPro,
    Bishop,
    BishopPro,
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
    // fn texture_for(piece_kind: PieceKind) -> Texture2D {
    //     match piece_kind {
    //         King => AssetLoader::get_texture("king"),
    //         Gold => "gold.png",
    //         Silver => "silver.png",
    //         SilverPro => "silver_pro.png",
    //         Pawn => "pawn.png",
    //         PawnPro => "pawn_pro.png",
    //     }
    // }

    pub fn new(id: usize, kind: PieceKind, player: usize) -> Self {
        Self {
            id, kind, player,
            location: GameLocation::OutOfGame,
            location_index: usize::MAX,
        }
    }

    /// Single-space piece moves.
    pub fn short_move_vectors(&self) -> Vec<(i8, i8)> {
       let mut vectors =  match self.kind {
            King =>      vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1), (-1,-1), (1,-1)],
            Gold =>      vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1)],
            Silver =>    vec![(0,1), (1,1), (-1,1), (-1,-1), (1,-1)],
            SilverPro => vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1)], // same as Gold
            RookPro =>   vec![(1,1), (-1,1), (-1,-1), (1,-1)],
            BishopPro => vec![(1,0), (0,1), (-1,0), (0,-1)],
            Pawn =>      vec![(0,1)],
            PawnPro =>   vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1)], // same as Gold
            _ => vec![],
        };
        // Flip the y axis for player 1. Could optimize by hard-coding these values.
        if self.player == 1 {
            vectors.iter_mut().for_each(|v| v.1 = -v.1);
        }
        vectors
    }

    /// Multiple-space piece moves. These are iterated over and stopped when another piece
    /// or the edge of the board is found.
    pub fn long_move_vectors(&self) -> Vec<(i8, i8)> {
        match self.kind {
            Rook | RookPro =>      vec![(1,0), (0,1), (-1,0), (0,-1)],
            Bishop | BishopPro =>  vec![(1,1), (-1,1), (-1,-1), (1,-1)],
            _ => Vec::new(),
        }
    }

    pub fn promotion_kind(&self) -> Option<PieceKind> {
        match self.kind {
            Silver => Some(SilverPro),
            Rook => Some(RookPro),
            Bishop => Some(BishopPro),
            Pawn => Some(PawnPro),
            King | Gold | SilverPro | RookPro | BishopPro | PawnPro => None,
        }
    }

    pub fn demotion_kind(&self) -> Option<PieceKind> {
        match self.kind {
            SilverPro => Some(Silver),
            RookPro => Some(Rook),
            BishopPro => Some(Bishop),
            PawnPro => Some(Pawn),
            King | Gold | Silver | Rook | Bishop | Pawn => None,
        }
    }

    #[allow(dead_code)]
    pub fn string_rep(&self) -> &str {
        match self.kind {
            King => "K",
            Gold => "G",
            Silver => "S",
            SilverPro => "S+",
            Rook => "R",
            RookPro => "R+",
            Bishop => "B",
            BishopPro => "B+",
            Pawn => "P",
            PawnPro => "P+",
        }
    }
}