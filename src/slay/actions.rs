use crate::slay::choices;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context;
use crate::slay::ids;
use crate::slay::showdown::completion::RollCompletion;
use crate::slay::showdown::consequences;
use crate::slay::specification;
use crate::slay::state;
use crate::slay::tasks;

use crate::slay::choices::ChoiceDisplay;
use crate::slay::errors::SlayError;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::showdown::common::ChallengeReason;
use crate::slay::showdown::common::Roll;
use crate::slay::showdown::completion::CompletionTracker;
use crate::slay::showdown::consequences::RollConsequenceRenameMe;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::showdown::offer::OfferChallengesState;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::showdown::roll_state::RollState;
use crate::slay::specification::CardType;
use crate::slay::state::Card;
use crate::slay::state::Game;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskProgressResult;

use super::choices::ChoiceInformation;
use super::choices::ChoiceLocator;
use super::choices::TasksChoice;

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
		RollReason::UseHeroAbility(card.spec.to_owned()),
	))
}

fn create_place_hero_choice(
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
		vec![
			Box::new(RemoveActionPointsTask::new(player_index, 1)),
			Box::new(OfferChallengesTask::new(OfferChallengesState::new(
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
			))) as Box<dyn PlayerTask>,
		],
	)
}

fn create_place_item_choice(locator: choices::ChoiceLocator, card: &state::Card) -> TasksChoice {
	let player_index = locator.player_index;
	TasksChoice::new(
		choices::ChoiceInformation::new(
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
		vec![
			Box::new(RemoveActionPointsTask::new(player_index, 1)),
			Box::new(PlaceItemTask {
				player_index,
				card_id: card.id,
			}),
		],
	)
}

#[derive(Debug, Clone)]
struct PlaceItemTask {
	player_index: usize,
	card_id: ids::CardId,
}

impl PlayerTask for PlaceItemTask {
	fn make_progress(
		&mut self, _context: &mut game_context::GameBookKeeping, _game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		log::info!("TODO: Implement placing an item card...");
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!(
			"Player {} places the item {}",
			self.player_index, self.card_id
		)
	}
}

fn create_cast_magic_choice(locator: choices::ChoiceLocator, card: &state::Card) -> TasksChoice {
	let player_index = locator.player_index;
	TasksChoice::new(
		choices::ChoiceInformation::new(
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
		vec![
			Box::new(RemoveActionPointsTask::new(player_index, 1)),
			Box::new(CastMagicTask {
				player_index,
				card_id: card.id,
			}),
		],
	)
}

#[derive(Debug, Clone)]
struct CastMagicTask {
	player_index: usize,
	card_id: ids::CardId,
}

impl PlayerTask for CastMagicTask {
	fn make_progress(
		&mut self, _context: &mut game_context::GameBookKeeping, _game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		log::info!("TODO: Implement casting a magic card...");
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!(
			"Player {} places the item {}",
			self.player_index, self.card_id
		)
	}
}

// TODO: Refactor this to use TasksChoice...
#[derive(Clone, Debug)]
struct CastMagic {
	choice_information: choices::ChoiceInformation,
	card_id: ids::CardId,
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

fn create_draw_choice(locator: ChoiceLocator) -> TasksChoice {
	let player_index = locator.player_index;
	TasksChoice::new(
		choices::ChoiceInformation::new(
			locator,
			choices::ChoiceDisplay {
				highlight: Some(choices::DisplayPath::DeckAt(state::DeckPath::Draw)),
				arrows: vec![choices::DisplayArrow {
					source: choices::DisplayPath::DeckAt(state::DeckPath::Draw),
					destination: choices::DisplayPath::DeckAt(state::DeckPath::Hand(player_index)),
				}],
				label: format!("Draw a card."),
				roll_modification_choice: None,
			},
		),
		vec![
			Box::new(RemoveActionPointsTask::new(player_index, 1)),
			Box::new(DrawTask {
				player_index,
				number_to_draw: 1,
			}),
		],
	)
}

#[derive(Debug, Clone)]
struct DrawTask {
	player_index: usize,
	number_to_draw: u32,
}

impl PlayerTask for DrawTask {
	fn make_progress(
		&mut self, _context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		for _ in 0..self.number_to_draw {
			let stack = game.draw.deal();
			game.players[self.player_index].hand.stacks.push_back(stack);
		}
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("{} draws {} cards.", self.player_index, self.number_to_draw)
	}
}

fn create_replace_hand_choice(locator: ChoiceLocator) -> TasksChoice {
	let player_index = locator.player_index;
	TasksChoice::new(
		ChoiceInformation::new(
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
						destination: choices::DisplayPath::DeckAt(state::DeckPath::Hand(locator.player_index)),
					},
				],
				label: "Replace your hand with 5 new cards.".to_string(),
				roll_modification_choice: None,
			},
		),
		vec![
			Box::new(RemoveActionPointsTask::new(player_index, 3)),
			Box::new(ReplaceHandTask { player_index }),
		],
	)
}

