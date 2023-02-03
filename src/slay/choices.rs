use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;

use crate::slay::deadlines::Timeline;
use crate::slay::ids;
use crate::slay::showdown::completion::Completion;
use crate::slay::showdown::roll_modification::ModificationPath;
use crate::slay::showdown::roll_modification::RollModificationChoiceType;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::tasks::move_card::MoveCardTask;

use super::specification::HeroType;
use super::specs::hero::HeroAbilityType;
use super::specs::items::AnotherItemType;
use super::specs::magic::MagicSpell;
use super::specs::modifier::ModifierKinds;
use super::specs::monster::Monster;
use super::state::game::GameStaticInformation;
use super::tasks::task_params::TaskParamName;
use super::tasks::tasks::search_discard::SearchDiscardFilters;

#[derive(Debug, Clone, PartialEq)]
pub enum ChoicesType {
	ChooseCardToGive(ids::PlayerIndex),
	SpendActionPoints,
	SearchDiscard(SearchDiscardFilters),
	ChoosePlayerParam(TaskParamName),
	ChooseCardParam(TaskParamName),
	OfferChallenges,
	PlayImmediately(SlayCardSpec), // Does it really need an argument?
	PlayOneOfImmediately,
	ReturnAnItemCard,
	Discard,
	Sacrifice,
	ModifyChallenge, /*(ChallengeReason)*/
	// Do we need the player?
	ModifyRoll, // different from modify challenge?
	ContinueDiscardingAndDestroying(u32),
	RevealAndDestroy,
	PlaceAHeroCard,

	BullseyeKeep,
	BullseyeOrdering(SlayCardSpec, SlayCardSpec),
}

