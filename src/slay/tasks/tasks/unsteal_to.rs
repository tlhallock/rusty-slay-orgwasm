// DonateHeroTo
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::specs::hero::HeroAbilityType;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::{PlayerTask, TaskProgressResult};
use crate::slay::tasks::task_params::TaskParamName;

#[derive(Clone, Debug)]
pub struct UnstealTo {
	victim_param: TaskParamName,
	// hero_card: HeroAbilityType,
}

impl UnstealTo {
	pub fn create(// victim_param: TaskParamName,
	) -> Box<dyn PlayerTask> {
		Box::new(Self {
			victim_param: TaskParamName::TipsyTootieVictim,
			// hero_card: HeroAbilityType::TipsyTootie,
		})
	}
}

impl PlayerTask for UnstealTo {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		// What if there is no one to steal from?
		let victim_index = game.player_param(player_index, &self.victim_param)?;
		let maybe_card = game
			.deck(DeckPath::Party(player_index))
			.tops()
			.find(|card| {
				matches!(
					card.card_type,
					SlayCardSpec::HeroCard(HeroAbilityType::TipsyTootie)
				)
			})
			.map(|card| card.id);
		if let Some(card_id) = maybe_card {
			game.move_card(
				DeckPath::Party(player_index),
				DeckPath::Party(victim_index),
				card_id,
			);
		} else {
			log::error!("Could not find tipsie tootie!!!!!!!!");
		}
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		"Give tipsie tootie away".to_owned()
	}
}
