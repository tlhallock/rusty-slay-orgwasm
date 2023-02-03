
use crate::slay::choices::Choice;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct Bullseye {}

impl Bullseye {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for Bullseye {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.replentish_for(3);
		let mut card_ids = Vec::default();
		{
			let iter = &mut game.deck(DeckPath::Draw).tops();
			for _ in 0..3 {
				if let Some(card) = iter.next() {
					card_ids.push(card.to_owned());
				}
			}
		}
		if card_ids.is_empty() {
			// context.emit(Notification::NoCardsToDraw)
			return Ok(TaskProgressResult::TaskComplete);
		}
		game.players[player_index].choose(Choices {
			choices_type: ChoicesType::BullseyeKeep,
			options: card_ids
				.into_iter()
				.map(|card| {
					TasksChoice::new(
						context.id_generator.generate(),
						Choice::BullseyeKeep(card.card_type),
						ChoiceDisplayType::Card_(card.card_type),
						vec![BullseyeReorderChoice::create(card.id)],
					)
				})
				.collect(),
			default_choice: None,
			timeline: deadlines::get_refactor_me_deadline(),
		});

		// TODO: Not implemented...
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"do bullseye".to_owned()
	}
}

#[derive(Clone, Debug)]
pub struct BullseyeReorderChoice {
	to_keep: ids::CardId,
}

impl BullseyeReorderChoice {
	pub fn create(to_keep: ids::CardId) -> Box<dyn PlayerTask> {
		Box::new(Self { to_keep }) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for BullseyeReorderChoice {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.move_card(DeckPath::Draw, DeckPath::Hand(player_index), self.to_keep);

		if game.deck(DeckPath::Draw).num_top_cards() == 0 {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let next_tops = &mut Vec::default();
		{
			let iter = &mut game.deck(DeckPath::Draw).tops();
			if let Some(card) = iter.next() {
				next_tops.push(card.to_owned());
			}
			if let Some(card) = iter.next() {
				next_tops.push(card.to_owned());
			}
		}
		let iter = &mut next_tops.iter();
		if let Some(first_card) = iter.next() {
			if let Some(second_card) = iter.next() {
				game.players[player_index].choose(Choices {
					choices_type: ChoicesType::BullseyeOrdering(first_card.card_type, second_card.card_type),
					options: vec![
						TasksChoice::new(
							context.id_generator.generate(),
							Choice::BullseyeReorder,
							ChoiceDisplayType::Yes,
							vec![BullseyeReorder::create()],
						),
						TasksChoice::new(
							context.id_generator.generate(),
							Choice::BullseyeDoNotReorder,
							ChoiceDisplayType::No,
							vec![],
						),
					],
					default_choice: None,
					timeline: deadlines::get_refactor_me_deadline(),
				})
			}
		}
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"choose which order to put it back".to_owned()
	}
}

#[derive(Clone, Debug)]
pub struct BullseyeReorder {}

impl BullseyeReorder {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for BullseyeReorder {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		_player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.deck_mut(DeckPath::Draw).swap_first_cards();
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Swap the next cards to be drawn".to_owned()
	}
}
