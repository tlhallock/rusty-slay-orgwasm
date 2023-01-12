use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specification::HeroType;
use crate::slay::state::game::Game;
use crate::slay::tasks::core::pull;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct Mimimeow {}

impl Mimimeow {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for Mimimeow {
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
			if !game.players[player_index].has_hero_type(&HeroType::Thief) {
				continue;
			}
			pull::pull_a_random_card(context, game, player_index, victim_index);
		}
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"do slippery paws".to_owned()
	}
}
