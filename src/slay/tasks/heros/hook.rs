use crate::slay::choices::CardPath;
use crate::slay::choices::Choice;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::notification::Notification;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::core::draw::DrawTask;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::tasks::add_tasks::AddTasks;
use crate::slay::tasks::tasks::immediate::create_play_card_immediately_task;
use crate::slay::tasks::tasks::immediate::PlayImmediatelyFilter;

#[derive(Clone, Debug)]
pub struct Hook {
	filter: PlayImmediatelyFilter,
	// extra_task: Option<Box<dyn PlayerTask>>,
}

impl Hook {
	pub fn create(filter: PlayImmediatelyFilter) -> Box<dyn PlayerTask> {
		Box::new(Hook {
			filter, /*extra_task: None,*/
		})
	}
}

impl PlayerTask for Hook {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		// let cards = game.players[player_index]
		// 	.hand
		// 	.tops()
		// 	.filter(|card| self.filter.can_play_immediately(card))
		// 	.map(|card| card.to_owned())
		// 	.collect::<Vec<_>>();

		// let mut options = cards
		// 	.into_iter()

		let mut options = game.players[player_index]
			.hand
			.tops()
			.filter(|card| self.filter.can_play_immediately(card))
			// .map(|card| card.to_owned())
			.filter_map(|card| {
				create_play_card_immediately_task(context, game, player_index, card).map(
					// |TasksChoice { id, choice, display, .. }| {
					|tasks| {
						TasksChoice::new(
							context.id_generator.generate(),
							Choice::PlayImmediately(card.card_type),
							ChoiceDisplayType::HighlightPath(DisplayPath::CardAt(CardPath::TopCardIn(
								DeckPath::Hand(player_index),
								card.id,
							))),
							vec![AddTasks::create(tasks), DrawTask::create(1)],
						)
					},
				)
			})
			.collect::<Vec<_>>();
		let able = !options.is_empty();
		context.emit(&Notification::CanPlayImmediately(player_index, able));

		if !able {
			return Ok(TaskProgressResult::TaskComplete);
		}

		let default_choice = context.id_generator.generate();
		options.push(TasksChoice::new(
			default_choice,
			Choice::DoNotPlayImmediately,
			ChoiceDisplayType::No,
			vec![],
		));

		game.players[player_index].choose(Choices {
			choices_type: ChoicesType::PlayOneOfImmediately,
			options,
			default_choice: Some(default_choice),
			timeline: deadlines::get_refactor_me_deadline(),
		});
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		"Play a card immediately".to_owned()
	}
}
