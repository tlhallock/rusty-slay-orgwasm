use std::collections::HashSet;

use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::Choices;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;

// #[derive(Debug, Clone)]
// pub struct ReturnModifier {
// 	num: u32,
// 	include: Option<HashSet<ids::CardId>>,
// }

// impl Discard {
// 	pub fn new(num: u32) -> Self {
// 		Self { num, include: None }
// 	}
// 	pub fn discard_one_of(include: HashSet<ids::CardId>) -> Self {
// 		Self {
// 			num: 1,
// 			include: Some(include),
// 		}
// 	}

// 	pub fn should_include(&self, card_id: ids::CardId) -> bool {
// 		if let Some(include) = self.include {
// 			include.contains(&card_id)
// 		} else {
// 			true
// 		}
// 	}
// }
