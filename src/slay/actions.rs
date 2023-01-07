use crate::slay::choices;
use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::ChoiceInformation;
use crate::slay::choices::ChoiceLocator;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;

use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::showdown::common::ChallengeReason;
use crate::slay::showdown::common::Roll;
use crate::slay::showdown::completion::CompletionTracker;
use crate::slay::showdown::completion::RollCompletion;

use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::showdown::offer::OfferChallengesState;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::showdown::roll_state::RollState;
use crate::slay::specification;
use crate::slay::specification::CardType;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::stack::Card;
use crate::slay::tasks;
use crate::slay::tasks::MoveCardTask;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskProgressResult;

use super::choices::DisplayArrow;
use super::choices::DisplayPath;
use super::state::game::Game;
use super::state::stack::CardSpecPerspective;

// Emit logs like "Waiting for challenges..."

#[derive(Debug, Clone)]
pub struct AddTasks {
	tasks: Vec<Box<dyn PlayerTask>>,
}

impl PlayerTask for AddTasks {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[player_index]
			.tasks
			.prepend_from(&mut self.tasks);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Adding some tasks".to_owned()
	}
}

fn create_roll_for_ability_task(
	context: &mut GameBookKeeping, player_index: ids::PlayerIndex, card: &Card,
) -> DoRollTask {
	DoRollTask::new(RollState::new(
		player_index,
		card.hero_ability().as_ref().unwrap().to_consequences(),
		Roll::create_from(&mut context.rng),
		RollReason::UseHeroAbility(CardSpecPerspective::new(&card.spec)),
	))
}

fn create_place_hero_choice(
	context: &mut GameBookKeeping, game: &Game, locator: choices::ChoiceLocator, card: &Card,
) -> TasksChoice {
	let player_index = locator.player_index;
	TasksChoice::new(
		choices::ChoiceInformation::new(
			locator,
			ChoiceDisplay {
				highlight: Some(choices::DisplayPath::CardIn(
					DeckPath::Hand(player_index),
					card.id,
				)),
				arrows: vec![choices::DisplayArrow {
					source: choices::DisplayPath::CardIn(DeckPath::Hand(player_index), card.id),
					destination: choices::DisplayPath::DeckAt(DeckPath::Party(player_index)),
				}],
				label: format!("Place {} in your party.", card.label()),
				roll_modification_choice: None,
			},
		),
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			Box::new(OfferChallengesTask::new(OfferChallengesState::new(
				player_index,
				RollConsequences {
					success: RollConsequence {
						condition: Condition::challenge_denied(),
						tasks: vec![
							Box::new(MoveCardTask {
								source: DeckPath::Hand(player_index),
								destination: DeckPath::Party(player_index),
								card_id: card.id,
							}) as Box<dyn tasks::PlayerTask>,
							Box::new(create_roll_for_ability_task(context, player_index, card))
								as Box<dyn tasks::PlayerTask>,
						],
					},
					loss: Some(RollConsequence {
						condition: Condition::challenge_sustained(),
						tasks: vec![Box::new(MoveCardTask {
							source: DeckPath::Hand(player_index),
							destination: DeckPath::Discard,
							card_id: card.id,
						}) as Box<dyn tasks::PlayerTask>],
					}),
				},
				ChallengeReason::PlaceHeroCard(CardSpecPerspective::new(&card.spec)),
			))) as Box<dyn PlayerTask>,
		],
	)
}

fn create_place_item_choice(locator: choices::ChoiceLocator, card: &Card) -> TasksChoice {
	let _player_index = locator.player_index;
	TasksChoice::new(
		choices::ChoiceInformation::new(
			locator.to_owned(),
			choices::ChoiceDisplay {
				highlight: Some(choices::DisplayPath::CardIn(
					DeckPath::Hand(locator.player_index),
					card.id,
				)),
				// Could have it going to each other deck?
				// arrows: vec![],
				label: format!("Use the ability {}, discarding the card.", card.label()),
				..Default::default()
			},
		),
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			Box::new(PlaceItemTask { card_id: card.id }),
		],
	)
}

#[derive(Debug, Clone)]
struct PlaceItemTask {
	card_id: ids::CardId,
}

impl PlayerTask for PlaceItemTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, _game: &mut Game, _player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		log::info!("TODO: Implement placing an item card...");
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Places the item {}", self.card_id)
	}
}

