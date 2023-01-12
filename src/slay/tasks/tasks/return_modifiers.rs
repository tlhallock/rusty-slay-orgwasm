use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
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
enum ReturnModifiersTarget {
	Myself,
	Everyone,
}

#[derive(Clone, Debug)]
pub struct ReturnModifierTask {
	target: ReturnModifiersTarget,
}

impl ReturnModifierTask {
	pub fn return_everyones() -> Box<dyn PlayerTask> {
		Box::new(Self {
			target: ReturnModifiersTarget::Everyone,
		})
	}
	pub fn return_mine() -> Box<dyn PlayerTask> {
		Box::new(Self {
			target: ReturnModifiersTarget::Myself,
		})
	}
	fn player_indices(
		&self,
		player_index: ids::PlayerIndex,
		number_of_player: usize,
	) -> Vec<ids::PlayerIndex> {
		match self.target {
			ReturnModifiersTarget::Myself => vec![player_index],
			ReturnModifiersTarget::Everyone => (0..number_of_player).collect(),
		}
	}
}

impl PlayerTask for ReturnModifierTask {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		chooser_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let player_indices = self.player_indices(chooser_index, game.number_of_players());
		let mut options = Vec::new();
		for player_index in player_indices {
			for stack in game.players[player_index].party.stacks() {
				for modifier in stack.modifiers.iter() {
					options.push(TasksChoice::new(
						context.id_generator.generate(),
						ChoiceDisplay {
							display_type: ChoiceDisplayType::Card_(modifier.card_type),
							label: format!(
								"{} from {}",
								modifier.get_spec().label,
								game.players[player_index].name
							),
						},
						vec![Box::new(MoveCardTask {
							source: DeckPath::Party(player_index),
							destination: DeckPath::Hand(player_index),
							card_id: modifier.id,
						})],
					));
				}
			}
		}
		if options.is_empty() {
			return Ok(TaskProgressResult::TaskComplete);
		}
		game.players[chooser_index].choices = Some(Choices {
			instructions: "Choose which modifier card to return".to_owned(),
			default_choice: None,
			options,
			timeline: deadlines::get_refactor_me_deadline(),
		});
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Returning all modifiers".to_owned()
	}
}
