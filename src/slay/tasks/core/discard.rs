use std::collections::HashSet;

use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::Choices;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specification::HeroType;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::move_card::MoveCardTask;

#[derive(Debug, Clone)]
pub enum DiscardVictimSpec {
	Myself,
	FromParam(TaskParamName),
	PlayersWith(HeroType),
}

#[derive(Debug, Clone)]
pub struct Discard {
	num: u32,
	include: Option<HashSet<ids::CardId>>,
	victim: DiscardVictimSpec,
}

impl Discard {
	pub fn create(num: u32) -> Box<dyn PlayerTask> {
		Box::new(Self::new(num))
	}
	pub fn new(num: u32) -> Self {
		Self {
			num,
			include: None,
			victim: DiscardVictimSpec::Myself,
		}
	}
	pub fn discard_one_of(include: HashSet<ids::CardId>) -> Self {
		Self {
			num: 1,
			include: Some(include),
			victim: DiscardVictimSpec::Myself,
		}
	}
	pub fn from_param(num: u32, param: TaskParamName) -> Box<dyn PlayerTask> {
		Box::new(Self {
			num,
			include: None,
			victim: DiscardVictimSpec::FromParam(param),
		})
	}
	pub fn each_player_with(hero_type: HeroType) -> Box<dyn PlayerTask> {
		Box::new(Self {
			num: 1,
			include: None,
			victim: DiscardVictimSpec::PlayersWith(hero_type),
		})
	}

	pub fn should_include(&self, card_id: ids::CardId) -> bool {
		if let Some(include) = &self.include {
			include.contains(&card_id)
		} else {
			true
		}
	}

	pub fn get_victims(
		&self,
		game: &Game,
		myself: ids::PlayerIndex,
	) -> SlayResult<Vec<ids::PlayerIndex>> {
		match self.victim {
			DiscardVictimSpec::Myself => Ok(vec![myself]),
			DiscardVictimSpec::FromParam(param) => Ok(vec![game.player_param(myself, &param)?]),
			DiscardVictimSpec::PlayersWith(hero_type) => Ok(
				game
					.players
					.iter()
					.filter(|player| player.has_hero_type(&hero_type))
					.map(|player| player.player_index)
					.collect(),
			),
		}
	}
}

impl PlayerTask for Discard {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		if self.num == 0 {
			return Ok(TaskProgressResult::TaskComplete);
		}
		for victim in self.get_victims(game, player_index)? {
			// check if the number to discard is the same as the number of cards in your hand.
			self.num -= 1;
			let options: Vec<TasksChoice> = game.players[victim]
				.hand
				.tops()
				.filter(|card| self.should_include(card.id))
				.map(|card| {
					TasksChoice::prepend(
						context.id_generator.generate(),
						ChoiceDisplay {
							display_type: card.as_choice(),
							label: format!("Discard {}", card.get_spec().label),
						},
						vec![Box::new(MoveCardTask {
							source: DeckPath::Hand(player_index),
							destination: DeckPath::Discard,
							card_id: card.id,
						}) as Box<dyn PlayerTask>],
					)
				})
				.collect();

			if options.is_empty() {
				continue;
			}
			game.players[victim].choices = Some(Choices::new(
				options,
				None,
				deadlines::get_discard_deadline(),
				"Choose a card to discard.".to_owned(),
			));
		}
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!("Player must discard {} cards", self.num)
	}
}
