use crate::slay::choices::CardPath;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::modifiers::PlayerModifier;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::offer::OfferChallengesState;
use crate::slay::showdown::roll::ChallengeReason;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::showdown::roll_state::RollState;
use crate::slay::specification::HeroType;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::specs::magic::MagicSpell;
use crate::slay::specs::monster;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::player::HeroTypeCounter;
use crate::slay::state::stack::Card;
use crate::slay::tasks::core::draw::DrawTask;
use crate::slay::tasks::core::pull::PullFromTask;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::add_tasks::AddTasks;
use crate::slay::tasks::tasks::card_used::CardUsedTask;
use crate::slay::tasks::tasks::do_roll::DoRollTask;
use crate::slay::tasks::tasks::magic::MagicTask;
use crate::slay::tasks::tasks::move_card::MoveCardTask;
use crate::slay::tasks::tasks::offer_challenges::OfferChallengesTask;
use crate::slay::tasks::tasks::params::ChoosePlayerParameterTask;
use crate::slay::tasks::tasks::params::ClearParamsTask;
use crate::slay::tasks::tasks::remove_action_points::RemoveActionPointsTask;
use crate::slay::tasks::tasks::replace_hand::ReplaceHandTask;

use super::choices::Action;
use super::choices::Choice;
use super::specs::hero::HeroAbilityType;
use super::specs::items::AnotherItemType;
use super::specs::monster::Monster;

// Emit logs like "Waiting for challenges..."

fn create_roll_for_ability_task(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card: &Card,
	hero_card: HeroAbilityType,
) -> Box<dyn PlayerTask> {
	Box::new(AddTasks {
		tasks: vec![
			Box::new(CardUsedTask::new(player_index, card.id)),
			Box::new(DoRollTask::new(RollState::create(
				context,
				game,
				player_index,
				hero_card.to_consequences(),
				RollReason::UseHeroAbility(card.card_type),
			))) as Box<dyn PlayerTask>,
		],
	})
}

pub fn create_place_hero_challenges(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	hero_card: HeroAbilityType,
) -> Box<dyn PlayerTask> {
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
						hero_card,
					),
				],
			},
			loss: Some(RollConsequence {
				condition: Condition::challenge_sustained(),
				tasks: vec![card_path.get_discard_task()],
			}),
		},
		ChallengeReason::PlaceHeroCard(game.card(card_path).card_type),
	))) as Box<dyn PlayerTask>
}

fn create_place_hero_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	hero_card: HeroAbilityType,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		Choice::UseActionPoints(Action::PlaceHeroInParty(hero_card)),
		card_path.display().to_highlight(),
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			create_place_hero_challenges(context, game, player_index, card_path, hero_card),
		],
	)
}

pub fn create_place_item_task(players_with_stacks: Vec<ids::PlayerIndex>) -> Box<dyn PlayerTask> {
	Box::new(AddTasks {
		tasks: vec![
			ChoosePlayerParameterTask::one_of(
				TaskParamName::PlayerToGiveItem,
				"Choose a player to give this item to.",
				players_with_stacks,
			),
			ClearParamsTask::create(),
		],
	})
}

pub fn create_place_item_challenge_offer(
	game: &Game,
	player_index: ids::PlayerIndex,
	card: &Card,
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
	item_card: AnotherItemType,
	players_with_stacks: Vec<ids::PlayerIndex>,
) -> TasksChoice {
	TasksChoice::new(
		id,
		Choice::UseActionPoints(Action::PlaceItem(item_card)),
		display_type,
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			create_place_item_challenge_offer(game, placer_index, card, players_with_stacks),
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
		Choice::UseActionPoints(Action::CastMagic(spell)),
		card_path.display().to_highlight(),
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
		Choice::UseActionPoints(Action::Draw),
		ChoiceDisplayType::HighlightPath(DisplayPath::DeckAt(DeckPath::Draw)),
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			DrawTask::create(1),
		],
	)
}

fn create_replace_hand_choice(id: ids::ChoiceId) -> TasksChoice {
	TasksChoice::new(
		id,
		Choice::UseActionPoints(Action::Draw),
		ChoiceDisplayType::HighlightPath(DisplayPath::DeckAt(DeckPath::Discard)),
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
		Choice::UseActionPoints(Action::Draw),
		ChoiceDisplayType::Forfeit,
		vec![Box::new(RemoveActionPointsTask::new(current_amount_remaining)) as Box<dyn PlayerTask>],
	)
}

fn create_roll_for_ability_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	hero_card: HeroAbilityType,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		Choice::UseActionPoints(Action::RollForAbility(hero_card)),
		card_path.display().to_highlight(),
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			create_roll_for_ability_task(context, game, player_index, game.card(card_path), hero_card),
		],
	)
}

fn create_attack_monster_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	monster: Monster,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		Choice::UseActionPoints(Action::AttackMonster(monster)),
		card_path.display().to_highlight(),
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
	if let SlayCardSpec::HeroCard(hero_card) = game.card(card_path).card_type {
		return Some(create_place_hero_choice(
			context,
			game,
			player_index,
			card_path,
			hero_card,
		));
	}
	if let SlayCardSpec::Item(item_card) = game.card(card_path).card_type {
		// .get_spec().card_modifier.as_ref() {
		let players_with_stacks = game.players_with_stacks();
		if !players_with_stacks.is_empty() {
			return Some(create_place_item_choice(
				game,
				player_index,
				context.id_generator.generate(),
				game.card(card_path),
				card_path.display().to_highlight(),
				item_card,
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
	if let SlayCardSpec::HeroCard(hero_card) = game.card(card_path).card_type {
		return Some(create_roll_for_ability_choice(
			context,
			game,
			player_index,
			card_path,
			hero_card,
		));
	}
	if matches!(
		game.card(card_path).card_type,
		SlayCardSpec::PartyLeader(HeroType::Thief)
	) {
		return Some(TasksChoice::new(
			context.id_generator.generate(),
			Choice::UseActionPoints(Action::UseLeader(HeroType::Thief)),
			ChoiceDisplayType::HighlightPath(DisplayPath::CardAt(card_path)),
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
			if let SlayCardSpec::MonsterCard(monster) = monster_card.card_type {
				// /////////////////////////////////////////////////////////////////////
				//  Just write some unit tests for this....
				// /////////////////////////////////////////////////////////////////////
				let hero_type_counts = &mut HeroTypeCounter::new();
				game.players[player_index].count_hero_types(hero_type_counts);
				let requirements = &mut monster.create_spec().requirements.to_vec();
				/*				println!(
					"Does\n\t\tleader={:?}\n\t\tparty={:?}\nsatisfy requirements\n\t\t{:?}?",
					game.players[player_index].leader.card_type,
					game.players[player_index]
						.party
						.stacks()
						.map(|stack| stack.get_hero_type())
						.collect::<Vec<_>>(),
					requirements,
				); */
				if !monster::player_satisfies_requirements(hero_type_counts, requirements) {
					//					println!("NO");
					continue;
				}
				//				println!("YES");
				// /////////////////////////////////////////////////////////////////////
				options.push(create_attack_monster_choice(
					context,
					game,
					player_index,
					CardPath::TopCardIn(DeckPath::ActiveMonsters, monster_card.id),
					monster,
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
		choices_type: ChoicesType::SpendActionPoints,
		options,
		default_choice: Some(default_choice),
		timeline: deadlines::get_action_point_choice_deadline(),
	});
}