#[derive(Clone, Debug)]
pub struct Choices {
	pub choices_type: ChoicesType,
	pub options: Vec<TasksChoice>,
	pub default_choice: Option<ids::ChoiceId>,
	pub timeline: Timeline,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChoicesPerspective {
	pub choices_type: ChoicesType,
	pub timeline: Timeline,
	pub options: Vec<ChoicePerspective>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Action {
	Forfeit,
	PlaceHeroInParty(HeroAbilityType),
	CastMagic(MagicSpell),
	PlaceItem(AnotherItemType),
	Draw,
	ReplaceHand,
	AttackMonster(Monster),
	UseLeader(HeroType),
	RollForAbility(HeroAbilityType),
}

// TODO: switch to these:
// #[derive(Clone, PartialEq, Debug)]
// pub enum PlayerParameter {}

// #[derive(Clone, PartialEq, Debug)]
// pub enum CardParameter {}

#[derive(Clone, PartialEq, Debug)]
pub enum Choice {
	UseActionPoints(Action),
	SetCompletion(Completion),
	Modify(ModificationPath, ModifierKinds, i32),
	Challenge,
	ChooseDiscardedCard(SlayCardSpec),
	ReturnItem(AnotherItemType, HeroAbilityType),

	SetPlayerParam(TaskParamName, ids::PlayerIndex),
	SetCardParameter(TaskParamName, SlayCardSpec),

	PlayImmediately(SlayCardSpec),
	DoNotPlayImmediately,
	Discard(SlayCardSpec),
	Sacrifice(HeroAbilityType),
	// SetParameter
	ChooseCardToGive(SlayCardSpec, ids::PlayerIndex),
	QuitAction,
	ContinueDiscardingAndDestroying,
	RevealChallengeAndDestroy,
	PlaceHeroImmediately(HeroAbilityType),
	BullseyeKeep(SlayCardSpec),
	BullseyeReorder,
	BullseyeDoNotReorder,
}

// TODO: Rename this to Choice
#[derive(Debug, Clone)]
pub struct TasksChoice {
	pub id: ids::ChoiceId,
	pub choice: Choice,
	pub display: ChoiceDisplayType,
	tasks: Vec<Box<dyn PlayerTask>>,
	prepend: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChoicePerspective {
	pub is_default: bool,
	pub choice_id: ids::ChoiceId,
	pub choice: Choice,
	pub display: ChoiceDisplayType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceDisplay {
	// pub arrows: Vec<DisplayArrow>,
	pub display_type: ChoiceDisplayType,
	// for i18n, this should still be an enum
	// pub label: String,

	// pub highlight: Option<DisplayPath>,
	// pub references_id: Option<ids::ElementId>,

	// TODO: Replace the following string with some useful enum...
	// pub label: String,
	// TODO: get rid of this...
	// pub roll_modification_choice: Option<RollModificationChoice>,
}

// impl ChoicesType {
// 	pub fn label(&self) -> String {
// 		match self {
//     	ChoicesType::SpendActionPoints => String::from(
// 				"How would you like to use your action points?"
// 			),
// 		}
// 	}
// }

impl Choice {
	pub fn label(&self) -> String {
		match self {
			Choice::UseActionPoints(action) => match action {
				Action::Forfeit => String::from("Do nothing this round."),
				Action::PlaceHeroInParty(hero_card) => format!("Place {} in your party", hero_card.label()),
				Action::CastMagic(magic_card) => format!("Play {}", magic_card.label()),
				Action::PlaceItem(item_card) => format!("Place {} on some hero card.", item_card.label()),
				Action::Draw => String::from("Draw a card."),
				Action::ReplaceHand => String::from("Use 3 action points to replace your entire hand."),
				Action::AttackMonster(monster) => format!("Attack {}", monster.label()),
				Action::UseLeader(_) => String::from("Use Shadow Claw to pull from another player's hand."),
				Action::RollForAbility(hero_card) => format!("Roll for {}", hero_card.label()),
			},
			Choice::SetCompletion(completion) => match completion {
				Completion::Thinking => todo!(),
				Completion::DoneUntilModification => {
					String::from("Do not modify this roll, unless someone else does.")
				}
				Completion::AllDone => String::from("Do not modify this roll, even if someone else does."),
			},
			Choice::Modify(path, kind, amount) => {
				format!("Use {:?} to modify {:?} by {}", kind, path, amount,)
			}
			Choice::Challenge => String::from("Challenge!"),
			Choice::SetPlayerParam(parameter, player_index) => {
				format!("Set {:?} to player {}", parameter, player_index + 1)
			}
			Choice::SetCardParameter(parameter, card) => {
				format!("Set {:?} to {}.", parameter, card.label())
			}
			Choice::ChooseDiscardedCard(spec) => spec.label().to_owned(),
			Choice::ReturnItem(item, hero) => format!("Return {} from {}", item.label(), hero.label(),),
			Choice::PlayImmediately(card) => format!("Play {} immediately", card.label(),),
			Choice::DoNotPlayImmediately => String::from("Do not play immediately"),
			Choice::Discard(card) => format!("Sacrifice {}", card.label(),),
			Choice::Sacrifice(hero_card) => format!("Sacrifice {}", hero_card.label(),),
			Choice::ChooseCardToGive(card, player_index) => {
				format!("Give {} to player {}.", card.label(), player_index,)
			}
			Choice::QuitAction => String::from("No"),
			Choice::ContinueDiscardingAndDestroying => format!("Discard and destroy again"),
			Choice::RevealChallengeAndDestroy => format!(
				// Are we sure it was pulled?
				"Reveal that you pulled a challenge card, so you can destroy a hero card.",
			),
			Choice::PlaceHeroImmediately(hero_card) => {
				format!("Place {} in you party immediately.", hero_card.label(),)
			}
			Choice::BullseyeKeep(spec) => format!("Place {} in your hand.", spec.label(),),
			Choice::BullseyeReorder => String::from("Change the order."),
			Choice::BullseyeDoNotReorder => String::from("Keep the current order."),
		}
	}
	pub fn get_notification(
		&self,
		game: &GameStaticInformation,
		player_index: ids::PlayerIndex,
	) -> String {
		// Player {} chose to
		match self {
			Choice::UseActionPoints(action) => match action {
				Action::Forfeit => format!("To do nothing, lol."),
				Action::PlaceHeroInParty(hero_card) => format!(
					"Player {} chose to place {} in their party.",
					game.player_name(player_index),
					hero_card.label(),
				),
				Action::CastMagic(magic_card) => format!(
					"{} chose to use the magic card {:?}",
					game.player_name(player_index),
					magic_card,
				),
				Action::PlaceItem(item) => format!(
					"{} chose to place the item {:?}",
					game.player_name(player_index),
					item,
				),
				Action::Draw => format!("{} chose to draw a card", game.player_name(player_index),),
				Action::ReplaceHand => format!(
					"{} chose to replace their hand with 5 new cards",
					game.player_name(player_index),
				),
				Action::AttackMonster(monster) => format!(
					"{} chose to attack {}",
					game.player_name(player_index),
					monster.label(),
				),
				Action::UseLeader(_) => format!(
					"{} chose to user their thiefy party leader's ability (and pull a card from somebody's hand).",
					game.player_name(player_index),
				),
				Action::RollForAbility(hero_card) => format!(
					"{} chose to roll for {}'s ability.",
					game.player_name(player_index),
					hero_card.label(),
				),
			},
			Choice::SetCompletion(persist) => format!(
				"{}'s completion is {:?}.",
				game.player_name(player_index),
				persist,
			),
			Choice::Modify(_, _, _) => format!(
				"{} chose to modify something by something.",
				game.player_name(player_index),
			),
			Choice::Challenge => format!("{} chose to challenge!", game.player_name(player_index),),
			Choice::SetPlayerParam(_, _) => {
				format!("{} chose a player.", game.player_name(player_index),)
			}
			Choice::SetCardParameter(_, _) => format!("{} chose a card", game.player_name(player_index),),
			Choice::ChooseDiscardedCard(_) => format!(
				"{} chose a card from the discard pile",
				game.player_name(player_index),
			),
			Choice::ReturnItem(_, _) => format!(
				"{} chose to return an item card.",
				game.player_name(player_index),
			),
			Choice::PlayImmediately(card) => format!(
				"{} decided whether to play {} immediately.",
				game.player_name(player_index),
				card.label(),
			),
			Choice::DoNotPlayImmediately => format!(
				"{} does not want to play immediately",
				game.player_name(player_index),
			),
			Choice::Discard(_) => format!(
				"{} chose to discard a certain card.",
				game.player_name(player_index),
			),
			Choice::Sacrifice(_) => format!(
				"{} chose to sacrifice a certain something.",
				game.player_name(player_index),
			),
			Choice::ChooseCardToGive(_, recipient) => format!(
				"{} gave {} a secret card",
				game.player_name(player_index),
				game.player_name(*recipient),
			),
			Choice::QuitAction => format!("{} chose to stop.", game.player_name(player_index),),
			Choice::ContinueDiscardingAndDestroying => format!(
				"{} is going to continue discarding and destroying.",
				game.player_name(player_index),
			),
			Choice::RevealChallengeAndDestroy => format!(
				"{} drew a challenge card, and will now destroy a hero card.",
				game.player_name(player_index)
			),
			Choice::PlaceHeroImmediately(hero_card) => format!(
				"{} placed {} in their party immediately",
				game.player_name(player_index),
				hero_card.label(),
			),
			Choice::BullseyeKeep(_) => {
				format!("{} kept one of the cards.", game.player_name(player_index),)
			}
			Choice::BullseyeReorder => format!(
				"{} changed the order of the next two cards.",
				game.player_name(player_index),
			),
			Choice::BullseyeDoNotReorder => format!(
				"{} did not change the order of the next two cards.",
				game.player_name(player_index),
			),
		}
	}
}

/*

*/

impl ChoicesType {
	pub fn get_instructions(&self) -> String {
		match self {
			ChoicesType::SpendActionPoints => {
				String::from("How would you like to use your action points?")
			}
			ChoicesType::SearchDiscard(filters) => {
				format!("Search the discard pile for {}.", filters.description())
			}
			ChoicesType::ModifyChallenge => String::from("Choose whether to modify the challenge."),
			ChoicesType::OfferChallenges => String::from("Choose whether to challenge."),
			ChoicesType::PlayImmediately(card) => format!(
				"You have received {}, would you like to play it immediately?",
				card.label(),
			),
			ChoicesType::ModifyRoll => String::from("Choose whether to modify the current roll."),
			ChoicesType::Discard => String::from("Choose a card in your hand to discard."),
			ChoicesType::ReturnAnItemCard => {
				String::from("Return an item card to someone's (TODO) hand.")
			}
			ChoicesType::ChoosePlayerParam(_) => String::from("Choose a player"),
			ChoicesType::ChooseCardParam(_) => String::from("Choose a card."),
			ChoicesType::Sacrifice => String::from("Choose a hero card to sacrifice."),
			ChoicesType::ChooseCardToGive(recipient) => {
				format!("Pick a card to give to player {}", recipient)
			}
			ChoicesType::ContinueDiscardingAndDestroying(num_remaining) => format!(
				"Would you like to discard a card so that you can destroy a hero card? (#{})",
				num_remaining,
			),
			ChoicesType::RevealAndDestroy => String::from(
				"Would you like to reveal and destroy?", // TODO: reveal what? (the card you drew? that it was a hero?)
			),
			ChoicesType::PlaceAHeroCard => {
				String::from("whice hero would you like to place in your party?")
			}
			ChoicesType::PlayOneOfImmediately => String::from("Which card would you like to play?"),
			ChoicesType::BullseyeKeep => String::from("Which card would you like to keep?"),
			ChoicesType::BullseyeOrdering(first, second) => format!(
				"The next two cards are {} and then {}. Would you like to swap the order?",
				first.label(),
				second.label(),
			),
		}
	}
}

impl Choices {
	pub fn new(
		options: Vec<TasksChoice>,
		default_choice: Option<ids::ChoiceId>,
		timeline: Timeline,
		choices_type: ChoicesType,
		// instructions: String,
	) -> Self {
		Self {
			options,
			default_choice,
			timeline,
			choices_type,
			// instructions,
		}
	}

	pub fn choice_perspetives(&self) -> Vec<ChoicePerspective> {
		self
			.options
			.iter()
			.map(|choice| choice.to_perspective(self.default_choice.iter().any(|dc| *dc == choice.id)))
			.collect()
	}

	pub fn to_perspective(&self) -> ChoicesPerspective {
		ChoicesPerspective {
			timeline: self.timeline.to_owned(),
			choices_type: self.choices_type.to_owned(),
			options: self.choice_perspetives(),
		}
	}
}

impl Summarizable for Choices {
	fn summarize<W: Write>(
		&self,
		f: &mut BufWriter<W>,
		indentation_level: u32,
	) -> Result<(), std::io::Error> {
		for _ in 0..indentation_level {
			write!(f, "  ")?;
		}
		write!(f, "choices: ({}): ", self.choices_type.get_instructions())?;
		for option in self.options.iter() {
			write!(f, "'{}', ", option.choice.label())?;
		}
		writeln!(f)?;
		Ok(())
	}
}

// TODO: move this
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum CardPath {
	TopCardIn(DeckPath, ids::CardId),
	ModifyingCardIn(DeckPath, ids::CardId, ids::CardId),
	Leader(ids::PlayerIndex, ids::CardId),
}

impl CardPath {
	pub fn display(&self) -> DisplayPath {
		DisplayPath::CardAt(*self)
	}

	pub fn get_deck_path(&self) -> DeckPath {
		match self {
			CardPath::TopCardIn(dp, _) => *dp,
			CardPath::ModifyingCardIn(dp, _, _) => *dp,
			CardPath::Leader(_, _) => unreachable!(),
		}
	}

	pub fn get_card_id(&self) -> ids::CardId {
		match self {
			CardPath::TopCardIn(_, card_id) => *card_id,
			CardPath::ModifyingCardIn(_, _, card_id) => *card_id,
			CardPath::Leader(_, card_id) => *card_id,
		}
	}

	pub fn get_place_task(&self) -> Box<dyn PlayerTask> {
		self.get_move_task(DeckPath::Party(self.get_player_index().unwrap()))
	}

	pub fn get_player_index(&self) -> Option<ids::PlayerIndex> {
		self.get_deck_path().get_player_index()
	}

	pub fn get_move_task(&self, destination: DeckPath) -> Box<dyn PlayerTask> {
		match self {
			CardPath::TopCardIn(deck_path, card_id) => Box::new(MoveCardTask {
				source: *deck_path,
				destination,
				card_id: *card_id,
			}),
			CardPath::ModifyingCardIn(deck_path, _, card_id) => Box::new(MoveCardTask {
				source: *deck_path,
				destination,
				card_id: *card_id,
			}),
			CardPath::Leader(_, _) => unreachable!(),
		}
	}

	pub fn get_discard_task(&self) -> Box<dyn PlayerTask> {
		self.get_move_task(DeckPath::Discard)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum DisplayPath {
	DeckAt(DeckPath),
	CardAt(CardPath),
	Player(ids::PlayerIndex),
	Roll(ModificationPath),
}

impl DisplayPath {
	pub fn to_highlight(&self) -> ChoiceDisplayType {
		ChoiceDisplayType::HighlightPath(*self)
	}
}

// #[derive(Debug, Clone)]
// pub struct DisplayArrow {
// 	pub source: DisplayPath,
// 	pub destination: DisplayPath,
// }

// impl Default for ChoiceDisplay {
//     fn default() -> Self {
//         Self {
// 					highlight: Default::default(),
// 					arrows: Default::default(),
// 					display_type: ChoiceDisplayType::Text("Please fill in the text for this choice"),
// 					label: Default::default(),
// 					roll_modification_choice: Default::default()
// 				}
//     }
// }

// #[derive(Debug, Clone)]
// pub struct ChoiceLocator {
// 	pub id: ids::ChoiceId,
// 	pub player_index: ids::PlayerIndex,
// }

// #[derive(Debug, Clone)]
// pub struct ChoiceInformation {
// 	pub locator: ChoiceLocator,
// 	pub display: ChoiceDisplay,
// }

// impl ChoiceInformation {
// 	pub fn new(locator: ChoiceLocator, display: ChoiceDisplay) -> Self {
// 		Self { locator, display }
// 	}

// 	pub fn get_id(&self) -> ids::ChoiceId {
// 		self.locator.id
// 	}
// 	pub fn player_index(&self) -> usize {
// 		self.locator.player_index
// 	}
// }

// dyn_clone::clone_trait_object!(Choice);

// pub trait Choice: Debug + dyn_clone::DynClone {
// 	fn select(
// 		&mut self, context: &mut GameBookKeeping, game: &mut Game,
// 	) -> SlayResult<()>;

// 	fn get_choice_information(&self) -> &ChoiceInformation;
// }

// impl<'de> Deserialize<'de> for Box<dyn Choice> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de> {
//         todo!()
//     }
// }

impl TasksChoice {
	pub fn new(
		id: ids::ChoiceId,
		choice: Choice,
		display: ChoiceDisplayType,
		tasks: Vec<Box<dyn PlayerTask>>,
	) -> Self {
		Self {
			id,
			choice,
			display,
			tasks,
			prepend: false,
		}
	}
	pub fn prepend(
		id: ids::ChoiceId,
		choice: Choice,
		display: ChoiceDisplayType,
		tasks: Vec<Box<dyn PlayerTask>>,
	) -> Self {
		Self {
			id,
			choice,
			display,
			tasks,
			prepend: true,
		}
	}

	pub fn select(
		&mut self,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> super::errors::SlayResult<()> {
		if self.prepend {
			game.players[player_index]
				.tasks
				.prepend_from(&mut self.tasks);
		} else {
			game.players[player_index].tasks.take_from(&mut self.tasks);
		}
		Ok(())
	}

	pub fn to_perspective(&self, is_default: bool) -> ChoicePerspective {
		ChoicePerspective {
			is_default,
			choice_id: self.id,
			choice: self.choice.to_owned(),
			display: self.display.to_owned(),
		}
	}
}

// impl Choice for TasksChoice {
// }

// #[derive(Debug, PartialEq, Clone)]
// pub enum ChoiceAssociationType {
// 	Representer,
// 	Source,
// 	Destination,
// }

// #[derive(Debug, PartialEq, Clone)]
// pub struct ChoiceAssociation {
// 	pub choice_id: ids::ChoiceId,
// 	pub association_type: ChoiceAssociationType,
// 	pub label: String,
// 	pub is_default: bool,
// }

impl ChoicesPerspective {
	pub fn represented_by(&self, display_path: &DisplayPath) -> Vec<ChoicePerspective> {
		self
			.options
			.iter()
			.filter(|choice| choice.display.represents(display_path))
			.map(|choice| choice.to_owned())
			.collect()
	}
	pub fn represents_card(&self, card_id: ids::CardId) -> Vec<ChoicePerspective> {
		self
			.options
			.iter()
			.filter(|choice| choice.display.represents_card(card_id))
			.map(|choice| choice.to_owned())
			.collect()
	}
}

impl ChoicePerspective {
	// fn new(
	// 	choices: &ChoicesPerspective,
	// 	choice: &ChoicePerspective,
	// 	default_choice: &Option<ids::ChoiceId>,
	// ) -> Self {
	// 	Self {
	// 		choice_id: choice.choice_id,
	// 		label: choice.display.label.to_owned(),
	// 		is_default: choices.default_choice.iter().any(|id| *id == choice.id),
	// 	}
	// }

	// fn create_from_choice(choices: &ChoicesPerspective, choice: &ChoicePerspective, path: DisplayPath) -> Vec<Self> {
	// 	let mut ret = Vec::new();
	// 	if let ChoiceDisplayType::HighlightPath(display_path) = choice.display.display_type {
	// 		if path == display_path {
	// 			ret.push(ChoiceAssociation::new(
	// 				choices,
	// 				choice,
	// 				ChoiceAssociationType::Representer,
	// 			));
	// 		}
	// 	}
	//  lol
	// // Not even sure if this will be used...
	// let (already_source, already_destination) = (false, false);
	// for arrow in choice.display.arrows.iter() {
	// 	if arrow.source == path && !already_source {
	// 			ret.push(ChoiceAssociation::new(
	// 					choices, choice, ChoiceAssociationType::Source));
	// 	}
	// 	if arrow.destination == path && !already_destination {
	// 			ret.push(ChoiceAssociation::new(
	// 					choices, choice, ChoiceAssociationType::Destination));
	// 	}
	// }
	// ret
	// }

	// pub fn create_from_choices(choices: &Option<&ChoicesPerspective>, path: DisplayPath) -> Vec<Self> {
	// 	if let Some(choices) = choices {
	// 		choices
	// 			.options
	// 			.iter()
	// 			.flat_map(|choice| ChoiceAssociation::create_from_choice(
	// 				choices, choice, path))
	// 			.collect()
	// 	} else {
	// 		Vec::new()
	// 	}
	// }
}

// Defines how this choice should be viewed.
#[derive(Debug, PartialEq, Clone)]
pub enum ChoiceDisplayType {
	// TODO: rename this tp "represented with" ...
	HighlightPath(DisplayPath),
	Modify(RollModificationChoiceType),
	Challenge(SlayCardSpec),
	SetCompletion(Completion),
	Text(&'static str),
	Card_(SlayCardSpec),
	Yes,
	No,
	Forfeit,
}

impl ChoiceDisplayType {
	pub fn hand_card(player_index: ids::PlayerIndex, card_id: ids::CardId) -> ChoiceDisplayType {
		ChoiceDisplayType::HighlightPath(DisplayPath::CardAt(CardPath::TopCardIn(
			DeckPath::Hand(player_index),
			card_id,
		)))
	}

	pub fn represents(&self, display_path: &DisplayPath) -> bool {
		if let ChoiceDisplayType::HighlightPath(represents_path) = self {
			*display_path == *represents_path
		} else {
			false
		}
	}

	pub fn represents_card(&self, card_id: ids::CardId) -> bool {
		if let ChoiceDisplayType::HighlightPath(display_path) = self {
			match display_path {
				DisplayPath::CardAt(card_path) => match card_path {
					CardPath::TopCardIn(_, cid) => *cid == card_id,
					CardPath::ModifyingCardIn(_, _, cid) => *cid == card_id,
					CardPath::Leader(_, cid) => *cid == card_id,
				},
				_ => false,
			}
		} else {
			false
		}
	}

	fn belongs_to_particular_roll(&self, path: ModificationPath) -> bool {
		if let ChoiceDisplayType::Modify(modification_choice) = self {
			match modification_choice {
				RollModificationChoiceType::AddToRoll(_, _, path2) => path == *path2,
				RollModificationChoiceType::RemoveFromRoll(_, _, path2) => path == *path2,
			}
		} else {
			false
		}
	}

	fn belongs_to_all_showdowns(&self) -> bool {
		matches!(self, ChoiceDisplayType::SetCompletion(_))
	}

	// He he, I wonder how many ways I could say that.
	pub fn belongs_to_challenge_roll(&self, path: ModificationPath) -> bool {
		self.belongs_to_particular_roll(path)
	}
	pub fn belongs_to_challenge(&self) -> bool {
		self.belongs_to_all_showdowns()
	}
	pub fn belongs_to_roll(&self) -> bool {
		self.belongs_to_particular_roll(ModificationPath::Roll) || self.belongs_to_all_showdowns()
	}
	pub fn belongs_to_offer(&self) -> bool {
		self.belongs_to_all_showdowns()
	}
}
