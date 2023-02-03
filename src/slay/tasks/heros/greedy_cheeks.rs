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
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::tasks::move_card::MoveCardTask;

#[derive(Clone, Debug)]
pub struct GreedyCheeks {}

impl GreedyCheeks {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for GreedyCheeks {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		for victim_index in 0..game.number_of_players() {
			if player_index == victim_index {
				continue;
			}

			let options = game.players[victim_index]
				.hand
				.tops()
				.map(|card| {
					TasksChoice::new(
						context.id_generator.generate(),
						Choice::ChooseCardToGive(card.card_type, player_index),
						ChoiceDisplayType::HighlightPath(DisplayPath::CardAt(CardPath::TopCardIn(
							DeckPath::Hand(victim_index),
							card.id,
						))),
						vec![Box::new(MoveCardTask {
							source: DeckPath::Hand(victim_index),
							destination: DeckPath::Hand(player_index),
							card_id: card.id,
						})],
					)
				})
				.collect::<Vec<_>>();

			if options.is_empty() {
				continue;
			}
			game.players[victim_index].choose(Choices {
				choices_type: ChoicesType::ChooseCardToGive(player_index),
				default_choice: None,
				timeline: deadlines::get_refactor_me_deadline(),
				options,
			});
		}
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"do greedy cheeks".to_owned()
	}
}
