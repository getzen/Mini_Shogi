// Action
// Any player action like moving, drawing, swapping, etc that a player can take in the game.

use crate::{game::Coord};

#[derive(Debug, Clone)]
pub enum ActionKind {
    MoveNoCapture,
    MoveWithCapture,
    // MoveWithPromotion,
    FromReserve,
    ToReserve,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub action_kind: ActionKind,
    pub piece_id: usize,
    pub from: Option<Coord>,
    pub to: Coord,
    // pub captured_piece_id,
    // pub from: Coord,
    // pub to: Coord,
    // pub captured_piece: Option<Piece>
}

impl Action {
    pub fn new(action_kind: ActionKind, piece_id: usize, from: Option<Coord>, to: Coord) -> Self {
        Self {
            action_kind, piece_id, from, to,
        }
    }

    pub fn undo(&self) -> Action {
        match self.action_kind {
            ActionKind::MoveNoCapture => {
                let to_coord = self.to;
                Action {
                    from: Some(self.to), to: self.from.unwrap(), ..*self
                }
            }
            ActionKind::MoveWithCapture => { ///////// fix this implementation
                let to_coord = self.to;
                Action {
                    from: Some(self.to), to: self.from.unwrap(), ..*self
                }
            }
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
