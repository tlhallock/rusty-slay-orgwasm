use crate::slay::{
	errors::SlayResult,
	game_context::GameBookKeeping,
	ids,
	state::game::Game,
	tasks::player_tasks::{PlayerTask, TaskProgressResult},
};

#[derive(Debug, Clone)]
pub struct DrawTask {
	pub number_to_draw: usize,
}

impl DrawTask {
	pub fn create(num: usize) -> Box<dyn PlayerTask> {
		Box::new(Self::new(num))
	}
	pub fn new(number_to_draw: usize) -> Self {
		Self { number_to_draw }
	}
}

impl PlayerTask for DrawTask {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.replentish_for(self.number_to_draw);
		game.players[player_index]
			.hand
			.extend(game.draw.drain(0..self.number_to_draw));

		// TODO: Check everything about drawing...
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Draw {} cards.", self.number_to_draw)
	}
}
