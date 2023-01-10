use crate::slay::actions::DrawTask;
use crate::slay::choices::{ChoiceDisplay, ChoiceDisplayType, Choices, TasksChoice};
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::modifiers::{ModifierOrigin, PlayerModifier};
use crate::slay::specs::magic::MagicSpell;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::{Card, Stack};
use crate::slay::tasks::{MoveCardTask, PlayerTask, TaskParamName, TaskProgressResult};
use crate::slay::{deadlines, ids};

use super::destroy::DestroyTask;
use super::discard::Discard;
use super::params::{
	ChooseCardFromPlayerParameterTask, ChoosePlayerParameterTask, ClearParamsTask,
	SetParameterToMyself,
};
use super::steal::{StealCardFromTask, StealTask, UnStealCardFromTask};



pub enum SearchDiscardFilters {
	IsHero,
}

impl SearchDiscardFilters {
	fn filter(&self, card: &Card) -> bool {
		match self {
    	SearchDiscardFilters::IsHero => card.is_hero(),
		}
	}
}


pub fn create_search_discard_choices(
	context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	filter: SearchDiscardFilters,
) -> Option<Choices> {
	let options = game
		.deck(DeckPath::Discard)
		.tops()
		.filter(|card| filter.filter(card))
		.map(|card| {
			TasksChoice::new(
				context.id_generator.generate(),
				ChoiceDisplay {
					display_type: ChoiceDisplayType::Card(card.as_perspective()),
					label: card.spec.label.to_owned(),
				},
				vec![Box::new(MoveCardTask {
					source: DeckPath::Discard,
					destination: DeckPath::Hand(player_index),
					card_id: card.id,
				})],
			)
		})
		.collect::<Vec<_>>();
	if options.is_empty() {
		return None;
	}
	Some(Choices {
		instructions: "Choose a hero card to add to your hand.".to_owned(),
		default_choice: None,
		options,
		timeline: deadlines::get_refactor_me_deadline(), // This one should probably be longer...
	})
}

#[derive(Clone, Debug)]
pub struct MagicTask {
	spell: MagicSpell,
}

impl MagicTask {
	pub fn new(spell: MagicSpell) -> Self {
		Self { spell }
	}
}

impl PlayerTask for MagicTask {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		match self.spell {
			MagicSpell::EnganglingTrap => {
				let tasks = &mut vec![Discard::create(2), StealTask::create()];
				game.players[player_index].tasks.prepend_from(tasks);
				Ok(TaskProgressResult::TaskComplete)
			}
			MagicSpell::CriticalBoost => {
				let tasks = &mut vec![DrawTask::create(3), Discard::create(1)];
				game.players[player_index].tasks.prepend_from(tasks);
				Ok(TaskProgressResult::TaskComplete)
			}
			MagicSpell::DestructiveSpell => {
				let tasks = &mut vec![Discard::create(1), DestroyTask::create()];
				game.players[player_index].tasks.prepend_from(tasks);
				Ok(TaskProgressResult::TaskComplete)
			}
			MagicSpell::WindsOfChange => {
				// description: "Return an Item card equipped to any player's Hero card to that player's hand,
				let tasks = &mut vec![ReturnModifierTask::create(), DrawTask::create(1)];
				game.players[player_index].tasks.prepend_from(tasks);
				Ok(TaskProgressResult::TaskComplete)
			}
			MagicSpell::EnchangedSpell => {
				let duration = game.get_turn().for_this_turn();
				game.players[player_index].buffs.add_buff(
					duration,
					PlayerModifier::AddToAllRolls(2),
					ModifierOrigin::FromMagicCard(self.spell),
				);
				Ok(TaskProgressResult::TaskComplete)
			}
			MagicSpell::ForcedExchange => {
				let tasks = &mut vec![
					ChoosePlayerParameterTask::exclude_self(
						TaskParamName::ForcedExchangeVictim,
						"Choose a player to forcefully exchange heros with.",
					),
					ChooseCardFromPlayerParameterTask::from_party(
						TaskParamName::ForcedExchangeVictim,
						TaskParamName::ForcedExchangeVictimCard,
						"Which hero card would you like to steal?",
					),
					StealCardFromTask::create(
						TaskParamName::ForcedExchangeVictim,
						TaskParamName::ForcedExchangeVictimCard,
					),
					// Note: Should we check if we win here?!?!?!
					SetParameterToMyself::create(TaskParamName::ForcedExchangeSelf),
					ChooseCardFromPlayerParameterTask::from_party(
						TaskParamName::ForcedExchangeSelf,
						TaskParamName::ForcedExchangeVictimDonationCard,
						"Which hero card would you like to move to their hand?",
					),
					UnStealCardFromTask::create(
						TaskParamName::ForcedExchangeSelf,
						TaskParamName::ForcedExchangeVictimDonationCard,
					),
					ClearParamsTask::create(),
				];
				game.players[player_index].tasks.prepend_from(tasks);
				Ok(TaskProgressResult::TaskComplete)
			}
			MagicSpell::ForcefulWinds => {
				let mut cards_to_move = Vec::new();
				for player_index in 0..game.number_of_players() {
					for stack in game.players[player_index].party.stacks_mut() {
						cards_to_move.append(&mut stack.modifiers);
					}
					game.players[player_index]
						.hand
						.extend(cards_to_move.drain(..).map(|c| Stack::new(c)));
				}
				Ok(TaskProgressResult::TaskComplete)
			}
			MagicSpell::CallToTheFallen => {
				game.players[player_index].choices = create_search_discard_choices(
					context,
					game,
					player_index,
					SearchDiscardFilters::IsHero
				);
				Ok(TaskProgressResult::TaskComplete)
			}
		}
	}

	fn label(&self) -> String {
		format!("Cast the {:?} spell", self.spell)
	}
}

#[derive(Clone, Debug)]
pub struct ReturnModifierTask {}

impl ReturnModifierTask {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {})
	}
}

impl PlayerTask for ReturnModifierTask {
	fn make_progress(
		&mut self, context: &mut GameBookKeeping, game: &mut Game, chooser_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let mut options = Vec::new();
		for player_index in 0..game.number_of_players() {
			for stack in game.players[player_index].party.stacks() {
				for modifier in stack.modifiers.iter() {
					options.push(TasksChoice::new(
						context.id_generator.generate(),
						ChoiceDisplay {
							display_type: ChoiceDisplayType::Card(modifier.as_perspective()),
							label: format!(
								"{} from {}",
								modifier.spec.label, game.players[player_index].name
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
