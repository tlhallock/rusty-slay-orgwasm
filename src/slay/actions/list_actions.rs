use crate::slay::actions::attack;
use crate::slay::actions::cast_magic;
use crate::slay::actions::default_actions;
use crate::slay::actions::place_hero;
use crate::slay::actions::place_item;
use crate::slay::actions::roll_for_ability;
use crate::slay::choices::Action;
use crate::slay::choices::CardPath;
use crate::slay::choices::Choice;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specification::HeroType;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::specs::monster;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::player::HeroTypeCounter;
use crate::slay::tasks::core::pull::PullFromTask;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::card_used::CardUsedTask;
use crate::slay::tasks::tasks::params::ChoosePlayerParameterTask;
use crate::slay::tasks::tasks::params::ClearParamsTask;
use crate::slay::tasks::tasks::remove_action_points::RemoveActionPointsTask;

fn create_hand_action_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
) -> Option<TasksChoice> {
	// unnecessary...
	let id = context.id_generator.generate();
	match game.card(card_path).card_type {
		SlayCardSpec::HeroCard(hero_card) => Some(place_hero::create_place_hero_choice(
			context,
			game,
			player_index,
			card_path,
			hero_card,
		)),
		SlayCardSpec::Item(item_card) => place_item::create_place_item_choice(
			context,
			game,
			player_index,
			id,
			game.card(card_path),
			card_path.display().to_highlight(),
			item_card,
		),
		SlayCardSpec::MagicCard(spell) => Some(cast_magic::create_cast_magic_choice(
			game,
			player_index,
			id,
			card_path,
			spell,
		)),
		SlayCardSpec::MonsterCard(_) | SlayCardSpec::PartyLeader(_) => {
			unreachable!();
		}
		_ => None,
	}
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
		return roll_for_ability::create_roll_for_ability_choice(
			context,
			game,
			player_index,
			card_path,
			hero_card,
		);
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
				ChoosePlayerParameterTask::exclude_self(TaskParamName::ShadowClawVictim),
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
	options.push(default_actions::create_forfeit_choice(
		game,
		player_index,
		default_choice,
	));
	options.push(default_actions::create_draw_choice(
		context.id_generator.generate(),
	));
	if remaining_action_points >= 3 {
		options.push(default_actions::create_replace_hand_choice(
			context.id_generator.generate(),
		));
	}
	if remaining_action_points >= 2 {
		for monster_card in game.monsters.tops() {
			// move this into attack.rs
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
				options.push(attack::create_attack_monster_choice(
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

	game.current_player_mut().choose(Choices {
		choices_type: ChoicesType::SpendActionPoints,
		options,
		default_choice: Some(default_choice),
		timeline: deadlines::get_action_point_choice_deadline(),
	});
}