fn create_cast_magic_choice(locator: choices::ChoiceLocator, card: &Card) -> TasksChoice {
	let _player_index = locator.player_index;
	TasksChoice::new(
		choices::ChoiceInformation::new(
			locator.to_owned(),
			choices::ChoiceDisplay {
				highlight: Some(choices::DisplayPath::CardIn(
					DeckPath::Hand(locator.player_index),
					card.id,
				)),
				arrows: vec![choices::DisplayArrow {
					source: choices::DisplayPath::CardIn(DeckPath::Hand(locator.player_index), card.id),
					destination: choices::DisplayPath::DeckAt(DeckPath::Discard),
				}],
				// Could show arrows to each of the possibilities
				label: format!("Cast {}.", card.label()),
				roll_modification_choice: None,
			},
		),
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			Box::new(CastMagicTask { card_id: card.id }),
		],
	)
}

#[derive(Debug, Clone)]
struct CastMagicTask {
	card_id: ids::CardId,
}

impl PlayerTask for CastMagicTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, _game: &mut Game, _player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		log::info!("TODO: Implement casting a magic card...");
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("cast some magic {}", self.card_id)
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
				highlight: Some(choices::DisplayPath::DeckAt(DeckPath::Draw)),
				arrows: vec![choices::DisplayArrow {
					source: choices::DisplayPath::DeckAt(DeckPath::Draw),
					destination: choices::DisplayPath::DeckAt(DeckPath::Hand(player_index)),
				}],
				label: "Draw a card.".to_string(),
				roll_modification_choice: None,
			},
		),
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			Box::new(DrawTask { number_to_draw: 1 }),
		],
	)
}

#[derive(Debug, Clone)]
pub struct DrawTask {
	number_to_draw: usize,
}

impl DrawTask {
	pub fn new(number_to_draw: usize) -> Self {
		Self { number_to_draw }
	}
}

impl PlayerTask for DrawTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.replentish_for(self.number_to_draw);
		game.players[player_index]
			.hand
			.extend(game.draw.drain(0..self.number_to_draw));
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Draw {} cards.", self.number_to_draw)
	}
}

fn create_replace_hand_choice(locator: ChoiceLocator) -> TasksChoice {
	let _player_index = locator.player_index;
	TasksChoice::new(
		ChoiceInformation::new(
			locator.to_owned(),
			choices::ChoiceDisplay {
				highlight: Some(choices::DisplayPath::DeckAt(DeckPath::Discard)),
				arrows: vec![
					choices::DisplayArrow {
						source: choices::DisplayPath::DeckAt(DeckPath::Hand(locator.player_index)),
						destination: choices::DisplayPath::DeckAt(DeckPath::Discard),
					},
					choices::DisplayArrow {
						source: choices::DisplayPath::DeckAt(DeckPath::Draw),
						destination: choices::DisplayPath::DeckAt(DeckPath::Hand(locator.player_index)),
					},
				],
				label: "Replace your hand with 5 new cards.".to_string(),
				roll_modification_choice: None,
			},
		),
		vec![
			Box::new(RemoveActionPointsTask::new(3)),
			Box::new(ReplaceHandTask {}),
		],
	)
}

#[derive(Debug, Clone)]
struct ReplaceHandTask {}

impl PlayerTask for ReplaceHandTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game
			.discard
			.extend(game.players[player_index].hand.drain(..));
		game.replentish_for(5);
		game.players[player_index]
			.hand
			.extend(game.draw.drain(0..5));
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Replace your hand with 5 new cards.".to_string()
	}
}

fn create_forfeit_choice(
	_context: &mut GameBookKeeping, game: &mut Game, locator: choices::ChoiceLocator,
) -> TasksChoice {
	let player_index = locator.player_index;
	let current_amount_remaining = game.players[player_index].get_remaining_action_points();
	TasksChoice::new(
		choices::ChoiceInformation::new(
			locator,
			choices::ChoiceDisplay {
				label: "Do nothing this turn".to_string(),
				..Default::default()
			},
		),
		vec![Box::new(RemoveActionPointsTask::new(current_amount_remaining)) as Box<dyn PlayerTask>],
	)
}

fn create_use_ability_task(
	_context: &mut GameBookKeeping, _game: &Game, _player_index: ids::PlayerIndex, _card: &Card,
) -> UseAbility {
	UseAbility::new()
}

