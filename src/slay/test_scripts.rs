use crate::slay::game_context::GameBookKeeping;

use super::ids;
use super::specification::{self, CardType};
use super::state::deck::Deck;
use super::state::game::Game;
use super::state::player::Player;
use super::state::stack::{Card, Stack};

pub fn initialize_empty_game(context: &mut GameBookKeeping, game: &mut Game) {
	specification::get_card_specs().iter().for_each(|spec| {
		for _ in 0..spec.repeat {
			let stack = Stack::new(Card::new(context.id_generator.generate(), spec.to_owned()));
			match spec.card_type {
				CardType::PartyLeader(_) => game.leaders.add(stack),
				CardType::Monster => game.monsters.add(stack),
				_ => game.draw.add(stack),
			};
		}
	});

	for player_index in 0..4 {
		let player = Player::new(
			&mut context.id_generator,
			format!("Unnamed bot {}", player_index + 1),
			player_index,
			game.leaders.deal().top,
		);
		game.players.push(player);
	}

	game.set_active_player(0);
	// game.current_player_mut().remaining_action_points = 3;
}

fn find_hero_card(deck: &Deck) -> Option<ids::CardId> {
	deck
		.iter()
		.filter(|stack| match stack.top.spec.card_type {
			CardType::Hero(_) => true,
			_ => false,
		})
		.map(|stack| stack.top.id)
		.next()
}

#[cfg(test)]
mod tests {
	use crate::slay::actions;
	use crate::slay::game_context::GameBookKeeping;
	use crate::slay::state::deck::DeckPath;
	use crate::slay::state::game::Game;

	#[test]
	pub fn test_add() {
		let context = &mut GameBookKeeping::new();
		let game = &mut Game::new(context);

		super::initialize_empty_game(context, game);
		let hero_card_id = super::find_hero_card(&game.draw).unwrap();
		game
			.move_card(DeckPath::Draw, DeckPath::Hand(0), hero_card_id)
			.unwrap();
		actions::assign_action_choices(context, game);
		assert!(game.players[0].choices.is_some());
		if let Some(choices) = game.players[0].choices.as_ref() {
			assert_eq!(choices.options.len(), 3);
		}

		// make_selection(context, game, player_id, choice_id)?;
		// advance_game(context, game)?
		assert_eq!(1 + 2, 3);
	}
}
