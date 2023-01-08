pub type ElementId = u32;
pub type CardId = ElementId;
pub type ChoiceId = ElementId;

pub type PlayerIndex = usize;

#[derive(Default, Debug, Clone)]
pub struct IdGenerator {
	pub next_id: u32,
}

impl IdGenerator {
	pub fn new() -> Self {
		IdGenerator { next_id: 0 }
	}
	pub fn from(other: &IdGenerator) -> Self {
		IdGenerator {
			next_id: other.next_id,
		}
	}
	pub fn generate(&mut self) -> ElementId {
		let ret = self.next_id;
		self.next_id += 1;
		ret
	}
}
