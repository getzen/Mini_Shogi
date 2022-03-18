// // // MonteCarloTree
// // // More advanced than MonteCarloPure.


// use std::collections::HashMap;
// use std::collections::hash_map::DefaultHasher;
// use std::fmt::Display;
// use std::hash::{Hash, Hasher};
// // use std::hash::Hash;
// use std::time::{Duration, Instant};

// use crate::game::*;
// use crate::Action;
// // use crate::action::ActionKind::*;
// use crate::ai::AIProgress;
// use crate::controller::Message;
// use crate::message_sender::MessageSender;
// // use crate::Piece;

// #[derive(Clone)]
// pub struct NodeData {
//     parent: u64, // hash value
//     player: u8, // current player
//     plays: f64,
//     results: f64,
//     actions_available: Vec<Action>,
//     //actions_available: [Action; 9], // convert for speed?
//     //actions_count: usize,
// }

// impl NodeData {
//     fn new(player: u8, parent: u64) -> Self {
//         Self {
//             player,
//             parent,
//             plays: 0.0,
//             results: 0.0,
//             actions_available: Vec::new(),
//         }
//     }
// }

// impl Display for NodeData {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "(pl:{} par:{} plays:{}, res:{}, act_len:{})", 
//         self.player, 
//         self.parent, 
//         self.plays, 
//         self.results, 
//         self.actions_available.len())
//     }
// }

// pub struct AIMonteCarloTree {
//     root: Game,
//     search_player: u8,
//     children: HashMap<u64, Game>,
//     node_data: HashMap<u64, NodeData>,
//     time_limit: Duration,
//     message_sender: MessageSender,
// }

// impl AIMonteCarloTree {
//     pub fn new(game: Game, time_limit: Duration, message_sender: MessageSender) -> Self {
//         Self {
//             root: game,
//             time_limit,
//             message_sender,
//             search_player: game.current_player as u8,
//             children: HashMap::new(),
//             node_data: HashMap::new(),
//         }
//     }

//     pub fn think(&mut self) -> AIProgress {
//         let now = Instant::now();
//         let mut progress= AIProgress::new();

//         // Add the root node to get started.
//         self.add_to_tree(self.root, 0);

//         while now.elapsed() < self.time_limit {
//             // Selection. Best node according to formula.
//             let node_key = self.selection();
//             // debug
//             let node_data = self.node_data.get(&node_key).unwrap();
//             let action_count = node_data.actions_available.len();
//             println!("actions available len: {}", action_count);

//             // Expansion. Get random child of selected node.
//             let child = self.expansion(node_key);
//             let child_key = self.add_to_tree(child, node_key);
//             // debug
//             println!("after child added, keys: {}", self.node_data.keys().len());

//             // Play it out.
//             let result = self.play_out(child.clone());

//             // Backprogation.
//             self.backpropagate(child_key, result);

//             for (k, v) in &self.node_data {
//                 println!("key: {}, node_data.plays: {}", k, v.plays);
//             }

//             // Update progress.
//             progress.nodes += 1;
//             progress.duration = now.elapsed();
//             // progress.pv = ....
//             self.message_sender.send(Message::AIUpdate(progress.clone()));
//         }

//         progress.duration = now.elapsed();
//         progress
//     }

//     // fn select(&self, node_key: u64) -> Vec<u64> {
//     //     let mut path = vec![];
//     //     loop {
//     //         path.push(node_key);
//     //         if !self.children.contains_key(&node_key) {
//     //             // node is unexplored or terminal
//     //             return path;
//     //         }
//     //         let node = self.children.get(&node_key).unwrap();
//     //         let unexplored = node.actions_available();
//     //         if !unexplored.is_empty() {

//     //         }
//     //     }
//     // }

//     fn hash_key_for(&self, game: &Game) -> u64 {
//         let mut hasher = DefaultHasher::new();
//         game.hash(&mut hasher);
//         hasher.finish()
//     }

//     fn add_to_tree(&mut self, mut game: Game, parent: u64) -> u64 {
//         let key = self.hash_key_for(&game);

//         let mut node_data = NodeData::new(game.current_player as u8, parent);
//         node_data.actions_available = game.actions_available();

//         self.children.insert(key, game);
//         self.node_data.insert(key, node_data);
//         key
//     }

//     /// Get game (node) key with the highest Upper Confidence Bound (UCB).
//     fn selection(&self) -> u64 {
//         let mut best_ucb = 0.0;
//         let mut best_key: u64 = 0;

//         // If there is only one child (usually just the root), return its key.
//         if self.children.len() == 1 {
//             if let Some(key) = self.children.keys().next() {
//                 return *key;
//             }
//         }

//         for (key, _game) in &self.children {
//             let node_data = self.node_data.get(&key).unwrap();
//             if node_data.parent == 0 { continue }; // root node
            
//             // Get parent's data and number of plays.
//             let parent_data = self.node_data.get(&node_data.parent);
//             let mut parent_plays = 0.0;
//             if let Some(p_data) = parent_data {
//                 parent_plays = p_data.plays;
//             }
//             let mut ucb_score = node_data.results / node_data.plays; // w / n
//             ucb_score += 1.414 * ((parent_plays).ln() / node_data.plays).sqrt();
//             println!("pp: {}, n_results: {}, n_plays: {}, ucb: {}", parent_plays, node_data.plays, node_data.results, &ucb_score);

//             if ucb_score > best_ucb {
//                 best_ucb = ucb_score;
//                 best_key = *key;
//                 println!("new best. ucb: {}, key: {}", &ucb_score, &key);
//             }
//         }
//         best_key
//     }

//     /// In this process, a new child node is added to the tree to that
//     /// node which was selected.
//     fn expansion(&mut self, parent_key: u64) -> Game {
//         let mut child = self.children.get(&parent_key).unwrap().clone();

//         self.node_data.entry(parent_key).and_modify(|data| {
//             // Pick a random action and perform it.
//             let rand_index = fastrand::usize(0..data.actions_available.len());
//             let rand_action = data.actions_available.swap_remove(rand_index);
//             child.perform_action(&rand_action, true);
//         });
//         // what about child's node_data and available actions?
//         self.add_to_tree(child, parent_key);
//         child
//     }

//     /// Simulation step. Play randomly until end, returning result.
//     fn play_out(&mut self, mut clone: Game) -> f64 { // result must be 0, 0.5, or 1.0
//         let mut result = 0.0;
//         while clone.update_state() == &GameState::Ongoing {
//             let mut available = clone.actions_available();
//             let rand_index = fastrand::usize(0..available.len());
//             let rand_action = available.swap_remove(rand_index);
//             clone.perform_action(&rand_action, true);
//             //progress.nodes += 1;
//         }
        
//         match clone.state {
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

//     /// After determining the value of the newly added
//     /// node, the remaining tree must be updated. So, the backpropagation
//     /// process is performed, where it backpropagates from the new node 
//     /// to the root node. During the process, the number of simulation 
//     /// stored in each node is incremented. Also, if the new node’s 
//     /// simulation results in a win, then the number of wins is also 
//     /// incremented.
//     fn backpropagate(&mut self, mut key: u64, result: f64) {
//         let mut reached_root = false;
//         while !reached_root {
//             self.node_data.entry(key).and_modify(|data| {
//                 data.plays += 1.0;
//                 data.results += result;

//                 println!("data key {}: {}", key, data);

//                 if data.parent == 0 {
//                     reached_root = true;
//                 } else {
//                     key = data.parent;
//                 }
//             });
//         }
//     }

// }