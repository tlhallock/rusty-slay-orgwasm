use crate::slay::choices::Choice;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;

#[derive(Debug, Clone)]
pub struct SacrificeTask {
	card_id: ids::CardId,
}

impl SacrificeTask {
	pub fn create(card_id: ids::CardId) -> Box<dyn PlayerTask> {
		Box::new(Self { card_id })
	}
}

impl PlayerTask for SacrificeTask {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let card_id = game.players[player_index]
			.party
			.stack(self.card_id)
			.ok_or_else(|| SlayError::new("Unable to find card to sacrifice"))?
			.get_id_to_sacrifice_or_destroy();
		// make a notification?

		game.move_card(DeckPath::Party(player_index), DeckPath::Discard, card_id)?;
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Player is sacrificing a hero.".to_string()
	}
}

#[derive(Debug, Clone)]
pub enum SacrificeVictim {
	Myself,
	FromParam(TaskParamName),
}

#[derive(Debug, Clone)]
pub struct ChooseSacrifice {
	num: u32,
	victim: SacrificeVictim,
}

impl ChooseSacrifice {
	pub fn create(num: u32) -> Box<dyn PlayerTask> {
		Box::new(Self::new(num))
	}
	pub fn new(num: u32) -> Self {
		Self {
			num,
			victim: SacrificeVictim::Myself,
		}
	}
	pub fn from_param(param: TaskParamName) -> Box<dyn PlayerTask> {
		Box::new(Self {
			num: 1,
			victim: SacrificeVictim::FromParam(param),
		})
	}
	fn get_victim(
		&self,
		game: &Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<ids::PlayerIndex> {
		match self.victim {
			SacrificeVictim::Myself => Ok(player_index),
			SacrificeVictim::FromParam(param) => game.player_param(player_index, &param),
		}
	}
}

// fn card_is_sacrificable(stack: &state::Stack) -> bool {
//   true
// }

impl PlayerTask for ChooseSacrifice {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		if self.num == 0 {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let victim_index = self.get_victim(game, player_index)?;
		let party = &game.players[victim_index].party;
		let mut options: Vec<TasksChoice> = party
			.tops()
			// .filter(card_is_sacrificable)
			.map(|card| {
				if let SlayCardSpec::HeroCard(hero_card) = card.card_type {
					TasksChoice::new(
						context.id_generator.generate(),
						Choice::Sacrifice(hero_card),
						card.as_choice(),
						vec![SacrificeTask::create(card.id)],
					)
				} else {
					unreachable!();
				}
			})
			.collect();

		if options.len() == self.num as usize {
			for option in options.iter_mut() {
				option.select(game, victim_index)?;
			}
			return Ok(TaskProgressResult::TaskComplete);
		}

		if options.is_empty() {
			return Ok(TaskProgressResult::TaskComplete);
		}

		game.players[victim_index].choose(Choices {
			choices_type: ChoicesType::Sacrifice,
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
