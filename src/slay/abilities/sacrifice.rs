use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::ChoiceInformation;
use crate::slay::choices::ChoiceLocator;
use crate::slay::choices::Choices;
use crate::slay::choices::DisplayArrow;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::MoveCardTask;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskParamName;
use crate::slay::tasks::TaskProgressResult;

#[derive(Debug, Clone)]
pub struct Sacrifice {
	num: u32,
}

impl Sacrifice {
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
			.iter()
			// .filter(card_is_sacrificable)
			.map(|s| {
				TasksChoice::new(
					ChoiceInformation {
						locator: ChoiceLocator {
							id: context.id_generator.generate(),
							player_index,
						},
						display: ChoiceDisplay {
							label: format!("Sacrifice {}.", s.top.label()),
							highlight: Some(DisplayPath::CardIn(DeckPath::Hand(player_index), s.top.id)),
							arrows: vec![DisplayArrow {
								source: DisplayPath::CardIn(DeckPath::Hand(player_index), s.top.id),
								destination: DisplayPath::DeckAt(DeckPath::Discard),
							}],
							roll_modification_choice: None,
						},
					},
					vec![Box::new(MoveCardTask {
						source: DeckPath::Party(player_index),
						destination: DeckPath::Discard,
						card_id: s.top.id,
					})],
				)
			})
			.collect();

		if options.len() == self.num as usize {
			for option in options.iter_mut() {
				option.select(context, game)?;
			}
			return Ok(TaskProgressResult::TaskComplete);
		}

		if options.is_empty() {
			return Ok(TaskProgressResult::TaskComplete);
		}

		let default_choice = options[0].get_choice_information().get_id();
		game.players[player_index].choices = Some(Choices {
			instructions: "Choose a card to sacrifice.".to_string(),
			options,
			default_choice: Some(default_choice),
			timeline: deadlines::get_sacrifice_deadline(),
		});

		self.num -= 1;
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Player is sacrificing {} heros.", self.num)
	}
}
