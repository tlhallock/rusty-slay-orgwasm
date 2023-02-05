use super::users::{PlayerInformation, UserId};
use chrono::DateTime;
use chrono::Utc;

pub struct GameOptions {}

pub enum SlotSelection {
	Any,
	OnlyHuman,
	LocalBot, /* (version) */
	RemoteBot,
}

pub enum SlotStatus {
	Empty,
	NotReady,
	Ready,
	Disconnected,
	Initializing,
	InGame,
}

pub struct Slot {
	pub selection: SlotSelection,
	user: Option<PlayerInformation>,
	last_heartbeat: DateTime<Utc>,
	ready: bool,
}

impl Slot {
	pub fn status(&self) -> SlotStatus {
		if self.user.is_none() {
			return SlotStatus::Empty;
		}
		// Check heart beat
		if self.ready {
			SlotStatus::NotReady
		} else {
			SlotStatus::Ready
		}
	}
	pub fn receive_heartbeat(&mut self) {
		self.last_heartbeat = chrono::offset::Utc::now();
	}
}

pub struct Lobby {
	pub creator: UserId,
	pub created: DateTime<Utc>, // TODO:
	pub options: GameOptions,
	pub slots: Vec<Slot>,
}

impl Lobby {
	pub fn status(&self) {}
}
