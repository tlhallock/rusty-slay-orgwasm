use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::Choices;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::MoveCardTask;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskProgressResult;

#[derive(Debug, Clone)]
pub struct Sacrifice {
	num: u32,
}

impl Sacrifice {
	pub fn create(num: u32) -> Box<dyn PlayerTask> {
		Box::new(Self::new(num))
	}
	pub fn new(num: u32) -> Self {
		Self { num }
	}
}

// fn card_is_sacrificable(stack: &state::Stack) -> bool {
//   true
// }

impl PlayerTask for Sacrifice {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		if self.num == 0 {
			return Ok(TaskProgressResult::TaskComplete);
		}

		let party = &game.players[player_index].party;
		let mut options: Vec<TasksChoice> = party
			.tops()
			// .filter(card_is_sacrificable)
			.map(|card| {
				TasksChoice::new(
					context.id_generator.generate(),
					ChoiceDisplay {
						display_type: card.as_choice(),
						label: format!("Sacrifice {}.", card.label()),
					},
					vec![Box::new(MoveCardTask {
						source: DeckPath::Party(player_index),
						destination: DeckPath::Discard,
						card_id: card.id,
					})],
				)
			})
			.collect();

		if options.len() == self.num as usize {
			for option in options.iter_mut() {
				option.select(game, player_index)?;
			}
			return Ok(TaskProgressResult::TaskComplete);
		}

		if options.is_empty() {
			return Ok(TaskProgressResult::TaskComplete);
		}

		game.players[player_index].choices = Some(Choices {
			instructions: "Choose a card to sacrifice.".to_string(),
			options,
			default_choice: None,
			timeline: deadlines::get_sacrifice_deadline(),
		});

		self.num -= 1;
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Player is sacrificing {} heros.", self.num)
	}
}
