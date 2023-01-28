use crate::slay::choices::Choice;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::state::game::Game;
use crate::slay::tasks::core::destroy::DestroyTask;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::reveal::Reveal;

#[derive(Clone, Debug)]
pub struct PanChucksDestroy {}

impl PanChucksDestroy {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for PanChucksDestroy {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let drew_challenge = vec![
			game.card_param(player_index, &TaskParamName::PanChuckFirstCard)?,
			game.card_param(player_index, &TaskParamName::PanChuckSecondCard)?,
		]
		.iter()
		.flatten()
		.any(|card_id| {
			if let Some(card) = game.find_card(*card_id) {
				matches!(card.card_type, SlayCardSpec::Challenge)
			} else {
				false
			}
		});
		if drew_challenge {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let default_choice = context.id_generator.generate();
		game.players[player_index].choices = Some(Choices {
			choices_type: ChoicesType::RevealAndDestroy,
			timeline: deadlines::get_refactor_me_deadline(),
			default_choice: Some(default_choice),
			options: vec![
				TasksChoice::prepend(
					context.id_generator.generate(),
					Choice::RevealChallengeAndDestroy,
					ChoiceDisplayType::Yes,
					vec![
						Reveal::create(SlayCardSpec::Challenge),
						DestroyTask::create(),
					],
				),
				TasksChoice::prepend(
					default_choice,
					Choice::DoNotPlayImmediately,
					ChoiceDisplayType::No,
					vec![],
				),
			],
		});
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"do greedy cheeks".to_owned()
	}
}
