use std::collections::VecDeque;

use yew::Callback;

use crate::slay::driver::AdvanceGameResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::message::Notification;
use crate::slay::state::game::{Game, GamePerspective};
use crate::slay::{driver, strategy};

use super::card_modal::CardModalInfo;

#[derive(Clone)]
pub struct AppState {
	pub context: GameBookKeeping,
	pub game: Game,
	pub my_player_index: ids::PlayerIndex,
	pub notifications: VecDeque<Notification>,
}

impl AppState {
	pub fn new() -> Self {
		// let object = JsValue::from("world");
		// log::info!("Hello {}", object.as_string().unwrap());

		let mut context = GameBookKeeping::new();
		let mut game = Game::new(&mut context);
		driver::initialize_game(&mut context, &mut game);
		let player_index = game.active_player_index();
		Self {
			context,
			game,
			my_player_index: player_index,
			notifications: Default::default(),
		}
	}

	pub fn perspective(&self) -> GamePerspective {
		self.game.to_player_perspective(Some(self.my_player_index))
	}

	fn make_selection(&mut self, choice_id: ids::ChoiceId) -> bool {
		let new_notifications = &mut Vec::new();
		{
			let mut notify = |n| new_notifications.push(n);
			driver::make_selection(
				&mut self.context,
				&mut self.game,
				self.my_player_index,
				choice_id,
				&mut notify,
			)
			.expect("oops");
		}

		self.notifications.extend(new_notifications.drain(..));

		match driver::advance_game(&mut self.context, &mut self.game).expect("uh oh") {
			AdvanceGameResult::GameOver => true, // Need to return that the game is complete...
			AdvanceGameResult::WaitingForPlayers => false,
		}
	}

	pub fn create_new_state(&self, choice_id: ids::ChoiceId) -> Self {
		let mut new_state = self.clone();
		new_state.make_selection(choice_id);

		let (player_index, _choice_id) =
			strategy::pick_a_random_choice(&mut new_state.context, &mut new_state.game)
				.expect("I knew it.");

		new_state.my_player_index = player_index;
		new_state
	}
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ChoiceState {
	pub highlighted_choice: Option<ids::ChoiceId>,
}

#[derive(Clone, PartialEq)]
pub struct GameCallbacks {
	pub choose: Option<Callback<ids::ChoiceId, ()>>,
	pub view_card: Callback<Option<CardModalInfo>, ()>,
}
