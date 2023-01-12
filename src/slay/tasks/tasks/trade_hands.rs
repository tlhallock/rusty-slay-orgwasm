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
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let victim_index = game.player_param(player_index, &self.param)?;
		let victim_drained = game.players[victim_index].hand.drain(..);
		let my_drained = game.players[player_index].hand.drain(..);
		game.players[player_index].hand.extend(victim_drained);
		game.players[victim_index].hand.extend(my_drained);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Trading hands with somebody.")
	}
}
