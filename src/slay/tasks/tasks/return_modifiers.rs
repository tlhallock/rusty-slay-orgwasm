use crate::slay::choices::{ChoiceDisplay, ChoiceDisplayType, Choices, TasksChoice};
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::{PlayerTask, TaskProgressResult};
use crate::slay::tasks::tasks::move_card::MoveCardTask;
use crate::slay::{deadlines, ids};

#[derive(Clone, Debug)]
pub struct ReturnModifierTask {}

impl ReturnModifierTask {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {})
	}
}

impl PlayerTask for ReturnModifierTask {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		chooser_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let mut options = Vec::new();
		for player_index in 0..game.number_of_players() {
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