fn create_roll_for_ability_choice(
	context: &mut GameBookKeeping, _game: &Game, locator: ChoiceLocator, card: &Card,
) -> TasksChoice {
	let player_index = locator.player_index;
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
			Box::new(RemoveActionPointsTask::new(1)),
			Box::new(create_roll_for_ability_task(context, player_index, card)) as Box<dyn PlayerTask>,
		],
	)
}

fn create_attack_monster_choice(
	context: &mut GameBookKeeping, _game: &Game, locator: ChoiceLocator, monster_card: &Card,
) -> TasksChoice {
	let player_index = locator.player_index;
	TasksChoice::new(
		ChoiceInformation::new(
			locator.to_owned(),
			ChoiceDisplay {
				highlight: Some(DisplayPath::DeckAt(DeckPath::Discard)),
				arrows: vec![DisplayArrow {
					source: DisplayPath::DeckAt(DeckPath::ActiveMonsters),
					destination: DisplayPath::DeckAt(DeckPath::SlainMonsters(locator.player_index)),
				}],
				label: format!("Attack {}", monster_card.label()),
				roll_modification_choice: None,
			},
		),
		vec![
			Box::new(RemoveActionPointsTask::new(2)) as Box<dyn PlayerTask>,
			Box::new(DoRollTask::new(RollState::new(
				player_index,
				monster_card
					.spec
					.monster
					.as_ref()
					.unwrap()
					.consequences
					.clone(),
				Roll::create_from(&mut context.rng),
				RollReason::AttackMonster(CardSpecPerspective::new(&monster_card.spec)),
			))) as Box<dyn PlayerTask>,
		],
	)
}

impl GameBookKeeping {
	pub fn locator(&mut self, player_index: ids::PlayerIndex) -> choices::ChoiceLocator {
		choices::ChoiceLocator {
			id: self.id_generator.generate(),
			player_index,
		}
	}
}

fn create_hand_action_choice(
	context: &mut GameBookKeeping, game: &Game, locator: choices::ChoiceLocator, card: &Card,
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
	context: &mut GameBookKeeping, game: &Game, locator: ChoiceLocator, card: &Card,
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
			Some(create_roll_for_ability_choice(context, game, locator, card))
		}
	}
}

pub fn assign_action_choices(context: &mut GameBookKeeping, game: &mut Game) {
	// let player_index = game.active_player_index();
	let player_index = game.current_player().player_index;
	let remaining_action_points = game.current_player().get_remaining_action_points();
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
		for monster_card in game.monsters.iter() {
			let locator = context.locator(player_index);
			options.push(create_attack_monster_choice(
				context,
				game,
				locator,
				&monster_card.top,
			))
		}
	}

	for stack in game.current_player().hand.iter() {
		let locator = context.locator(player_index);
		if let Some(hand_choice) = create_hand_action_choice(context, game, locator, &stack.top) {
			options.push(hand_choice);
		}
	}
	for stack in game.current_player().party.iter() {
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
		default_choice: Some(default_choice),
		timeline: deadlines::get_action_point_choice_deadline(),
	});
}

//////////////////////////////////////////////////////////////////////
/// Tasks
//////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct UseAbility {
	// player_index: ids::PlayerIndex,
	// amount: u32,
}

impl UseAbility {
	pub fn new() -> Self {
		Self {}
	}
}
impl PlayerTask for UseAbility {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, _game: &mut Game, _player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		log::info!("TODO: Implement using a hero ability.");
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Use hero ability".to_string()
	}
}

#[derive(Clone, Debug)]
pub struct RemoveActionPointsTask {
	amount: u32,
}

impl RemoveActionPointsTask {
	pub fn new(amount: u32) -> Self {
		Self { amount }
	}
}
impl PlayerTask for RemoveActionPointsTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[player_index].action_points_used(self.amount);
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!("Deducting {} action points.", self.amount)
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
		&mut self, context: &mut GameBookKeeping, game: &mut Game, _player_index: ids::PlayerIndex,
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
		&mut self, context: &mut GameBookKeeping, game: &mut Game, _player_index: ids::PlayerIndex,
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
	card_id: ids::CardId,
}

impl CardUsedTask {
	pub fn new(_player_index: ids::PlayerIndex, card_id: ids::CardId) -> Self {
		Self { card_id }
	}
}
impl PlayerTask for CardUsedTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<tasks::TaskProgressResult> {
		game.players[player_index].set_card_played(self.card_id);
		Ok(tasks::TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!("Marking {} as used", self.card_id)
	}
}
