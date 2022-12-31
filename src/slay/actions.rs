use crate::slay::choices;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context;
use crate::slay::ids;
use crate::slay::showdown::consequences;
use crate::slay::specification;
use crate::slay::state;
use crate::slay::tasks;

use super::choices::ChoiceDisplay;
use super::choices_rewrite::TasksChoice;
use super::errors::SlayError;
use super::game_context::GameBookKeeping;
use super::showdown::base::ShowDown;
use super::showdown::common::ChallengeReason;
use super::showdown::common::Roll;
use super::showdown::consequences::RollConsequenceRenameMe;
use super::showdown::consequences::RollConsequences;
use super::showdown::offer::OfferChallengesState;
use super::showdown::roll_state::RollReason;
use super::showdown::roll_state::RollState;
use super::specification::CardType;
use super::state::Card;
use super::state::Game;
use super::tasks::PlayerTask;
use super::tasks::TaskProgressResult;

// Emit logs like "Waiting for challenges..."

fn create_roll_for_ability_task(
	context: &mut GameBookKeeping, game: &Game, player_index: usize, card: &Card,
) -> DoRollTask {
	let condition = card
		.hero_ability()
		.as_ref()
		.unwrap()
		.success_condition
		.to_owned();
	let use_ability =
		Box::new(create_use_ability_task(context, game, player_index, card)) as Box<dyn PlayerTask>;
	DoRollTask::new(RollState::new(
		player_index,
		RollConsequences::new(
			player_index,
			vec![RollConsequenceRenameMe {
				condition,
				tasks: vec![use_ability],
			}],
		),
		Roll::create_from(&mut context.rng),
		game.number_of_players(),
		RollReason::UseHeroAbility(card.spec.to_owned()),
	))
}

fn create_place_hero_task(
	context: &mut GameBookKeeping, game: &Game, locator: choices::ChoiceLocator, card: &Card,
) -> TasksChoice {
	let player_index = locator.player_index;
	let roll_for_ability = Box::new(create_roll_for_ability_task(
		context,
		game,
		player_index,
		card,
	)) as Box<dyn tasks::PlayerTask>;
	TasksChoice::new(
		choices::ChoiceInformation::new(
			locator,
			ChoiceDisplay {
				highlight: Some(choices::DisplayPath::CardIn(
					state::DeckPath::Hand(player_index),
					card.id,
				)),
				arrows: vec![choices::DisplayArrow {
					source: choices::DisplayPath::CardIn(state::DeckPath::Hand(player_index), card.id),
					destination: choices::DisplayPath::DeckAt(state::DeckPath::Party(player_index)),
				}],
				label: format!("Place {} in your party.", card.label()),
				roll_modification_choice: None,
			},
		),
		vec![Box::new(OfferChallengesTask::new(OfferChallengesState::new(
			game.number_of_players(),
			player_index,
			consequences::RollConsequences::new(
				player_index,
				vec![
					consequences::RollConsequenceRenameMe {
						condition: consequences::Condition::challenge_denied(),
						tasks: vec![
							Box::new(tasks::MoveCardTask {
								source: state::DeckPath::Hand(player_index),
								destination: state::DeckPath::Party(player_index),
								card_id: card.id,
							}) as Box<dyn tasks::PlayerTask>,
							roll_for_ability,
						],
					},
					consequences::RollConsequenceRenameMe {
						condition: consequences::Condition::challenge_sustained(),
						tasks: vec![Box::new(tasks::MoveCardTask {
							source: state::DeckPath::Hand(player_index),
							destination: state::DeckPath::Discard,
							card_id: card.id,
						}) as Box<dyn tasks::PlayerTask>],
					},
				],
			),
			ChallengeReason::PlaceHeroCard(card.spec.to_owned()),
		))) as Box<dyn PlayerTask>],
	)
}

// TODO: Refactor this to use TasksChoice...
#[derive(Clone, Debug)]
struct PlaceItem {
	choice_information: choices::ChoiceInformation,
	card_id: ids::CardId,
}

