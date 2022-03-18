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
    //move_vectors: Vec<Coord>,
}

impl Piece {
    pub fn new(id: usize, kind: PieceKind, player: usize) -> Self {
        Self {
            id, kind, player,
            //move_vectors: Vec::<Coord>::new()
        }
    }

    pub fn move_coords(&self) -> Vec<Coord> {
        match self.kind {
            PieceKind::King => vec![Coord(1,0), Coord(0,1), Coord(-1,0), Coord(0,-1),
                                    Coord(1,1), Coord(-1,1), Coord(-1,-1), Coord(1,-1)],

            PieceKind::Rook => vec![Coord(1,0), Coord(0,1), Coord(-1,0), Coord(0,-1)],

            PieceKind::Bishop => vec![Coord(1,1), Coord(-1,1), Coord(-1,-1), Coord(1,-1)],

            PieceKind::Pawn => {
                if self.player == 0 {
                    vec![Coord(0,1)]
                } else {
                    vec![Coord(0,-1)]
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