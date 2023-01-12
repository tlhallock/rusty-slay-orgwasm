use crate::slay::choices::CardPath;
use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;


use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::modifiers::ItemModifier;


use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;

use crate::slay::showdown::offer::OfferChallengesState;
use crate::slay::showdown::roll::ChallengeReason;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::showdown::roll_state::RollState;
use crate::slay::specification::MonsterSpec;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;

use crate::slay::tasks::core::draw::DrawTask;
use crate::slay::tasks::core::pull::PullFromTask;


use super::modifiers::PlayerModifier;
use super::specification::HeroType;
use super::specs::cards::SlayCardSpec;
use super::specs::hero::HeroAbility;
use super::specs::magic::MagicSpell;
use super::tasks::player_tasks::PlayerTask;
use super::tasks::task_params::TaskParamName;
use super::tasks::tasks::add_tasks::AddTasks;
use super::tasks::tasks::card_used::CardUsedTask;
use super::tasks::tasks::do_roll::DoRollTask;
use super::tasks::tasks::magic::MagicTask;
use super::tasks::tasks::move_card::MoveCardTask;
use super::tasks::tasks::offer_challenges::OfferChallengesTask;
use super::tasks::tasks::params::ChoosePlayerParameterTask;
use super::tasks::tasks::params::ClearParamsTask;
use super::tasks::tasks::remove_action_points::RemoveActionPointsTask;
use super::tasks::tasks::replace_hand::ReplaceHandTask;

// Emit logs like "Waiting for challenges..."

fn create_roll_for_ability_task(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card: &Card,
	ability: &HeroAbility,
) -> Box<dyn PlayerTask> {
	Box::new(AddTasks {
		tasks: vec![
			Box::new(CardUsedTask::new(player_index, card.id)),
			Box::new(DoRollTask::new(RollState::create(
				context,
				game,
				player_index,
				ability.to_consequences(),
				RollReason::UseHeroAbility(card.card_type),
			))) as Box<dyn PlayerTask>,
		],
	})
}

fn create_place_hero_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	ability: &HeroAbility,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		ChoiceDisplay {
			display_type: card_path.display().to_highlight(),
			label: format!(
				"Place {} in your party.",
				game.card(card_path).get_spec().label
			),
		},
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			Box::new(OfferChallengesTask::new(OfferChallengesState::new(
				player_index,
				RollConsequences {
					success: RollConsequence {
						condition: Condition::challenge_denied(),
						tasks: vec![
							card_path.get_place_task(),
							create_roll_for_ability_task(
								context,
								game,
								player_index,
								game.card(card_path),
								ability,
							),
						],
					},
					loss: Some(RollConsequence {
						condition: Condition::challenge_sustained(),
						tasks: vec![card_path.get_discard_task()],
					}),
				},
				ChallengeReason::PlaceHeroCard(game.card(card_path).card_type),
			))) as Box<dyn PlayerTask>,
		],
	)
}

pub fn create_place_item_task(players_with_stacks: Vec<ids::PlayerIndex>) -> Box<dyn PlayerTask> {
	Box::new(AddTasks {
		tasks: vec![ChoosePlayerParameterTask::one_of(
			TaskParamName::PlayerToGiveItem,
			"Choose a player to give this item to.",
			players_with_stacks,
		)],
	})
}

pub fn create_place_item_challenge_offer(
	game: &Game,
	player_index: ids::PlayerIndex,
	card: &Card,
	_item_modifier: &ItemModifier,
	players_with_stacks: Vec<ids::PlayerIndex>,
) -> Box<dyn PlayerTask> {
	let place_item = create_place_item_task(players_with_stacks);
	if game.players[player_index].has_modifier(PlayerModifier::ItemsCannotBeChallenged) {
		return place_item;
	}
	Box::new(OfferChallengesTask::new(OfferChallengesState::new(
		player_index,
		RollConsequences {
			success: RollConsequence {
				condition: Condition::challenge_denied(),
				tasks: vec![place_item],
			},
			loss: Some(RollConsequence {
				condition: Condition::challenge_sustained(),
				tasks: vec![Box::new(MoveCardTask {
					source: DeckPath::Hand(player_index),
					destination: DeckPath::Discard,
					card_id: card.id,
				}) as Box<dyn PlayerTask>],
			}),
		},
		ChallengeReason::PlaceHeroCard(card.card_type),
	))) as Box<dyn PlayerTask>
}

