use crate::slay::actions::place_hero;
use crate::slay::choices::CardPath;
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
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;

use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Debug, Clone)]
pub struct PlaceHero {}

impl PlaceHero {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for PlaceHero {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let mut options = Vec::new();
		for card in game.players[player_index].hand.tops() {
			if let SlayCardSpec::HeroCard(hero_card) = card.card_type {
				options.push(TasksChoice::new(
					context.id_generator.generate(),
					Choice::PlaceHeroImmediately(hero_card),
					ChoiceDisplayType::hand_card(player_index, card.id),
					place_hero::create_place_hero_challenges(
						context,
						game,
						player_index,
						CardPath::TopCardIn(DeckPath::Hand(player_index), card.id),
						hero_card,
					),
				));
			}
		}
		if options.is_empty() {
			return Ok(TaskProgressResult::TaskComplete);
		}
		game.players[player_index].choose(Choices {
			choices_type: ChoicesType::PlaceAHeroCard,
			options,
			default_choice: None,
			timeline: deadlines::get_refactor_me_deadline(),
		});
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		String::from("Place hero")
	}
}
