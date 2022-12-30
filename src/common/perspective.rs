// lol, I can't say `use crate::ids;`

use chrono::{DateTime, Utc};
use log;

use crate::slay::choices::Choices;
use crate::slay::ids;
use crate::slay::showdown::common::{Roll, RollModification};
use crate::slay::showdown::completion::{CompletionTracker, RollCompletion};
use crate::slay::showdown::roll_state::{RollReason, RollState};
use crate::slay::specification::{CardSpec, CardType, HeroAbility, MonsterSpec, Visibility};
use crate::slay::state::{self, Game, Player, Stack};

use std::fmt::Debug;

use crate::slay::specification::Perspective;

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerCompletionPerspective {
	pub player_name: String,
	pub completion: RollCompletion,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModificationPerspective {
	pub modifyer_name: String,
	pub modifying_card_spec: CardSpecPerspective,
	pub modification_amount: i32,
}

impl RollModification {
	pub fn to_perspective(&self, game: &Game) -> ModificationPerspective {
		let modifying_card = game.card(self.card_id).unwrap();
		let modifying_card_spec = CardSpecPerspective::new(&modifying_card.spec);
		ModificationPerspective {
			modifyer_name: game.players[self.modifying_player_index].name.to_owned(),
			modifying_card_spec,
			modification_amount: self.modification_amount,
		}
	}
}

impl CompletionTracker {
	fn to_perspective(&self, game: &Game) -> Vec<PlayerCompletionPerspective> {
		self
			.player_completions
			.iter()
			.map(|(player_index, completion)| {
				(PlayerCompletionPerspective {
					player_name: game.players[*player_index].name.to_owned(),
					completion: *completion,
				})
			})
			.collect()
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum RollModificationChoiceType {
	AddToRoll(CardSpec, i32),
	RemoveFromRoll(CardSpec, i32),
	Nothing(bool),
}

impl RollModificationChoiceType {
	pub fn from_card(spec: &CardSpec, amount: i32) -> Self {
		if amount < 0 {
			RollModificationChoiceType::RemoveFromRoll(spec.to_owned(), amount)
		} else {
			RollModificationChoiceType::AddToRoll(spec.to_owned(), amount)
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct RollModificationChoice {
	pub choice_id: ids::ChoiceId,
	pub choice_type: RollModificationChoiceType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RollPerspective {
	id: ids::RollId,
	pub roller_name: String,
	pub initial: Roll,
	pub history: Vec<ModificationPerspective>,
	pub completions: Vec<PlayerCompletionPerspective>,
	pub roll_total: i32,
	pub success: bool,
	pub deadline: Option<DateTime<Utc>>,
	pub reason: RollReason,
	pub choices: Vec<RollModificationChoice>,
}

impl RollState {
	pub fn to_perspective(&self, game: &Game) -> RollPerspective {
		RollPerspective {
			id: 0, // Need to fill this in again?
			roller_name: game.players[self.roller_index].name.to_owned(),
			initial: self.initial.to_owned(),
			history: self
				.history
				.iter()
				.map(|m| m.to_perspective(game))
				.collect(),
			completions: self.completion_tracker.to_perspective(game),
			roll_total: self.calculate_roll_total(),
			success: false,
			deadline: self.completion_tracker.deadline,
			reason: self.reason.to_owned(),
			choices: todo!(),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum ChoiceAssociationType {
	Representer,
	Source,
	Destination,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChoiceAssociation {
	pub choice_id: ids::ChoiceId,
	pub association_type: ChoiceAssociationType,
	pub label: String,
	pub is_default: bool,
}

impl ChoiceAssociation {
	fn new(association_type: ChoiceAssociationType, choice_info: &ChoicePerspective) -> Self {
		log::info!("Found association");
		Self {
			choice_id: choice_info.choice_id,
			association_type,
			label: choice_info.label.to_owned(),
			is_default: choice_info.is_default,
		}
	}

	fn create_from_choice(choice_info: &ChoicePerspective, id: ids::ElementId) -> Vec<Self> {
		let mut ret = Vec::new();
		ret.extend(
			choice_info
				.highlight
				.iter()
				.filter(|highlight_id| **highlight_id == id)
				.map(|_| ChoiceAssociation::new(ChoiceAssociationType::Representer, choice_info)),
		);
		ret.extend(choice_info.arrows.iter().flat_map(|a| {
			let mut ret: Vec<ChoiceAssociation> = Vec::new();
			if let Some(source_id) = a.0 {
				if source_id == id {
					ret.push(ChoiceAssociation::new(
						ChoiceAssociationType::Source,
						choice_info,
					))
				}
			}
			if let Some(source_id) = a.1 {
				if source_id == id {
					ret.push(ChoiceAssociation::new(
						ChoiceAssociationType::Destination,
						choice_info,
					))
				}
			}
			ret
		}));
		ret
	}

	fn create_from_choices(choices: &Option<ChoicesPerspective>, id: ids::ElementId) -> Vec<Self> {
		if let Some(choices2) = choices {
			return choices2
				.actions
				.iter()
				.flat_map(|a| ChoiceAssociation::create_from_choice(a, id))
				.collect();
		}
		vec![]
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct CardSpecPerspective {
	pub card_type: CardType,
	pub label: String,
	pub description: String,
	pub image_path: String,
	pub monster: Option<MonsterSpec>,
	pub modifiers: Vec<i32>,
}

impl CardSpecPerspective {
	pub fn new(spec: &CardSpec) -> Self {
		Self {
			card_type: spec.card_type.to_owned(),
			label: spec.label.to_owned(),
			description: spec.description.to_owned(),
			image_path: spec.image_path.to_owned(),
			monster: spec.monster.to_owned(),
			modifiers: spec.modifiers.to_owned(),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct CardPerspective {
	pub id: ids::CardId,
	pub played_this_turn: bool,
	pub spec: CardSpecPerspective,
	pub choice_associations: Vec<ChoiceAssociation>,
}

impl state::Card {
	pub fn to_perspective(&self, choices: &Option<ChoicesPerspective>) -> CardPerspective {
		CardPerspective {
			id: self.id,
			played_this_turn: self.played_this_turn,
			spec: CardSpecPerspective::new(&self.spec),
			choice_associations: ChoiceAssociation::create_from_choices(choices, self.id),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct StackPerspective {
	pub top: CardPerspective,
	pub modifiers: Vec<CardPerspective>,
}

impl Stack {
	pub fn to_perspective(&self, choices: &Option<ChoicesPerspective>) -> StackPerspective {
		StackPerspective {
			top: self.top.to_perspective(choices),
			modifiers: self
				.modifiers
				.iter()
				.map(|s| s.to_perspective(choices))
				.collect(),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeckPerspective {
	pub id: ids::DeckId,
	pub label: String,
	pub count: usize,
	pub stacks: Option<Vec<StackPerspective>>,

	pub choice_associations: Vec<ChoiceAssociation>,
}

impl state::Deck {
	pub fn to_perspective(
		&self, perspective: &Perspective, choices: &Option<ChoicesPerspective>,
	) -> DeckPerspective {
		let visibility = self.spec.visibility.get(perspective);
		match visibility {
			Visibility::Visible => DeckPerspective {
				id: self.id,
				count: self.stacks.len(),
				label: self.spec.label.to_owned(),
				stacks: Some(
					self
						.stacks
						.iter()
						.map(|s| s.to_perspective(choices))
						.collect(),
				),
				choice_associations: ChoiceAssociation::create_from_choices(choices, self.id),
			},
			Visibility::Summary => DeckPerspective {
				id: self.id,
				count: self.stacks.len(),
				label: self.spec.label.to_owned(),
				stacks: None,
				choice_associations: ChoiceAssociation::create_from_choices(choices, self.id),
			},
			Visibility::Invisible => {
				unreachable!();
			}
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerPerspective {
	// List the hero types they have
	// Are they active player?
	// Is this you?
	// action points
	pub id: ids::PlayerId,
	pub name: String,
	pub me: bool,
	pub active: bool,
	// pub choices: Option<Choices>,
	pub remaining_action_points: u32,
	pub total_action_points: u32,

	pub leader: CardPerspective,
	pub decks: Vec<DeckPerspective>,

	pub choice_associations: Vec<ChoiceAssociation>,
}

impl state::Player {
	pub fn to_perspective(
		&self, perspective: &Perspective, choices: &Option<ChoicesPerspective>, active: bool,
	) -> PlayerPerspective {
		PlayerPerspective {
			id: self.id,
			name: self.name.to_owned(),
			remaining_action_points: self.remaining_action_points,
			leader: self.leader.to_perspective(choices),
			decks: self
				.decks()
				.iter()
				.filter(|d| d.is_visible(perspective))
				.map(|d| d.to_perspective(perspective, choices))
				.collect(),
			me: perspective == &Perspective::Owner,
			active,
			choice_associations: ChoiceAssociation::create_from_choices(choices, self.id),
			total_action_points: 3,
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChoicePerspective {
	pub is_default: bool,
	pub choice_id: ids::ChoiceId,
	pub label: String,
	pub highlight: Option<ids::ElementId>,
	// This will probably need an arrow id...
	pub arrows: Vec<(Option<ids::ElementId>, Option<ids::ElementId>)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChoicesPerspective {
	pub instructions: String,
	pub deadline: Option<DateTime<Utc>>,
	pub actions: Vec<ChoicePerspective>,
}

impl Choices {
	pub fn to_perspective(&self, game: &Game) -> ChoicesPerspective {
		ChoicesPerspective {
			deadline: self.deadline,
			instructions: self.instructions.to_owned(),
			actions: self
				.options
				.iter()
				.map(|o| {
					let info = o.get_choice_information();
					ChoicePerspective {
						is_default: info.get_id() == self.default_choice,
						choice_id: info.get_id(),
						label: info.display.label.to_owned(),
						highlight: None,
						arrows: vec![],
					}
				})
				.collect(),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct GamePerspective {
	pub players: Vec<PlayerPerspective>,
	// pub challenge: Option<ChallengeState>,
	// pub roll: Option<RollState>,
	pub decks: Vec<DeckPerspective>,
	pub choices: Option<ChoicesPerspective>,
	pub roll: Option<RollPerspective>,
	pub turn: state::Turn,
}

impl GamePerspective {
	pub fn rotated_players(&self) -> Vec<&PlayerPerspective> {
		let mut ret: Vec<&PlayerPerspective> = self.players.iter().collect();
		let position = ret.iter().position(|p| p.me);
		position.map(|index| ret.rotate_left(index));
		ret
	}
}

pub fn get_perspective(owner_id: ids::PlayerId, player_id: ids::PlayerId) -> &'static Perspective {
	if owner_id == player_id {
		&Perspective::Owner
	} else {
		&Perspective::Spectator
	}
}

impl state::Game {
	pub fn to_player_perspective(&self, player_id: ids::PlayerId) -> GamePerspective {
		let perspective = &Perspective::Spectator;
		let choices = self
			.players
			.iter()
			.find(|p| p.id == player_id)
			.map(|p| p.choices.as_ref())
			.flatten()
			.map(|c| c.to_perspective(&self));
		let players = self
			.players
			.iter()
			.map(|p| {
				p.to_perspective(
					get_perspective(p.id, player_id),
					&choices,
					p.player_index == self.active_player_index(),
				)
			})
			.collect();
		let decks = self
			.decks()
			.iter()
			.filter(|d| d.is_visible(perspective))
			.map(|d| d.to_perspective(perspective, &choices))
			.collect();
		GamePerspective {
			players,
			decks,
			turn: self.get_turn(),
			choices,
			roll: self.showdown.get_roll().map(|r| r.to_perspective(self)),
		}
	}
}