pub fn create_place_item_choice(
	game: &Game,
	placer_index: ids::PlayerIndex,
	id: ids::ChoiceId,
	card: &Card,
	display_type: ChoiceDisplayType,
	item_modifier: &ItemModifier,
	players_with_stacks: Vec<ids::PlayerIndex>,
) -> TasksChoice {
	TasksChoice::new(
		id,
		ChoiceDisplay {
			display_type,
			label: format!("Place item {}.", card.label()),
		},
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			create_place_item_challenge_offer(
				game,
				placer_index,
				card,
				item_modifier,
				players_with_stacks,
			),
		],
	)
}

fn create_cast_magic_choice(
	game: &Game,
	player_index: ids::PlayerIndex,
	id: ids::ChoiceId,
	card_path: CardPath,
	spell: MagicSpell,
) -> TasksChoice {
	TasksChoice::new(
		id,
		ChoiceDisplay {
			display_type: card_path.display().to_highlight(),
			label: format!("Cast {}", game.card(card_path).get_spec().label),
		},
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			card_path.get_discard_task(),
			Box::new(OfferChallengesTask::new(OfferChallengesState::new(
				player_index,
				RollConsequences {
					success: RollConsequence {
						condition: Condition::challenge_denied(),
						tasks: vec![Box::new(MagicTask::new(spell)) as Box<dyn PlayerTask>],
					},
					loss: None,
				},
				ChallengeReason::CastMagic(game.card(card_path).card_type),
			))) as Box<dyn PlayerTask>,
		],
	)
}

fn create_draw_choice(id: ids::ChoiceId) -> TasksChoice {
	TasksChoice::new(
		id,
		ChoiceDisplay {
			display_type: ChoiceDisplayType::HighlightPath(DisplayPath::DeckAt(DeckPath::Draw)),
			label: "Draw a card.".to_string(),
		},
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			Box::new(DrawTask { number_to_draw: 1 }),
		],
	)
}

fn create_replace_hand_choice(id: ids::ChoiceId) -> TasksChoice {
	TasksChoice::new(
		id,
		ChoiceDisplay {
			display_type: ChoiceDisplayType::HighlightPath(DisplayPath::DeckAt(DeckPath::Discard)),
			label: "Replace your hand with 5 new cards.".to_string(),
		},
		vec![
			Box::new(RemoveActionPointsTask::new(3)),
			Box::new(ReplaceHandTask {}),
		],
	)
}

fn create_forfeit_choice(
	game: &mut Game,
	player_index: ids::PlayerIndex,
	id: ids::ChoiceId,
) -> TasksChoice {
	let current_amount_remaining = game.players[player_index].get_remaining_action_points();
	TasksChoice::new(
		id,
		ChoiceDisplay {
			display_type: ChoiceDisplayType::Forfeit,
			label: "Do nothing this turn".to_string(),
		},
		vec![Box::new(RemoveActionPointsTask::new(current_amount_remaining)) as Box<dyn PlayerTask>],
	)
}

fn create_roll_for_ability_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	ability: &HeroAbility,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		ChoiceDisplay {
			display_type: card_path.display().to_highlight(),
			label: format!("Use {}'s ability", game.card(card_path).get_spec().label),
		},
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			create_roll_for_ability_task(context, game, player_index, game.card(card_path), ability),
		],
	)
}

fn create_attack_monster_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	monster: &MonsterSpec,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		ChoiceDisplay {
			display_type: card_path.display().to_highlight(),
			label: format!("Attack {}", game.card(card_path).get_spec().label),
		},
		vec![
			Box::new(RemoveActionPointsTask::new(2)) as Box<dyn PlayerTask>,
			Box::new(DoRollTask::new(RollState::create(
				context,
				game,
				player_index,
				monster.get_consequences(card_path.get_card_id()),
				RollReason::AttackMonster(game.card(card_path).card_type),
			))) as Box<dyn PlayerTask>,
		],
	)
}

