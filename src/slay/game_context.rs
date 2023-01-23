use crate::slay::ids;
use crate::slay::notification;

use rand::rngs::ThreadRng;
use rand::thread_rng;

#[derive(Clone)]
pub struct GameBookKeeping {
	pub id_generator: ids::IdGenerator,
	pub rng: ThreadRng,
	// pub notifier: Option<Box<dyn Fn(Notification) -> ()>>,
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

	pub fn emit(&mut self, notification: &notification::Notification) {
		log::info!("Notification: {:?}", notification);
		// self.notifier.iter().for_each(|f| f(notification.to_owned()));
	}
}