impl PlaceItem {
	pub fn new(locator: choices::ChoiceLocator, card: &state::Card) -> Self {
		Self {
			card_id: card.id,
			choice_information: choices::ChoiceInformation::new(
				locator.to_owned(),
				choices::ChoiceDisplay {
					highlight: Some(choices::DisplayPath::CardIn(
						state::DeckPath::Hand(locator.player_index),
						card.id,
					)),
					// Could have it going to each other deck?
					// arrows: vec![],
					label: format!("Use the ability {}, discarding the card.", card.label()),
					..Default::default()
				},
			),
		}
	}
}

// TODO: Refactor this to use TasksChoice...
impl choices::Choice for PlaceItem {
	fn select(
		&mut self, _context: &mut game_context::GameBookKeeping, _game: &mut state::Game,
	) -> SlayResult<()> {
		todo!()
	}

	fn get_choice_information(&self) -> &choices::ChoiceInformation {
		&self.choice_information
	}
}

// TODO: Refactor this to use TasksChoice...
#[derive(Clone, Debug)]
struct CastMagic {
	choice_information: choices::ChoiceInformation,
	card_id: ids::CardId,
}

impl CastMagic {
	pub fn new(locator: choices::ChoiceLocator, card: &state::Card) -> Self {
		Self {
			card_id: card.id,
			choice_information: choices::ChoiceInformation::new(
				locator.to_owned(),
				choices::ChoiceDisplay {
					highlight: Some(choices::DisplayPath::CardIn(
						state::DeckPath::Hand(locator.player_index),
						card.id,
					)),
					arrows: vec![choices::DisplayArrow {
						source: choices::DisplayPath::CardIn(
							state::DeckPath::Hand(locator.player_index),
							card.id,
						),
						destination: choices::DisplayPath::DeckAt(state::DeckPath::Discard),
					}],
					// Could show arrows to each of the possibilities
					label: format!("Cast {}.", card.label()),
					roll_modification_choice: None,
				},
			),
		}
	}
}

impl choices::Choice for CastMagic {
	fn select(
		&mut self, _context: &mut game_context::GameBookKeeping, _game: &mut state::Game,
	) -> SlayResult<()> {
		todo!()
	}

	fn get_choice_information(&self) -> &choices::ChoiceInformation {
		&self.choice_information
	}
}

// #[derive(Clone, Debug)]
// struct UseAbility {
//     choice_information: choices::ChoiceInformation,
//     card_id: ids::CardId,
// }

// impl UseAbility {
//     pub fn new(locator: choices::ChoiceLocator, card: &state::Card) -> Self {
//         Self {
//             card_id: card.id,
//             choice_information: choices::ChoiceInformation::new(
//                 locator.to_owned(),
//                 choices::ChoiceDisplay {
//                     highlight: Some(choices::DisplayPath::CardIn(
//                         state::DeckPath::Party(locator.player_index),
//                         card.id,
//                     )),
//                     label: format!("Use hero {}'s ability.", card.label()),
//                     ..Default::default()
//                 },
//             ),
//         }
//     }
// }

// TODO: Refactor this to use TasksChoice...
#[derive(Clone, Debug)]
struct DrawChoice {
	choice_information: choices::ChoiceInformation,
}

impl DrawChoice {
	pub fn new(locator: choices::ChoiceLocator) -> Self {
		Self {
			choice_information: choices::ChoiceInformation::new(
				locator.to_owned(),
				choices::ChoiceDisplay {
					highlight: Some(choices::DisplayPath::DeckAt(state::DeckPath::Draw)),
					arrows: vec![choices::DisplayArrow {
						source: choices::DisplayPath::DeckAt(state::DeckPath::Draw),
						destination: choices::DisplayPath::DeckAt(state::DeckPath::Hand(locator.player_index)),
					}],
					label: format!("Draw a card."),
					roll_modification_choice: None,
				},
			),
		}
	}
}

