use crate::slay::{
	errors::SlayResult,
	game_context::GameBookKeeping,
	ids,
	state::game::Game,
	tasks::{
		player_tasks::{PlayerTask, TaskProgressResult},
		task_params::TaskParamName,
	},
};

#[derive(Clone, Debug)]
pub struct TradeHands {
	param: TaskParamName,
}

impl TradeHands {
	pub fn create(param: TaskParamName) -> Box<dyn PlayerTask> {
		Box::new(Self { param })
	}
}

impl PlayerTask for TradeHands {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let victim_index = game.player_param(player_index, &self.param)?;

		let mut victim_cards = Vec::new();
		let mut my_cards = Vec::new();

		victim_cards.extend(game.players[victim_index].hand.drain(..));
		my_cards.extend(game.players[player_index].hand.drain(..));
		game.players[player_index]
			.hand
			.extend(victim_cards.drain(..));
		game.players[victim_index].hand.extend(my_cards.drain(..));
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Trading hands with somebody.")
	}
}
