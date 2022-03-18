// Piece

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceKind {
    X,
    O,
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

    #[allow(dead_code)]
    pub fn string_rep(&self) -> &str {
        match self.kind {
            PieceKind::X => "X",
            PieceKind::O => "O",
        }
    }
}