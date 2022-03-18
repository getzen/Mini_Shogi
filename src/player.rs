// // Player

// use std::io::stdin;
// use std::io::stdout;
// use std::io::Write;

// use std::collections::HashSet;

// use crate::ai_minimax::AIMinimax;
// use crate::ai_monte_carlo::AIMonteCarlo;
// use crate::ai_random::AIRandom;

// use crate::{Coord, Game, Action, Piece};
// use crate::action::ActionKind::*;
// use crate::piece::PieceKind;



// #[derive(Debug, Clone)]
// pub struct Player {
//     pub id: usize,
//     pub kind: PlayerKind, // ai or human
//     pub piece_reserve: HashSet<Option<usize>>,
// }

// impl Player {
//     // pub fn new(id: usize, kind: PlayerKind) -> Self {
//     //     Self {
//     //         id,
//     //         kind,
//     //         piece_reserve: HashSet::new(),
//     //     }
//     // }

//     // pub fn add_piece_to_reserve(&mut self, id: usize) {
//     //     self.piece_reserve.insert(Some(id));
//     // }

//     // pub fn remove_piece_from_reserve(&mut self, id: usize) {
//     //     self.piece_reserve.remove(&Some(id));
//     // }

//     // // Returns the id of the first player piece to match the given kind in the given vec.
//     // pub fn reserve_piece_of(&self, kind: PieceKind, pieces: &[Piece]) -> Option<usize> {
//     //     for i in &self.piece_reserve {
//     //         if let Some(id) = i {
//     //             if pieces[*id].kind == kind {
//     //                 return *i;
//     //             }
//     //         }
//     //     }
//     //     None
//     // }

    

// }