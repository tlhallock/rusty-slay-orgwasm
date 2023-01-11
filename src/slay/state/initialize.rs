use std::collections::HashSet;

use enum_iterator::all;
use rand::prelude::SliceRandom;
use rand::Rng;

use crate::slay::game_context::GameBookKeeping;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::state::game::Game;
use crate::slay::{actions, ids};

use super::deck::DeckPath;
use super::player::Player;
use super::stack::{Card, Stack};

fn initialize_global_decks(context: &mut GameBookKeeping, game: &mut Game) {
	let (draw_capacity, leaders_capacity, monsters_capacity) = (101, 10, 20);
	let mut draw = Vec::with_capacity(draw_capacity);
	let mut leaders = Vec::with_capacity(leaders_capacity);
	let mut monsters = Vec::with_capacity(monsters_capacity);
	all::<SlayCardSpec>().for_each(|spec_type| {
		let spec = spec_type.get_card_spec_creation();

		for _ in 0..spec.repeat {
			let stack = Stack::new(Card::new(
				context.id_generator.generate(),
				spec_type.to_owned(),
			));

			match spec.get_initial_deck() {
				DeckPath::Draw => draw.push(stack),
				DeckPath::PartyLeaders => leaders.push(stack),
				DeckPath::NextMonsters => monsters.push(stack),
				_ => unreachable!(),
			};
		}
	});
	if draw_capacity != draw.len() {
		println!("Draw's capacity should be {}", draw.len())
	}
	if leaders_capacity != draw.len() {
		println!("Leader's capacity should be {}", leaders.len())
	}
	if monsters_capacity != draw.len() {
		println!("Monster's capacity should be {}", monsters.len())
	}

	[&mut draw, &mut leaders, &mut monsters]
		.iter_mut()
		.for_each(|deck| deck.shuffle(&mut context.rng));

	game.draw.extend(draw.drain(..));
	game.next_monsters.extend(monsters.drain(..));
	game.leaders.extend(leaders.drain(..));
}

fn initialize_players(_context: &mut GameBookKeeping, game: &mut Game) {
	for player_index in 0..4 {
		let player = Player::new(
			format!("Unnamed bot {}", player_index + 1),
			player_index,
			game.leaders.deal().top,
		);
		game.players.push(player);
	}
}

pub fn initialize_game(context: &mut GameBookKeeping, game: &mut Game) {
	initialize_global_decks(context, game);
	initialize_players(context, game);
	game.monsters.extend(game.next_monsters.drain(0..3));

	for player_index in 0..game.number_of_players() {
		let drain = game.draw.drain(0..5);
		game.players[player_index].hand.extend(drain);
	}

	// initialize the first first random player
	game.set_active_player(context.rng.gen_range(0..game.number_of_players()));
	game.current_player_mut().turn_begin();
	actions::assign_action_choices(context, game);
}

fn randomly_initialize_hand(
	context: &mut GameBookKeeping,
	game: &mut Game,
	player_index: ids::PlayerIndex,
) {
	let number_of_hand_cards = context.rng.gen_range(0..10);
	let drain = game.draw.drain(0..number_of_hand_cards);
	game.players[player_index].hand.extend(drain);
}

fn randomly_initialize_monsters(
	context: &mut GameBookKeeping,
	game: &mut Game,
	player_index: ids::PlayerIndex,
) {
	let number_of_monsters = context.rng.gen_range(0..3);
	let drain = game.next_monsters.drain(0..number_of_monsters);
	game.players[player_index].slain_monsters.extend(drain);

	// Need to add the buffs...
}

fn randomly_initialize_modifiers(
	_context: &mut GameBookKeeping,
	_game: &mut Game,
	_player_index: ids::PlayerIndex,
) {
	// TODO!
}

fn adding_card_would_mean_player_wins(
	game: &mut Game,
	player_index: ids::PlayerIndex,
	stack: &Stack,
) -> bool {
	if let Some(hero_type) = stack.get_hero_type() {
		let hero_types = &mut HashSet::new();
		game.players[player_index].collect_hero_types(hero_types);
		hero_types.insert(hero_type);
		hero_types.len() >= 6
	} else {
		false
	}
}

fn randomly_initialize_party(
	context: &mut GameBookKeeping,
	game: &mut Game,
	player_index: ids::PlayerIndex,
) {
	let number_of_party_cards = context.rng.gen_range(0..10);
	for _ in 0..number_of_party_cards {
		if let Some(stack) = game.draw.maybe_deal() {
			if adding_card_would_mean_player_wins(game, player_index, &stack) {
				game.draw.add(stack);
			} else {
				game.players[player_index].party.add(stack);
			}
		}
	}
}

pub fn initialize_game_to_random_state(context: &mut GameBookKeeping, game: &mut Game) {
	initialize_global_decks(context, game);
	initialize_players(context, game);
	game.monsters.extend(game.next_monsters.drain(0..3));

	for player_index in 0..game.number_of_players() {
		randomly_initialize_hand(context, game, player_index);
		randomly_initialize_party(context, game, player_index);
		randomly_initialize_modifiers(context, game, player_index);
		randomly_initialize_monsters(context, game, player_index);
	}

	// initialize the first first random player
	game.set_active_player(context.rng.gen_range(0..game.number_of_players()));
	game.current_player_mut().turn_begin();
	actions::assign_action_choices(context, game);
}
