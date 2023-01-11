use crate::slay::abilities::magic::MagicTask;
use crate::slay::abilities::params::ChoosePlayerParameterTask;
use crate::slay::choices::CardPath;
use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::modifiers::ItemModifier;
use crate::slay::showdown::common::ChallengeReason;
use crate::slay::showdown::common::Roll;
use crate::slay::showdown::completion::Completion;
use crate::slay::showdown::completion::CompletionTracker;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::current_showdown::ShowDown;
use crate::slay::showdown::offer::OfferChallengesState;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::showdown::roll_state::RollState;
use crate::slay::specification::MonsterSpec;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::tasks;
use crate::slay::tasks::MoveCardTask;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::TaskParamName;
use crate::slay::tasks::TaskProgressResult;

use super::specs::hero::HeroAbility;
use super::specs::magic::MagicSpell;

// Emit logs like "Waiting for challenges..."

#[derive(Debug, Clone)]
pub struct AddTasks {
	tasks: Vec<Box<dyn PlayerTask>>,
}

impl PlayerTask for AddTasks {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
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
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card: &Card,
	ability: &HeroAbility,
) -> Box<dyn PlayerTask> {
	Box::new(AddTasks {
		tasks: vec![
			Box::new(CardUsedTask::new(player_index, card.id)),
			Box::new(DoRollTask::new(RollState::create(
				context,
				game,
				player_index,
				ability.to_consequences(),
				RollReason::UseHeroAbility(card.card_type),
			))) as Box<dyn PlayerTask>,
		],
	})
}

fn create_place_hero_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	ability: &HeroAbility,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		ChoiceDisplay {
			display_type: card_path.display().to_highlight(),
			label: format!(
				"Place {} in your party.",
				game.card(card_path).get_spec().label
			),
		},
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
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
								ability,
							),
						],
					},
					loss: Some(RollConsequence {
						condition: Condition::challenge_sustained(),
						tasks: vec![card_path.get_discard_task()],
					}),
				},
				ChallengeReason::PlaceHeroCard(game.card(card_path).card_type),
			))) as Box<dyn PlayerTask>,
		],
	)
}

