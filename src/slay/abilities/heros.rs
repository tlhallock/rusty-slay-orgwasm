use super::discard::Discard;
use super::pull;
use std::collections::HashSet;

use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specification::HeroType;
use crate::slay::state::game::Game;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskParamName;
use crate::slay::tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct SlipperyPaws {}

impl SlipperyPaws {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for SlipperyPaws {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let first_card =
			game.card_param(player_index, &TaskParamName::SlipperyPawsVictimPulledCard1)?;
		let second_card =
			game.card_param(player_index, &TaskParamName::SlipperyPawsVictimPulledCard2)?;
		if first_card.is_none() || second_card.is_none() {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let mut include = HashSet::new();
		include.insert(first_card.unwrap());
		include.insert(second_card.unwrap());
		// include = std::iter::once(first_card.unwrap()).chain(std::iter::once(second_card.unwrap())).collect();
		// let exclude = game
		// 	.deck(DeckPath::Hand(player_index))
		// 	.other_cards(&include);
		game.players[player_index]
			.tasks
			.prepend(Box::new(Discard::discard_one_of(include)));
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"do slippery paws".to_owned()
	}
}

#[derive(Clone, Debug)]
pub struct Mimimeow {}

impl Mimimeow {
	pub fn create() -> Box<dyn PlayerTask> {
		Box::new(Self {}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for Mimimeow {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		for victim_index in 0..game.number_of_players() {
			if player_index == victim_index {
				continue;
			}
			if !game.players[player_index].has_hero_type(&HeroType::Thief) {
				continue;
			}
			pull::pull_a_random_card(context, game, player_index, victim_index);
		}
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"do slippery paws".to_owned()
	}
}

// #[derive(Debug, Clone, PartialEq, Eq, Copy)]
// pub enum Ability {
// 	PlunderingPuma,
// }

// pub fn do_hero_ability(
// 	_context: &mut GameBookKeeping, _game: &mut Game, _player_index: ids::PlayerIndex,
// 	ability: Ability,
// ) -> Vec<Box<dyn PlayerTask>> {
// 	match ability {
// 		// Plundering Puma
// 		Ability::PlunderingPuma => vec![
// 			Box::new(ChoosePlayerParameterTask {
// 				param_name: TaskParamName::PlayerToPullFrom,
// 				instructions: "Choose a player to pull from.".to_owned(),
// 			}) as Box<dyn PlayerTask>,
// 			Box::new(PullFromTask {
// 				pulled_index_param_name: TaskParamName::PlayerToPullFrom,
// 			}) as Box<dyn PlayerTask>,
// 			Box::new(PullFromTask {
// 				pulled_index_param_name: TaskParamName::PlayerToPullFrom,
// 			}) as Box<dyn PlayerTask>,
// 		],
// 		_ => todo!(),
// 	}
// }

// card_type: CardType::Hero(HeroType::Thief),
// label: "Plundering Puma".to_string(),
// image_path: "cards/heros/thief/plundering_puma.jpg".to_string(),
// description: "Pull 2 cards from another player's hand. That player may DRAW a card.".to_string(),
// hero_ability: Some(HeroAbility::new(Condition::ge(6))),
// ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Thief),
//     label: "Slippery Paws".to_string(),
//     image_path: "cards/heros/thief/slippery_paws.jpg".to_string(),
//     description: "Pull 2 cards from another player's hand, then DISCARD one of those cards.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(6))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Thief),
//     label: "Smooth Mimimeow".to_string(),
//     image_path: "cards/heros/thief/smooth_mimimeow.jpg".to_string(),
//     description: "Pull a card from the hand of each other player with a Thief in their Party.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(7))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Thief),
//     label: "Meowzio".to_string(),
//     image_path: "cards/heros/thief/meowzio.jpg".to_string(),
//     description: "Choose a player. STEAL a Hero card from that player's Party and pull a card from that player's hand.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(10))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Thief),
//     label: "Shurikitty".to_string(),
//     image_path: "cards/heros/thief/shurikitty.jpg".to_string(),
//     description: "DESTROY a Hero card. If that Hero card had an item card equipped to it, add that Item card to your hand instead of moving it to the discard pile.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(9))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Thief),
//     label: "Kit Napper".to_string(),
//     image_path: "cards/heros/thief/kit_napper.jpg".to_string(),
//     description: "Steal a Hero card.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(9))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Thief),
//     label: "Silent Shadow".to_string(),
//     image_path: "cards/heros/thief/silent_shadow.jpg".to_string(),
//     description: "Look at another player's hand. Choose a card and add it to your hand.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(8))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Thief),
//     label: "Sly Pickings".to_string(),
//     image_path: "cards/heros/thief/sly_pickings.jpg".to_string(),
//     description: "Pull a card from another player's hand. If that card is an Item card, you may play it immediately.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(6))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Gaurdian),
//     label: "Holy Curselifter".to_string(),
//     image_path: "cards/heros/guardian/holy_curse_lifter.jpg".to_string(),
//     description: "Return a Cursed Item card equipped to a Hero card in your Part to your hand.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(5))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Gaurdian),
//     label: "Iron Resolve".to_string(),
//     image_path: "cards/heros/guardian/iron_resolve.jpg".to_string(),
//     description: "Cards you play cannot be challenged for the rest of your turn.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(8))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Gaurdian),
//     label: "Calming Voice".to_string(),
//     image_path: "cards/heros/guardian/calming_voice.jpg".to_string(),
//     description: "Hero cards in your Party cannot be stolen until your next turn.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(9))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Gaurdian),
//     label: "Vibrant Glow".to_string(),
//     image_path: "cards/heros/guardian/vibrant_glow.jpg".to_string(),
//     description: "+5 to all of your rolls until the end of your turn.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(9))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Gaurdian),
//     label: "Mighty Blade".to_string(),
//     image_path: "cards/heros/guardian/mighty_blade.jpg".to_string(),
//     description: "Hero cards in your Party cannot be destroyed until your next turn.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(8))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Gaurdian),
//     label: "Radiant Horn".to_string(),
//     image_path: "cards/heros/guardian/radiant_horn.jpg".to_string(),
//     description: "Search the discard pile for a Modifier card and add it to your hand.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(6))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Gaurdian),
//     label: "Guiding Light".to_string(),
//     image_path: "cards/heros/guardian/guiding_light.jpg".to_string(),
//     description: "Search the discard pile for a Hero card and add it to your hand.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(7))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Gaurdian),
//     label: "Wise Shield".to_string(),
//     image_path: "cards/heros/guardian/wise_shield.jpg".to_string(),
//     description: "+3 to all of your rolls until the end of your turn.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(6))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Fighter),
//     label: "Qi Bear".to_string(),
//     image_path: "cards/heros/fighter/chi_bear.jpg".to_string(),
//     description: "DISCARD up to 3 cards. For each card discarded, DESTROY a Hero card.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(10))),
//     ..Default::default()
// },
// CardSpec {
//     card_type: CardType::Hero(HeroType::Fighter),
//     label: "Pan Chucks".to_string(),
//     image_path: "cards/heros/fighter/pan_chucks.jpg".to_string(),
//     description: "DRAW 2 cards. If at least one of those cards is a Challenge card, you may reveal it, then DESTROY a Hero card.".to_string(),
//     hero_ability: Some(HeroAbility::new(Condition::ge(8))),
//     ..Default::default()

#[derive(Debug, Clone)]
pub struct VictimDraws {
	pub param_name: TaskParamName,
	pub number_to_draw: usize,
}

impl VictimDraws {
	pub fn create(param_name: TaskParamName) -> Box<dyn PlayerTask> {
		Box::new(Self {
			param_name,
			number_to_draw: 1,
		}) as Box<dyn PlayerTask>
	}
}

impl PlayerTask for VictimDraws {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let victim_index = game.players[player_index]
			.tasks
			.get_player_value(&self.param_name);
		if let Some(_victim_index) = victim_index {
			game.replentish_for(self.number_to_draw);
			game.players[player_index]
				.hand
				.extend(game.draw.drain(0..self.number_to_draw));
			Ok(TaskProgressResult::TaskComplete)
		} else {
			Ok(TaskProgressResult::NothingDone)
		}
	}

	fn label(&self) -> String {
		format!("The victim can now draw {} cards.", self.number_to_draw)
	}
}