#[derive(Debug, Clone)]
struct ReplaceHandTask {
	player_index: usize,
}

impl PlayerTask for ReplaceHandTask {
	fn make_progress(
		&mut self, _context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) -> SlayResult<TaskProgressResult> {
		let player = &mut game.players[self.player_index];
		player.remaining_action_points -= 3;
		game.discard.extend(player.hand.drain(..));
		player.hand.extend(game.draw.drain(0..5));
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!(
			"{} replaces their hand with 5 new cards.",
			self.player_index
		)
	}
}

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
	context: &mut GameBookKeeping, _game: &Game, locator: choices::ChoiceLocator,
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
			Box::new(RemoveActionPointsTask::new(player_index, 2)) as Box<dyn PlayerTask>,
			Box::new(DoRollTask::new(RollState::new(
				player_index,
				monster_card
					.spec
					.monster
					.as_ref()
					.unwrap()
					.create_consequences(player_index),
				Roll::create_from(&mut context.rng),
				RollReason::AttackMonster(monster_card.spec.to_owned()),
			))) as Box<dyn PlayerTask>,
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
) -> Option<TasksChoice> {
	match card.card_type() {
		CardType::Blank => None,
		CardType::Challenge => None,
		CardType::Modifier => None,
		CardType::PartyLeader(_) => unreachable!(),
		CardType::Monster => unreachable!(),
		CardType::Hero(_) => Some(create_place_hero_choice(context, game, locator, card)),
		CardType::Item(_) => Some(create_place_item_choice(locator, card)),
		CardType::Magic => Some(create_cast_magic_choice(locator, card)),
	}
}

fn create_party_action_choice(
	context: &mut GameBookKeeping, game: &Game, locator: choices::ChoiceLocator, card: &state::Card,
) -> Option<TasksChoice> {
	match card.card_type() {
		specification::CardType::Blank => None,
		specification::CardType::Item(_) => None,
		specification::CardType::Challenge => unreachable!(),
		specification::CardType::Modifier => unreachable!(),
		specification::CardType::Monster => unreachable!(),
		specification::CardType::Magic => unreachable!(),
		specification::CardType::PartyLeader(_) => None, // TODO: Some hero leaders provide action points
		specification::CardType::Hero(_) => {
			Some(create_use_ability_choice(context, game, locator, card))
		}
	}
}

pub fn assign_action_choices(context: &mut game_context::GameBookKeeping, game: &mut state::Game) {
	// let player_index = game.active_player_index();
	let player_index = game.current_player().player_index;
	let remaining_action_points = game.current_player().remaining_action_points;
	let mut options: Vec<TasksChoice> = Vec::new();
	let default_choice = context.id_generator.generate();
	options.push(create_forfeit_choice(
		context,
		game,
		choices::ChoiceLocator {
			id: default_choice,
			player_index,
		},
	));
	options.push(create_draw_choice(context.locator(player_index)));
	if remaining_action_points >= 3 {
		options.push(create_replace_hand_choice(context.locator(player_index)));
	}
	if remaining_action_points >= 2 {
		for monster_card in game.monsters.stacks.iter() {
			let locator = context.locator(player_index);
			options.push(create_attack_monster_choice(
				context,
				game,
				locator,
				&monster_card.top,
			))
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

	fn label(&self) -> String {
		format!("Use hero ability")
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
	fn label(&self) -> String {
		format!(
			"Deducting {} action points from {}.",
			self.amount, self.player_index
		)
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
		log::info!("making progress");
		if let Some(mut offer) = self.offer.take() {
			let mut completion_tracker = CompletionTracker::new(
				game.number_of_players(),
				deadlines::get_offer_challenges_deadline(),
			);
			// The current player is not allowed to challenge himself...
			completion_tracker.set_player_completion(offer.player_index, RollCompletion::AllDone);
			offer.completion_tracker = Some(completion_tracker);
			offer.assign_all_choices(context, game);
			game.showdown.offer(offer);
			log::info!("set the offer...");
			Ok(tasks::TaskProgressResult::TaskComplete)
		} else {
			Err(SlayError::new("Can only perform a choice once..."))
		}
	}
	fn label(&self) -> String {
		format!("Offering challenges for {:?}", self.offer)
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
			roll.completion_tracker = Some(CompletionTracker::new(
				game.number_of_players(),
				deadlines::get_roll_deadline(),
			));
			roll.assign_all_choices(context, game);
			game.showdown.roll(roll);
			Ok(tasks::TaskProgressResult::TaskComplete)
		} else {
			Err(SlayError::new("Can only perform a choice once..."))
		}
	}
	fn label(&self) -> String {
		format!("Doing a roll task for {:?}", self.roll)
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
	fn label(&self) -> String {
		format!(
			"Marking {} as used for player {}",
			self.card_id, self.player_index
		)
	}
}