impl choices::Choice for DrawChoice {
	fn select(
		&mut self, _context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<()> {
		let stack = game.draw.deal();
		game.players[self.choice_information.player_index()]
			.hand
			.stacks
			.push_back(stack);
		game.players[self.choice_information.player_index()].remaining_action_points -= 1;
		Ok(())
	}

	fn get_choice_information(&self) -> &choices::ChoiceInformation {
		&self.choice_information
	}
}

#[derive(Clone, Debug)]
struct ReplaceHandChoice {
	choice_information: choices::ChoiceInformation,
}

impl ReplaceHandChoice {
	pub fn new(locator: choices::ChoiceLocator) -> Self {
		Self {
			choice_information: choices::ChoiceInformation::new(
				locator.to_owned(),
				choices::ChoiceDisplay {
					highlight: Some(choices::DisplayPath::DeckAt(state::DeckPath::Discard)),
					arrows: vec![
						choices::DisplayArrow {
							source: choices::DisplayPath::DeckAt(state::DeckPath::Hand(locator.player_index)),
							destination: choices::DisplayPath::DeckAt(state::DeckPath::Discard),
						},
						choices::DisplayArrow {
							source: choices::DisplayPath::DeckAt(state::DeckPath::Draw),
							destination: choices::DisplayPath::DeckAt(state::DeckPath::Hand(
								locator.player_index,
							)),
						},
					],
					label: "Replace your hand with 5 new cards.".to_string(),
					roll_modification_choice: None,
				},
			),
		}
	}
}

impl choices::Choice for ReplaceHandChoice {
	fn select(
		&mut self, _context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<()> {
		let player = &mut game.players[self.choice_information.player_index()];
		player.remaining_action_points -= 3;
		game.discard.extend(player.hand.drain(..));
		player.hand.extend(game.draw.drain(0..5));
		Ok(())
	}

	fn get_choice_information(&self) -> &choices::ChoiceInformation {
		&self.choice_information
	}
}

// #[derive(Clone, Debug)]
// pub struct AttackChoice {
//     choice_information: choices::ChoiceInformation,
//     monster_card_id: ids::CardId,
//     monster_spec: specification::MonsterSpec,
// }

// impl AttackChoice {
//     pub fn new() -> Self {
//         Self {
//             monster_card_id: monster_card.id,
//             monster_spec: monster_card.monster_spec().as_ref().unwrap().to_owned(),
//             choice_information:
//         }
//     }
// }

// impl choices::Choice for AttackChoice {
//     fn select(
//         &mut self,
//         context: &mut game_context::GameBookKeeping,
//         game: &mut state::Game,
//     ) -> SlayResult<()> {
//         let player_index = self.choice_information.player_index();
//         let mut roll = ;
//         roll.assign_all_choices(context, game);
//         game.showdown.roll(roll);
//         game.players[player_index].remaining_action_points -= 2; // Should be a remove action points task...
//         Ok(())
//     }

//     fn get_choice_information(&self) -> &choices::ChoiceInformation {
//         &self.choice_information
//     }
// }

fn create_forfeit_choice(
	_context: &mut GameBookKeeping, game: &mut Game, locator: choices::ChoiceLocator,
) -> TasksChoice {
	let player_index = locator.player_index;
	let current_amount_remaining = game.players[player_index].remaining_action_points;
	TasksChoice::new(
		choices::ChoiceInformation::new(
			locator,
			choices::ChoiceDisplay {
				label: "Do nothing this turn".to_string(),
				..Default::default()
			},
		),
		vec![Box::new(RemoveActionPointsTask::new(
			player_index,
			current_amount_remaining,
		)) as Box<dyn PlayerTask>],
	)
}

fn create_use_ability_task(
	_context: &mut GameBookKeeping, _game: &Game, _player_index: usize, _card: &Card,
) -> UseAbility {
	UseAbility::new()
}

fn create_use_ability_choice(
	context: &mut GameBookKeeping, game: &Game, locator: choices::ChoiceLocator, card: &Card,
) -> TasksChoice {
	let player_index = locator.player_index;
	let _use_ability = Box::new(create_roll_for_ability_task(
		context,
		game,
		player_index,
		card,
	)) as Box<dyn PlayerTask>;
	TasksChoice::new(
		choices::ChoiceInformation::new(
			locator.to_owned(),
			choices::ChoiceDisplay {
				// TODO
				..Default::default()
			},
		),
		vec![
			Box::new(CardUsedTask::new(player_index, card.id)),
			Box::new(RemoveActionPointsTask::new(player_index, 1)),
			// TODO
		],
	)
}

fn create_attack_monster_choice(
	context: &mut GameBookKeeping, game: &Game, locator: choices::ChoiceLocator,
	monster_card: &state::Card,
) -> TasksChoice {
	let player_index = locator.player_index;
	TasksChoice::new(
		choices::ChoiceInformation::new(
			locator.to_owned(),
			choices::ChoiceDisplay {
				highlight: Some(choices::DisplayPath::DeckAt(state::DeckPath::Discard)),
				arrows: vec![choices::DisplayArrow {
					source: choices::DisplayPath::DeckAt(state::DeckPath::ActiveMonsters),
					destination: choices::DisplayPath::DeckAt(state::DeckPath::SlainMonsters(
						locator.player_index,
					)),
				}],
				label: format!("Attack {}", monster_card.label()),
				roll_modification_choice: None,
			},
		),
		vec![
			Box::new(DoRollTask::new(RollState::new(
				player_index,
				monster_card
					.spec
					.monster
					.as_ref()
					.unwrap()
					.create_consequences(player_index),
				Roll::create_from(&mut context.rng),
				game.number_of_players(),
				RollReason::AttackMonster(monster_card.spec.to_owned()),
			))) as Box<dyn PlayerTask>,
			Box::new(RemoveActionPointsTask::new(player_index, 2)) as Box<dyn PlayerTask>,
		],
	)
}

impl game_context::GameBookKeeping {
	pub fn locator(&mut self, player_index: usize) -> choices::ChoiceLocator {
		choices::ChoiceLocator {
			id: self.id_generator.generate(),
			player_index,
		}
	}
}

fn create_hand_action_choice(
	context: &mut GameBookKeeping, game: &Game, locator: choices::ChoiceLocator, card: &state::Card,
) -> Option<Box<dyn choices::Choice>> {
	match card.card_type() {
		CardType::Blank => None,
		CardType::Challenge => None,
		CardType::Modifier => None,
		CardType::PartyLeader(_) => unreachable!(),
		CardType::Monster => unreachable!(),
		CardType::Hero(_) => Some(Box::new(create_place_hero_task(
			context, game, locator, card,
		))),
		CardType::Item(_) => Some(Box::new(PlaceItem::new(locator, card))),
		CardType::Magic => Some(Box::new(CastMagic::new(locator, card))),
	}
}

fn create_party_action_choice(
	context: &mut GameBookKeeping, game: &Game, locator: choices::ChoiceLocator, card: &state::Card,
) -> Option<Box<dyn choices::Choice>> {
	match card.card_type() {
		specification::CardType::Blank => None,
		specification::CardType::Item(_) => None,
		specification::CardType::Challenge => unreachable!(),
		specification::CardType::Modifier => unreachable!(),
		specification::CardType::Monster => unreachable!(),
		specification::CardType::Magic => unreachable!(),
		specification::CardType::PartyLeader(_) => None, // TODO: Some hero leaders provide action points
		specification::CardType::Hero(_) => Some(Box::new(create_use_ability_choice(
			context, game, locator, card,
		))),
	}
}

pub fn assign_action_choices(context: &mut game_context::GameBookKeeping, game: &mut state::Game) {
	// let player_index = game.active_player_index();
	let player_index = game.current_player().player_index;
	let remaining_action_points = game.current_player().remaining_action_points;
	let mut options: Vec<Box<dyn choices::Choice>> = Vec::new();
	let default_choice = context.id_generator.generate();
	options.push(Box::new(create_forfeit_choice(
		context,
		game,
		choices::ChoiceLocator {
			id: default_choice,
			player_index,
		},
	)));
	options.push(Box::new(DrawChoice::new(context.locator(player_index))));
	if remaining_action_points >= 3 {
		options.push(Box::new(ReplaceHandChoice::new(
			context.locator(player_index),
		)));
	}
	if remaining_action_points >= 2 {
		for monster_card in game.monsters.stacks.iter() {
			let locator = context.locator(player_index);
			options.push(Box::new(create_attack_monster_choice(
				context,
				game,
				locator,
				&monster_card.top,
			)))
		}
	}

	for stack in game.current_player().hand.stacks.iter() {
		let locator = context.locator(player_index);
		if let Some(hand_choice) = create_hand_action_choice(context, game, locator, &stack.top) {
			options.push(hand_choice);
		}
	}
	for stack in game.current_player().party.stacks.iter() {
		let locator = context.locator(player_index);
		if let Some(hand_choice) = create_party_action_choice(context, game, locator, &stack.top) {
			options.push(hand_choice);
		}
	}
	{
		let locator = context.locator(player_index);
		if let Some(hand_choice) =
			create_party_action_choice(context, game, locator, &game.current_player().leader)
		{
			options.push(hand_choice);
		}
	}

	log::info!(
		"Assigning {} actions to player at index {}",
		options.len(),
		player_index
	);

	game.current_player_mut().choices = Some(choices::Choices {
		instructions: "Please choose an action".to_string(),
		options,
		default_choice,
		timeline: deadlines::get_action_point_choice_deadline(),
	});
}

//////////////////////////////////////////////////////////////////////
/// Tasks
//////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct UseAbility {
	// player_index: usize,
	// amount: u32,
}

impl UseAbility {
	pub fn new() -> Self {
		Self {}
	}
}
impl PlayerTask for UseAbility {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, _game: &mut Game,
	) -> SlayResult<TaskProgressResult> {
		todo!();
	}
}

