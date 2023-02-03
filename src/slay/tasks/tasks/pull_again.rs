use crate::slay::choices::CardPath;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::notification::Notification;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::core::pull::PullFromTask;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::immediate::PlayImmediatelyFilter;

#[derive(Clone, Debug)]
pub struct PullAgain {
	victim_param: TaskParamName,
	card_param: TaskParamName,
	filter: PlayImmediatelyFilter,
}

impl PullAgain {
	pub fn create(
		victim_param: TaskParamName,
		card_param: TaskParamName,
		filter: PlayImmediatelyFilter,
	) -> Box<dyn PlayerTask> {
		Box::new(Self {
			victim_param,
			card_param,
			filter,
		}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for PullAgain {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let _player = game.player_param(player_index, &self.victim_param)?;
		let card_option = game.card_param(player_index, &self.card_param)?;
		if card_option.is_none() {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let applies_to_pull = match card_option {
			None => false,
			Some(card_id) => self.filter.can_play_immediately(
				game.card(CardPath::TopCardIn(DeckPath::Hand(player_index), card_id)),
			),
		};

		context.emit(&Notification::CanPullAgain(player_index, applies_to_pull));
		if !applies_to_pull {
			return Ok(TaskProgressResult::TaskComplete);
		}
		game.players[player_index]
			.tasks
			.prepend(PullFromTask::create(self.victim_param));
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"pull again".to_owned()
	}
}
