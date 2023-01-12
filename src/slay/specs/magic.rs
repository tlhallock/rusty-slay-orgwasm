use enum_iterator::Sequence;

use crate::slay::{
	abilities::magic::{create_search_discard_choices, SearchDiscardFilters},
	errors::SlayResult,
	game_context::GameBookKeeping,
	ids,
	modifiers::{ModifierOrigin, PlayerModifier},
	state::{game::Game, stack::Stack},
	tasks::{
		core::{
			destroy::DestroyTask,
			discard::Discard,
			draw::DrawTask,
			steal::{StealCardFromTask, StealTask, UnStealCardFromTask},
		},
		player_tasks::TaskProgressResult,
		task_params::TaskParamName,
		tasks::{
			params::{
				ChooseCardFromPlayerParameterTask, ChoosePlayerParameterTask, ClearParamsTask,
				SetParameterToMyself,
			},
			return_modifiers::ReturnModifierTask,
		},
	},
};

#[derive(Debug, Clone, Copy, Sequence, PartialEq)]
pub enum MagicSpell {
	EnganglingTrap,
	CriticalBoost,
	DestructiveSpell,
	WindsOfChange,
	EnchangedSpell,
	ForcedExchange,
	ForcefulWinds,
	CallToTheFallen,
}
impl MagicSpell {
	pub fn perform_spell(
		&self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		match self {
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
				game.players[player_index].temporary_buffs.add_buff(
					duration,
					PlayerModifier::AddToAllRolls(2),
					ModifierOrigin::FromMagicCard(*self),
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
						.extend(cards_to_move.drain(..).map(Stack::new));
				}
				Ok(TaskProgressResult::TaskComplete)
			}
			MagicSpell::CallToTheFallen => {
				game.players[player_index].choices =
					create_search_discard_choices(context, game, player_index, SearchDiscardFilters::IsHero);
				Ok(TaskProgressResult::TaskComplete)
			}
		}
	}
}
