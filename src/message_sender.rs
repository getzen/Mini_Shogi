/// MessageSender

use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use crate::ai::AIProgress;

pub enum AIMessage {
    AIUpdate(AIProgress),
    SearchCompleted(AIProgress),
}

/// Sends messages using the owned transmitter (tx). If min_time_between is
/// Messages will be ignored unless min_time has passed since the last message
/// was sent. (Useful to not overload the receiver (rx) with, for example,
/// progress update messages.) Set min_time to None to alway send immediately.
#[derive(Clone)]
pub struct MessageSender {
    pub tx: Sender<Message>,
    pub min_time_between: Option<Duration>,
    last_time: Option<Instant>,
}

impl MessageSender {
    pub fn new(tx: Sender<Message>, min_time_between: Option<Duration>) -> Self {
        Self {
            tx, min_time_between,
            last_time: None,
        }
    }
    
    pub fn send(&mut self, message: Message) {
        let mut send = false;
        if self.min_time_between.is_none() || self.last_time.is_none() {
            send = true;
        }
        else if let Some(time) = self.last_time {
            send = time.elapsed() > self.min_time_between.unwrap();
        }
        if send {
            let result = self.tx.send(message);
            if result.is_err() {
                println!("MessageSender send error.");
            }
            if self.min_time_between.is_some() {
                self.last_time = Some(Instant::now());
            } 
        }
    }
}