pub fn create_place_item_challenge_offer(
	player_index: ids::PlayerIndex,
	card: &Card,
	_item_modifier: &ItemModifier,
	players_with_stacks: Vec<ids::PlayerIndex>,
) -> Box<dyn PlayerTask> {
	Box::new(OfferChallengesTask::new(OfferChallengesState::new(
		player_index,
		RollConsequences {
			success: RollConsequence {
				condition: Condition::challenge_denied(),
				tasks: vec![
					// TODO: place the item...
					ChoosePlayerParameterTask::one_of(
						TaskParamName::PlayerToGiveItem,
						"Choose a player to give this item to.",
						players_with_stacks,
					),
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
		ChallengeReason::PlaceHeroCard(card.card_type),
	))) as Box<dyn PlayerTask>
}

pub fn create_place_item_choice(
	placer_index: ids::PlayerIndex,
	id: ids::ChoiceId,
	card: &Card,
	display_type: ChoiceDisplayType,
	item_modifier: &ItemModifier,
	players_with_stacks: Vec<ids::PlayerIndex>,
) -> TasksChoice {
	TasksChoice::new(
		id,
		ChoiceDisplay {
			display_type,
			label: format!("Place item {}.", card.label()),
		},
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			create_place_item_challenge_offer(placer_index, card, item_modifier, players_with_stacks),
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
		ChoiceDisplay {
			display_type: card_path.display().to_highlight(),
			label: format!("Cast {}", game.card(card_path).get_spec().label),
		},
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			card_path.get_discard_task(),
			Box::new(OfferChallengesTask::new(OfferChallengesState::new(
				player_index,
				RollConsequences {
					success: RollConsequence {
						condition: Condition::challenge_denied(),
						tasks: vec![Box::new(MagicTask::new(spell)) as Box<dyn tasks::PlayerTask>],
					},
					loss: None,
				},
				ChallengeReason::CastMagic(game.card(card_path).card_type),
			))) as Box<dyn PlayerTask>,
		],
	)
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

fn create_draw_choice(id: ids::ChoiceId) -> TasksChoice {
	TasksChoice::new(
		id,
		ChoiceDisplay {
			display_type: ChoiceDisplayType::HighlightPath(DisplayPath::DeckAt(DeckPath::Draw)),
			label: "Draw a card.".to_string(),
		},
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
	pub fn create(num: usize) -> Box<dyn PlayerTask> {
		Box::new(Self::new(num))
	}
	pub fn new(number_to_draw: usize) -> Self {
		Self { number_to_draw }
	}
}

impl PlayerTask for DrawTask {
	fn make_progress(
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
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

fn create_replace_hand_choice(id: ids::ChoiceId) -> TasksChoice {
	TasksChoice::new(
		id,
		ChoiceDisplay {
			display_type: ChoiceDisplayType::HighlightPath(DisplayPath::DeckAt(DeckPath::Discard)),
			label: "Replace your hand with 5 new cards.".to_string(),
		},
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
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
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
	game: &mut Game,
	player_index: ids::PlayerIndex,
	id: ids::ChoiceId,
) -> TasksChoice {
	let current_amount_remaining = game.players[player_index].get_remaining_action_points();
	TasksChoice::new(
		id,
		ChoiceDisplay {
			display_type: ChoiceDisplayType::Forfeit,
			label: "Do nothing this turn".to_string(),
		},
		vec![Box::new(RemoveActionPointsTask::new(current_amount_remaining)) as Box<dyn PlayerTask>],
	)
}

fn create_roll_for_ability_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	ability: &HeroAbility,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		ChoiceDisplay {
			display_type: card_path.display().to_highlight(),
			label: format!("Use {}'s ability", game.card(card_path).get_spec().label),
		},
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			create_roll_for_ability_task(context, game, player_index, game.card(card_path), ability),
		],
	)
}

fn create_attack_monster_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	monster: &MonsterSpec,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		ChoiceDisplay {
			display_type: card_path.display().to_highlight(),
			label: format!("Attack {}", game.card(card_path).get_spec().label),
		},
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
	if let Some(ability) = game.card(card_path).get_spec().hero_ability.as_ref() {
		return Some(create_place_hero_choice(
			context,
			game,
			player_index,
			card_path,
			ability,
		));
	}
	if let Some(modifier) = game.card(card_path).get_spec().card_modifier.as_ref() {
		let players_with_stacks = game.players_with_stacks();
		if !players_with_stacks.is_empty() {
			return Some(create_place_item_choice(
				player_index,
				context.id_generator.generate(),
				game.card(card_path),
				card_path.display().to_highlight(),
				modifier,
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
	if let Some(ability) = game.card(card_path).get_spec().hero_ability.as_ref() {
		return Some(create_roll_for_ability_choice(
			context,
			game,
			player_index,
			card_path,
			ability,
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
			if let Some(monster) = monster_card.monster_spec() {
				options.push(create_attack_monster_choice(
					context,
					game,
					player_index,
					CardPath::TopCardIn(DeckPath::ActiveMonsters, monster_card.id),
					&monster,
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
		if let Some(hand_choice) =
			create_party_action_choice(context, game, player_index, CardPath::Leader(player_index))
		{
			options.push(hand_choice);
		}
	}

	log::info!(
		"Assigning {} actions to player at index {}",
		options.len(),
		player_index
	);

	game.current_player_mut().choices = Some(Choices {
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
		&mut self,
		_context: &mut GameBookKeeping,
		_game: &mut Game,
		_player_index: ids::PlayerIndex,
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
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
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
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		_player_index: ids::PlayerIndex,
	) -> SlayResult<tasks::TaskProgressResult> {
		log::info!("making progress");
		if let Some(mut offer) = self.offer.take() {
			let mut completion_tracker = CompletionTracker::new(
				game.number_of_players(),
				deadlines::get_offer_challenges_deadline(),
			);
			// The current player is not allowed to challenge himself...
			completion_tracker.set_player_completion(offer.player_index, Completion::AllDone);
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
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		_player_index: ids::PlayerIndex,
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
		&mut self,
		_context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<tasks::TaskProgressResult> {
		game.players[player_index].set_card_played(self.card_id);
		Ok(tasks::TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		format!("Marking {} as used", self.card_id)
	}
}
