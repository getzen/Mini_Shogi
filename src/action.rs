// Action
// Any player action like moving, drawing, swapping, etc that a player can take in the game.

use crate::{game::Coord};

#[derive(Debug, Clone, Copy)]
pub enum ActionKind {
    MoveNoCapture,
    MoveWithCapture,
    // MoveWithPromotion,
    FromReserve,
    ToReserve, // needed as undo for FromReserve
}

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub kind: ActionKind,
    pub piece_id: usize,
    pub from: Option<Coord>,
    pub to: Coord,
    pub captured_id: Option<usize>,
    pub reserve_index: Option<usize>,
}

impl Action {
    pub fn new(action_kind: ActionKind, 
        piece_id: usize, from: Option<Coord>, to: Coord, 
        captured_id: Option<usize>, reserve_index: Option<usize>) -> Self {
        Self {
            kind: action_kind,
            piece_id, from, to, captured_id, reserve_index,
        }
    }

    pub fn undo(&self) -> Action {
        match self.kind {
            ActionKind::MoveNoCapture => {
                Action {
                    from: Some(self.to), to: self.from.unwrap(), ..*self
                }
            }
            ActionKind::MoveWithCapture => {
                Action {
                    from: Some(self.to), to: self.from.unwrap(), ..*self
                }
            }
            ActionKind::FromReserve => {
                Action {
                    kind: ActionKind::ToReserve, ..*self
                }
            }
            ActionKind::ToReserve => {
                Action {
                    kind: ActionKind::FromReserve, ..*self
                }
            }
        }
    }
}
