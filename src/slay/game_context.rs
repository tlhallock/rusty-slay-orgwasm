use crate::slay::ids;
use crate::slay::message;

use rand::rngs::ThreadRng;
use rand::thread_rng;

#[derive(Clone)]
pub struct GameBookKeeping {
    pub id_generator: ids::IdGenerator,
    pub rng: ThreadRng,
}

impl Default for GameBookKeeping {
    fn default() -> Self {
        Self::new()
    }
}

impl GameBookKeeping {
    pub fn new() -> Self {
        GameBookKeeping {
            rng: thread_rng(),
            id_generator: ids::IdGenerator::new(),
        }
    }

    pub fn emit(&mut self, notification: &message::Notification) {
        println!("Notification: {}", notification.message_text);
    }
}
