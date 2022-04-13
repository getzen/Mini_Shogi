// MonteCarloTree
// More advanced than MonteCarloPure.
// This code is based on: https://github.com/wlongxiang/mcts/blob/main/monte_carlo_tree_search.py

// NON-FUNCTIONAL. See critical bug in ucb_select()
// I think this whole thing will need to be reworked somehow.

// use std::collections::HashMap;
// use std::time::{Duration, Instant};

// use crate::Action;
// use crate::ai::AIProgress;
// use crate::controller::Message;
// use crate::game::*;
// use crate::GameState::Ongoing;
// use crate::message_sender::MessageSender;

// pub struct AIMonteCarloTree {
//     root: Game,
//     search_player: u8,
//     time_limit: Duration,
//     message_sender: MessageSender,

//     reward: HashMap<Game, f64>, // Q
//     visits: HashMap<Game, f64>, // N
//     prev_action: HashMap<Game, Action>,
//     children: HashMap<Game, Vec<Game>>,
// }

// impl AIMonteCarloTree {
//     pub fn new(game: Game, time_limit: Duration, message_sender: MessageSender) -> Self {
//         Self {
//             root: game,
//             search_player: game.current_player as u8,
//             time_limit,
//             message_sender,
//             reward: HashMap::new(),
//             visits: HashMap::new(),
//             prev_action: HashMap::new(),
//             children: HashMap::new(),
//         }
//     }

//     pub fn think(&mut self) -> AIProgress {
//         let now = Instant::now();
//         let mut progress= AIProgress::new();

//         let root_clone = self.root.clone();
//         self.visits.insert(self.root, 0.0);

//         while now.elapsed() < self.time_limit {
//             self.one_iteration(&root_clone);

//             progress.nodes += 1;
//             progress.duration = now.elapsed();
//             // progress.pv = ....
//             self.message_sender.send(Message::AIUpdate(progress.clone()));
//         }

//         // Find best action
//         let mut best_score = f64::MIN;
//         let node_kids = self.children.get(&root_clone).unwrap();
//         for kid in node_kids {
//            let reward = self.reward.get(kid).unwrap();
//            let visits = self.visits.get(kid).unwrap();
//            let score = reward / visits;
//            if score > best_score {
//                best_score = score;
//                let action = self.prev_action.get(kid).unwrap();
//                progress.pv = vec![action.clone()];
//            }
//         }

//         progress.duration = now.elapsed();
//         progress
//     }

//     pub fn one_iteration(&mut self, node: &Game) {
//         let path = self.select(node);
//         let mut leaf = *path.last().unwrap();
//         self.expand(&mut leaf);
//         let mut reward = 0.0;
//         for _ in 0..1 {
//             reward += self.simulate(leaf.clone());
//         }
//         self.backup(path, reward);
//     }

//     fn select(&mut self, root: &Game) -> Vec<Game> {
//         let mut node = root;
//         let mut path = Vec::new();
//         loop {
//             path.push(*node);
//             if !self.children.contains_key(node) || node.state != Ongoing {
//                 //println!("children does not contain node");
//                 break;
//             }

//             let mut unexplored = Vec::new();
//             let node_kids = self.children.get(node).unwrap();
//             for kid in node_kids {
//                 if !self.children.contains_key(kid) {
//                     unexplored.push(*kid);
//                 }
//             }
//             if !unexplored.is_empty() {
//                 path.push(*unexplored.last().unwrap());
//                 //println!("found unexplored node");
//                 break;
//             }
//             //println!("geting ucb node");
//             node = self.ucb_select(node).unwrap();
//         }
//         path
//     }

//     fn ucb_select(&self, node: &Game) -> Option<&Game> {
//         let log_n_parent = self.visits.get(node).unwrap();
//         let mut best_ucb = f64::MIN;
//         let mut best_node: Option<&Game> = None;

//         let node_kids = self.children.get(node).unwrap();
//         if node_kids.is_empty() {
//             panic!("EMPTY!");
//         }

//         for node in node_kids {
//             let reward = self.reward.get(node).unwrap();
//             let visits = self.visits.get(node).unwrap();
//             let inner = log_n_parent / visits;
//             let ucb = reward / visits + 1.141 * inner.sqrt();

//             if ucb >= best_ucb {
//                 best_ucb = ucb;
//                 best_node = Some(node);
//             }
//         }
//         // debug code
//         // For some reason, best_node will remain None, even if node_kids is not empty.
//         // I am at a loss. Also, sometimes, node_kids *will* be empty, when it should
//         // never be.
//         if best_node.is_none() {
//             println!("NONE: best_ucb: {}, node count: {}", best_ucb, node_kids.len());
//         }

//         best_node
//     }

//     fn expand(&mut self, node: &mut Game) {
//         if self.children.contains_key(node) {
//             return;
//         }
//         let mut kids = Vec::new();
//         let actions = node.actions_available();
//         for action in actions {
//             let mut new_node = node.clone();
//             new_node.perform_action(&action, true);
//             self.prev_action.insert(new_node, action);
//             kids.push(new_node);

//             self.reward.insert(new_node, 0.0);
//             self.visits.insert(new_node, 0.0);
//         }
//         self.children.insert(*node, kids);
//     }

//     fn simulate(&mut self, mut node: Game) -> f64 {
//         let mut result = 0.0;
//         while node.update_state() == &GameState::Ongoing {
//             let mut available = node.actions_available();
//             let rand_index = fastrand::usize(0..available.len());
//             let rand_action = available.swap_remove(rand_index);
//             node.perform_action(&rand_action, true);
//             //progress.nodes += 1;
//         }
        
//         match node.state {
//             GameState::Draw => {
//                 result += 0.5;
//             },
//             GameState::WinPlayer0 => {
//                 if self.search_player == 0 {
//                     result += 1.0;
//                 } else {
//                     result += 0.0;
//                 }
//             },
//             GameState::WinPlayer1 => {
//                 if self.search_player == 1 {
//                     result += 1.0;
//                 } else {
//                     result += 0.0;
//                 }
//             },
//             GameState::Ongoing => {
//                 panic!("Game not completed!");
//             }
//         }
        
//         result
//     }

//     fn backup(&mut self, mut path: Vec<Game>, reward: f64) {
//         path.reverse();
//         for node in path {
//             self.visits.entry(node).and_modify(|visits| {
//                 *visits += 1.0;
//             });
//             self.reward.entry(node).and_modify(|rewards| {
//                 *rewards += reward;
//             });
//         }
//     }

    
// }