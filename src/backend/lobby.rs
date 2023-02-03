

use super::users::{PlayerInformation, UserId};

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
	selection: SlotSelection,
	user: Option<PlayerInformation>,
	last_heartbeat: String,
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
}

pub struct Lobby {
	creator: UserId,
	created: String, // TODO:
	options: GameOptions,
	slots: Vec<Slot>,
}

impl Lobby {
	pub fn status(&self) {}
}