#[derive(Clone, Debug)]
pub struct RemoveActionPointsTask {
	player_index: usize,
	amount: u32,
}

impl RemoveActionPointsTask {
	pub fn new(player_index: usize, amount: u32) -> Self {
		Self {
			player_index,
			amount,
		}
	}
}
impl PlayerTask for RemoveActionPointsTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game,
	) -> SlayResult<TaskProgressResult> {
		game.players[self.player_index].remaining_action_points -= self.amount;
		Ok(TaskProgressResult::TaskComplete)
	}
}

#[derive(Clone, Debug)]
pub struct OfferChallengesTask {
	offer: Option<OfferChallengesState>,
}

impl OfferChallengesTask {
	pub fn new(offer: OfferChallengesState) -> Self {
		Self { offer: Some(offer) }
	}
}
impl PlayerTask for OfferChallengesTask {
	fn make_progress(
		&mut self, context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<tasks::TaskProgressResult> {
		if let Some(offer) = self.offer.take() {
			offer.assign_all_choices(context, game);
			game.showdown.offer(offer);
			Ok(tasks::TaskProgressResult::TaskComplete)
		} else {
			Err(SlayError::new("Can only perform a choice once..."))
		}
	}
}

#[derive(Clone, Debug)]
pub struct DoRollTask {
	roll: Option<RollState>,
}

impl DoRollTask {
	pub fn new(roll: RollState) -> Self {
		Self { roll: Some(roll) }
	}
}
impl PlayerTask for DoRollTask {
	fn make_progress(
		&mut self, context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<tasks::TaskProgressResult> {
		if let Some(mut roll) = self.roll.take() {
			roll.completion_tracker.reset_timeline();
			roll.assign_all_choices(context, game);
			game.showdown.roll(roll);
			Ok(tasks::TaskProgressResult::TaskComplete)
		} else {
			Err(SlayError::new("Can only perform a choice once..."))
		}
	}
}

#[derive(Clone, Debug)]
pub struct CardUsedTask {
	player_index: usize,
	card_id: ids::CardId,
}

impl CardUsedTask {
	pub fn new(player_index: usize, card_id: ids::CardId) -> Self {
		Self {
			player_index,
			card_id,
		}
	}
}
impl PlayerTask for CardUsedTask {
	fn make_progress(
		&mut self, _context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<tasks::TaskProgressResult> {
		game.players[self.player_index]
			.party
			.card_mut(self.card_id)
			.ok_or_else(|| SlayError::new("Card no longer in party!"))?
			.played_this_turn = true;
		Ok(tasks::TaskProgressResult::TaskComplete)
	}
}
