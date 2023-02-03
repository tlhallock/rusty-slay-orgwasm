use std::collections::HashSet;

use crate::slay::choices::Choice;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesType;
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

// #[derive(Debug, Clone)]
// pub enum DiscardVictimSpec {
// 	Myself,
// 	FromParam(TaskParamName),
// 	PlayersWith(HeroType),
// }

#[derive(Debug, Clone)]
pub struct Discard {
	num: u32,
	include: Option<HashSet<ids::CardId>>,
}

impl Discard {
	pub fn create(num: u32) -> Box<dyn PlayerTask> {
		Box::new(Self::new(num))
	}
	pub fn new(num: u32) -> Self {
		Self { num, include: None }
	}
	pub fn discard_one_of(include: HashSet<ids::CardId>) -> Self {
		Self {
			num: 1,
			include: Some(include),
		}
	}

	fn should_include(&self, card_id: ids::CardId) -> bool {
		if let Some(include) = &self.include {
			include.contains(&card_id)
		} else {
			true
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
		// check if the number to discard is the same as the number of cards in your hand.
		self.num -= 1;

		let options: Vec<TasksChoice> = game.players[player_index]
			.hand
			.tops()
			.filter(|card| self.should_include(card.id))
			.map(|card| {
				TasksChoice::prepend(
					context.id_generator.generate(),
					Choice::Discard(card.card_type),
					card.as_choice(),
					vec![Box::new(MoveCardTask {
						source: DeckPath::Hand(player_index),
						destination: DeckPath::Discard,
						card_id: card.id,
					}) as Box<dyn PlayerTask>],
				)
			})
			.collect();

		if options.is_empty() {
			return Ok(TaskProgressResult::TaskComplete);
		}
		game.players[player_index].choose(Choices::new(
			options,
			None,
			deadlines::get_discard_deadline(),
			ChoicesType::Discard,
		));
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!("Player must discard {} cards", self.num)
	}
}

#[derive(Debug, Clone)]
pub struct DiscardFromParam {
	num: u32,
	victim_param: TaskParamName,
}
impl DiscardFromParam {
	pub fn create(num: u32, param: TaskParamName) -> Box<dyn PlayerTask> {
		Box::new(Self {
			num,
			victim_param: param,
		})
	}
}
impl PlayerTask for DiscardFromParam {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let victim_index = game.player_param(player_index, &self.victim_param)?;
		game.players[victim_index]
			.tasks
			.prepend(Discard::create(self.num));
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("{:?} will have to discard.", self.victim_param)
	}
}

#[derive(Debug, Clone)]
pub struct PlayersWithHeroTypeDiscard {
	num: u32,
	hero_type: HeroType,
}

impl PlayersWithHeroTypeDiscard {
	pub fn create(hero_type: HeroType) -> Box<dyn PlayerTask> {
		Box::new(Self { num: 1, hero_type })
	}
}

impl PlayerTask for PlayersWithHeroTypeDiscard {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		_player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		for player in game.players.iter_mut() {
			if !player.has_hero_type(&self.hero_type) {
				continue;
			}
			player.tasks.prepend(Discard::create(self.num));
		}
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!(
			"All players with {:?} will have to discard {} cards.",
			self.hero_type, self.num
		)
	}
}

// #[derive(Debug, Clone)]
// pub struct DiscardToSet {
//   parameter: TaskParamName,
// }

// impl DiscardToSet {
// 	pub fn create(parameter: TaskParamName,) -> Box<dyn PlayerTask> {
// 		Box::new(Self {parameter})
// 	}
// }

// impl PlayerTask for DiscardToSet {
// 	fn make_progress(
// 		&mut self,
// 		context: &mut GameBookKeeping,
// 		game: &mut Game,
// 		player_index: ids::PlayerIndex,
// 	) -> SlayResult<TaskProgressResult> {

// 		Ok(TaskProgressResult::TaskComplete)
// 	}

// 	fn label(&self) -> String {
// 		String::from("do discard to set.")
//   }
// }
