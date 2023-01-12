use rand::Rng;

use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;

#[derive(Clone, Debug)]
pub struct PullFromTask {
	pub victim_param: TaskParamName,
	pub output_param: Option<TaskParamName>,
}

impl PullFromTask {
	pub fn create(victim_param: TaskParamName) -> Box<dyn PlayerTask> {
		Box::new(Self {
			victim_param,
			output_param: None,
		}) as Box<dyn PlayerTask>
	}
	pub fn record_pulled(
		victim_param: TaskParamName,
		output_param: Option<TaskParamName>,
	) -> Box<dyn PlayerTask> {
		Box::new(Self {
			victim_param,
			output_param,
		}) as Box<dyn PlayerTask>
	}
}

pub fn pull_a_random_card(
	context: &mut GameBookKeeping,
	game: &mut Game,
	puller_index: ids::PlayerIndex,
	victim_index: ids::PlayerIndex,
) -> Option<ids::CardId> {
	let destination = DeckPath::Hand(puller_index);
	let source = DeckPath::Hand(victim_index);
	let number_of_cards = game.deck(source).num_top_cards();
	if number_of_cards == 0 {
		return None;
	}
	let card_index = context.rng.gen_range(0..number_of_cards);
	let stack = game.deck_mut(source).take_at_index(card_index);
	let ret = stack.top.id;
	game.deck_mut(destination).add(stack);
	Some(ret)
}

impl PlayerTask for PullFromTask {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		puller_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let victim_index = game.players[puller_index]
			.tasks
			.get_player_value(&self.victim_param)
			.ok_or_else(|| SlayError::new("Expected a parameter value."))?;
		let pulled_card = pull_a_random_card(context, game, puller_index, victim_index);
		if let Some(param) = self.output_param {
			game.players[puller_index]
				.tasks
				.set_card_value(param, pulled_card)?;
		}
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Pulling from a player".to_string()
	}
}