fn create_hand_action_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
) -> Option<TasksChoice> {
	if let Some(spell) = game.card(card_path).get_spec().spell.as_ref() {
		return Some(create_cast_magic_choice(
			game,
			player_index,
			context.id_generator.generate(),
			card_path,
			*spell,
		));
	}
	if let Some(ability) = game.card(card_path).get_spec().hero_ability.as_ref() {
		return Some(create_place_hero_choice(
			context,
			game,
			player_index,
			card_path,
			ability,
		));
	}
	if let Some(modifier) = game.card(card_path).get_spec().card_modifier.as_ref() {
		let players_with_stacks = game.players_with_stacks();
		if !players_with_stacks.is_empty() {
			return Some(create_place_item_choice(
				game,
				player_index,
				context.id_generator.generate(),
				game.card(card_path),
				card_path.display().to_highlight(),
				modifier,
				players_with_stacks,
			));
		}
	}
	None
}

fn create_party_action_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
) -> Option<TasksChoice> {
	if game.players[player_index].was_card_played(&game.card(card_path).id) {
		return None;
	}
	if let Some(ability) = game.card(card_path).get_spec().hero_ability.as_ref() {
		return Some(create_roll_for_ability_choice(
			context,
			game,
			player_index,
			card_path,
			ability,
		));
	}
	if matches!(
		game.card(card_path).card_type,
		SlayCardSpec::PartyLeader(HeroType::Thief)
	) {
		return Some(TasksChoice::new(
			context.id_generator.generate(),
			ChoiceDisplay {
				display_type: ChoiceDisplayType::HighlightPath(DisplayPath::CardAt(card_path)),
				label: String::from("Use the Shadow Claw to pull a card."),
			},
			vec![
				Box::new(RemoveActionPointsTask::new(1)),
				Box::new(CardUsedTask::new(player_index, card_path.get_card_id())),
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::ShadowClawVictim,
					"Choose a player to steal from.",
				),
				PullFromTask::create(TaskParamName::ShadowClawVictim),
				ClearParamsTask::create(),
			],
		));
	}
	None
}

pub fn assign_action_choices(context: &mut GameBookKeeping, game: &mut Game) {
	// let player_index = game.active_player_index();
	let player_index = game.current_player().player_index;
	let remaining_action_points = game.current_player().get_remaining_action_points();
	let mut options: Vec<TasksChoice> = Vec::new();
	let default_choice = context.id_generator.generate();
	options.push(create_forfeit_choice(game, player_index, default_choice));
	options.push(create_draw_choice(context.id_generator.generate()));
	if remaining_action_points >= 3 {
		options.push(create_replace_hand_choice(context.id_generator.generate()));
	}
	if remaining_action_points >= 2 {
		for monster_card in game.monsters.tops() {
			if let Some(monster) = monster_card.monster_spec() {
				options.push(create_attack_monster_choice(
					context,
					game,
					player_index,
					CardPath::TopCardIn(DeckPath::ActiveMonsters, monster_card.id),
					&monster,
				));
			}
		}
	}

	for card_path in game.current_player().hand.top_paths() {
		if let Some(hand_choice) = create_hand_action_choice(context, game, player_index, card_path) {
			options.push(hand_choice);
		}
	}
	for card_path in game.current_player().party.top_paths() {
		if let Some(hand_choice) = create_party_action_choice(context, game, player_index, card_path) {
			options.push(hand_choice);
		}
	}
	{
		if let Some(hand_choice) = create_party_action_choice(
			context,
			game,
			player_index,
			CardPath::Leader(player_index, game.players[player_index].leader.id),
		) {
			options.push(hand_choice);
		}
	}

	log::info!(
		"Assigning {} actions to player at index {}",
		options.len(),
		player_index
	);

	game.current_player_mut().choices = Some(Choices {
		instructions: "Please choose an action".to_string(),
		options,
		default_choice: Some(default_choice),
		timeline: deadlines::get_action_point_choice_deadline(),
	});
}
