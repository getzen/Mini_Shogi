// Action
// Any player action like moving, drawing, swapping, etc that a player can take in the game.

use crate::{game::Coord, piece::PieceKind};

#[derive(Debug, Clone)]
pub enum ActionKind {
    FromReserve,
    ToReserve,
    // MoveNoCapture,
    // MoveWithCapture,
    // MoveWithPromotion,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub action_kind: ActionKind,
    pub piece_kind: PieceKind,
    pub coord: Coord,
    // pub captured_piece_id,
    // pub from: Coord,
    // pub to: Coord,
    // pub captured_piece: Option<Piece>
}

impl Action {
    pub fn new(action_kind: ActionKind, piece_kind: PieceKind, coord: Coord) -> Self {
        Self {
            action_kind, piece_kind, coord,
        }
    }

    pub fn undo(&self) -> Action {
        match self.action_kind {
            ActionKind::FromReserve => {
                Action {
                    action_kind: ActionKind::ToReserve, ..*self
                }
            }
            ActionKind::ToReserve => {
                Action {
                    action_kind: ActionKind::FromReserve, ..*self
                }
            }
        }
    }
}
