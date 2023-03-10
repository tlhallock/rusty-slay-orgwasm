use crate::slay::choices::ChoicePerspective;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesPerspective;
use crate::slay::choices::DisplayPath;
use crate::slay::errors;
use crate::slay::ids;
use crate::slay::specification::HeroType;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::specs::visibility::Perspective;
use crate::slay::specs::visibility::VisibilitySpec;
use crate::slay::state::deck::Deck;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::deck::DeckPerspective;
use crate::slay::state::deck::DeckSpec;
use crate::slay::state::game::Game;
use crate::slay::state::game::GamePerspective;
use crate::slay::state::game::GameStaticInformation;
use crate::slay::state::stack::Card;
use crate::slay::state::summarizable::Summarizable;
use crate::slay::state::turn::Turn;
use crate::slay::status_effects::effect::PlayerStatusEffect;
use crate::slay::status_effects::effect_entry::PlayerStatusEffectEntry;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::PlayerTasks;

use enum_iterator::all;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;
use std::iter::Iterator;

use super::stack::ActiveHeroItem;
use super::stack::CardPerspective;
use super::status_effects::PlayerStatusEffects;

pub struct HeroTypeCounter {
	pub leader_type: Option<HeroType>,
	pub counts: HashMap<HeroType, u32>,
}

impl HeroTypeCounter {
	pub fn new() -> Self {
		Self {
			leader_type: None,
			counts: Default::default(),
		}
	}
	pub fn add_hero_type(&mut self, hero_type: &HeroType) {
		self.counts.insert(
			*hero_type,
			if let Some(prev) = self.counts.get(hero_type) {
				prev + 1
			} else {
				1
			},
		);
	}

	pub(crate) fn maybe_add_hero_type(&mut self, hero_type_option: Option<HeroType>) {
		if let Some(hero_type) = hero_type_option.as_ref() {
			self.add_hero_type(hero_type);
		}
	}

	pub fn try_to_satisfy_with_leader(&mut self, hero_type: HeroType) -> bool {
		if let Some(leader_type) = self.leader_type {
			if leader_type == hero_type {
				self.leader_type = None;
				return true;
			}
		}
		false
	}
	pub(crate) fn try_to_take_one_away(&mut self, hero_type: HeroType) -> bool {
		if self.try_to_satisfy_with_leader(hero_type) {
			return true;
		}
		if let Some(count) = self.counts.get(&hero_type) {
			if *count > 0 {
				self.counts.insert(hero_type, count - 1);
				return true;
			}
		}
		false
	}

	pub(crate) fn sum(&self) -> usize {
		let mut ret = 0u32;
		for count in self.counts.values() {
			ret += count;
		}
		ret as usize
	}
}

#[derive(Clone, Debug)]
pub struct Player {
	pub player_index: ids::PlayerIndex,
	pub name: String,

	pub temporary_buffs: PlayerStatusEffects,
	pub choices_: Option<Choices>,
	pub tasks: PlayerTasks,

	pub leader: Card,

	pub hand: Deck,
	pub party: Deck,
	pub slain_monsters: Deck,

	pub visible_hands: HashSet<ids::PlayerIndex>,

	played_this_turn: HashSet<ids::CardId>,
	remaining_action_points: u32,
}

impl Player {
	pub fn choose(&mut self, choices: Choices) {
		self.choices_ = Some(choices);
		// self.tasks.append(ChooseTask::create(choices));
	}
	pub fn set_choices(&mut self, choices: Option<Choices>) {
		self.choices_ = choices;
		// if let Some(choices) = choices {
		// 	self.tasks.append(ChooseTask::create(choices));
		// }
	}

	pub fn put_current_task_back(&mut self, task: Box<dyn PlayerTask>) -> errors::SlayResult<()> {
		self.tasks.put_current_task_back(task)?;
		Ok(())
	}
	pub fn decks(&self) -> [&Deck; 3] {
		[&self.hand, &self.party, &self.slain_monsters]
	}
	pub fn decks_mut(&mut self) -> [&mut Deck; 3] {
		[&mut self.hand, &mut self.party, &mut self.slain_monsters]
	}
	pub fn new(name: String, player_index: ids::PlayerIndex, leader: Card) -> Self {
		Player {
			player_index,
			name,
			choices_: None,
			tasks: Default::default(),
			remaining_action_points: 0,
			leader,
			temporary_buffs: Default::default(),
			hand: Deck::new(DeckSpec {
				visibility: VisibilitySpec::summary(),
				path: DeckPath::Hand(player_index),
			}),
			party: Deck::new(DeckSpec {
				visibility: VisibilitySpec::visible(),
				path: DeckPath::Party(player_index),
			}),
			slain_monsters: Deck::new(DeckSpec {
				visibility: VisibilitySpec::visible(),
				path: DeckPath::SlainMonsters(player_index),
			}),
			played_this_turn: Default::default(),
			visible_hands: Default::default(),
		}
	}

	pub fn turn_begin(&mut self) {
		self.remaining_action_points = self.calculate_total_action_points();
	}

	pub fn action_points_used(&mut self, amount: u32) {
		self.remaining_action_points -= amount;
	}

	pub fn set_card_played(&mut self, card_id: ids::CardId) {
		self.played_this_turn.insert(card_id);
	}

	pub fn was_card_played(&self, card_id: &ids::CardId) -> bool {
		self.played_this_turn.contains(card_id)
	}

	pub fn turn_end(&mut self) {
		self.played_this_turn.clear();
		self.visible_hands.clear();
	}

	pub fn count_hero_types(&self, hero_types: &mut HeroTypeCounter) {
		// Could this be a one liner?
		self.party.count_hero_types(hero_types);
		hero_types.leader_type = self.leader.get_unmodified_hero_type();
	}
	pub fn collect_hero_types(&self, hero_types: &mut HashSet<HeroType>) {
		// Could this be a one liner?
		self.party.collect_hero_types(hero_types);
		hero_types.insert(self.leader.get_unmodified_hero_type().unwrap());
	}
	pub fn has_hero_type(&self, hero_type: &HeroType) -> bool {
		self.leader.get_unmodified_hero_type().unwrap() == *hero_type
			|| self.party.contains_hero_type(hero_type)
	}

	pub fn take_current_task(&mut self) -> Option<Box<dyn PlayerTask>> {
		self.tasks.take_current_task()
	}

	pub fn clear_expired_modifiers(&mut self, turn: &Turn) {
		self.temporary_buffs.clear_expired_modifiers(turn);
	}

	pub(crate) fn get_remaining_action_points(&self) -> u32 {
		self.remaining_action_points
	}
	pub(crate) fn calculate_total_action_points(&self) -> u32 {
		if self.has_player_effect(PlayerStatusEffect::ExtraActionPoint) {
			4
		} else {
			3
		}
	}

	//////////////////////////////////////////////////////////////////////////////
	// status effects
	//////////////////////////////////////////////////////////////////////////////

	pub fn has_player_effect(&self, effect: PlayerStatusEffect) -> bool {
		self.temporary_buffs.has_player_effect(effect);
		guard_unwrap!(
			let SlayCardSpec::PartyLeader(hero_type) = self.leader.card_type
		);
		if let Some(status_effect) = hero_type.get_leader_effect() {
			if status_effect == effect {
				return true;
			}
		}
		for top in self.slain_monsters.tops() {
			guard_unwrap!(
				let SlayCardSpec::MonsterCard(monster) = top.card_type
			);
			if effect == monster.status_effect() {
				return true;
			}
		}
		false
	}

	pub(crate) fn player_effects(&self) -> impl Iterator<Item = PlayerStatusEffectEntry> + '_ {
		guard_unwrap!(
			let SlayCardSpec::PartyLeader(hero_type) = self.leader.card_type
		);
		self
			.temporary_buffs
			.player_effects()
			.chain(self.slain_monsters.tops().map(|top| {
				guard_unwrap!(
					let SlayCardSpec::MonsterCard(monster) = top.card_type
				);
				monster.status_effect_entry()
			}))
			.chain(hero_type.get_leader_effect_entry().into_iter())

		// let temps = self.temporary_buffs.player_effects();
		// if let Some(status_effect) = hero_type.get_leader_effect_entry() {
		// 	effects.push(status_effect);
		// }
		// for top in self.slain_monsters.tops() {
		// 	guard_unwrap!(
		// 		let SlayCardSpec::MonsterCard(monster) = top.card_type
		// 	);
		// 	effects.push(monster.status_effect_entry());
		// }

		// return temps;

		// if let SlayCardSpec::PartyLeader(hero_type) = self.leader.card_type {

		// } else {
		// 	unreachable!();
		// }
		// for top in self.slain_monsters.tops() {
		// 	if let SlayCardSpec::MonsterCard(monster) = top.card_type {
		// 		for modifier in monster.create_spec().modifiers {
		// 			visitor.visit_player_modifier(modifier, EffectOrigin::FromSlainMonster);
		// 		}
		// 	} else {
		// 		unreachable!()
		// 	}
		// }
	}

	pub fn hero_effects(&self) -> impl Iterator<Item = ActiveHeroItem> + '_ {
		self.party.stacks().flat_map(|stack| stack.hero_effects())

		// let mut ret = iter::empty::<(ids::CardId, HeroStatusEffectEntry)>();
		// for stack in self.party.stacks() {
		// 	ret = ret.chain(stack.hero_effects());
		// }
		// ret
	}

	// remove this.
	// pub(crate) fn tour_buffs(&self, visitor: &mut dyn ModifierVisitor) {
	// self.temporary_buffs.tour_buffs(visitor);
	// if let Some(buff) = self.leader.card_type.create_party_leader_buffs() {
	// 	visitor.visit_player_modifier(buff, EffectOrigin::FromPartyLeader(self.leader.id));
	// }
	// for top in self.slain_monsters.tops() {
	// 	if let Some(monster) = top.card_type.get_card_spec_creation().monster {
	// 		for modifier in monster.create_spec().modifiers {
	// 			visitor.visit_player_modifier(modifier, EffectOrigin::FromSlainMonster);
	// 		}
	// 	} else {
	// 		unreachable!()
	// 	}
	// }
	// // Is the following used?
	// for stack in self.party.stacks() {
	// 	for modifier_card in stack.modifiers.iter() {
	// 		if let Some(modifier) = modifier_card
	// 			.card_type
	// 			.get_card_spec_creation()
	// 			.card_modifier
	// 		{
	// 			visitor.visit_card_modifier(modifier, stack.top.card_type)
	// 		}
	// 	}
	// }
	// }

	pub fn to_perspective(&self, game: &Game, perspective: &Perspective) -> PlayerPerspective {
		PlayerPerspective {
			player_index: self.player_index,
			leader: self
				.leader
				.to_perspective(game.was_card_played(Some(self.player_index), self.leader.id)),
			remaining_action_points: self.get_remaining_action_points(),
			// Could be calculated...
			total_action_points: self.calculate_total_action_points(),
			decks: self
				.decks()
				.iter()
				.filter(|d| d.is_visible(perspective))
				.map(|d| d.to_perspective(game, Some(self.player_index), perspective))
				.collect(),
			represented_hero_types: all::<HeroType>()
				.map(|hero_type| RepresentedHeroType {
					hero_type,
					represented: self.has_hero_type(&hero_type),
				})
				.collect(),
		}
	}

	pub(crate) fn has_choices(&self) -> bool {
		self.choices_.is_some()
	}

	pub(crate) fn clear_choices(&mut self) {
		self.choices_ = None;
	}
}

impl Summarizable for Player {
	fn summarize<W: Write>(
		&self,
		f: &mut BufWriter<W>,
		indentation_level: u32,
	) -> Result<(), std::io::Error> {
		for _ in 0..indentation_level {
			write!(f, "  ")?;
		}
		writeln!(f, "player {}", self.player_index)?;
		self.hand.summarize(f, indentation_level + 1)?;
		self.party.summarize(f, indentation_level + 1)?;
		self.slain_monsters.summarize(f, indentation_level + 1)?;

		if let Some(choices) = self.choices_.as_ref() {
			choices.summarize(f, indentation_level + 1)?;
		}
		self.tasks.summarize(f, indentation_level + 1)?;

		Ok(())
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct RepresentedHeroType {
	pub hero_type: HeroType,
	pub represented: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerPerspective {
	pub remaining_action_points: u32,
	pub total_action_points: u32,
	pub decks: Vec<DeckPerspective>,
	pub represented_hero_types: Vec<RepresentedHeroType>,
	pub leader: CardPerspective,
	player_index: usize,
}

impl PlayerPerspective {
	// List the hero types they have
	// Maybe this -> pub choices: Option<Choices>,

	pub fn name<'a>(&self, statics: &'a GameStaticInformation) -> &'a String {
		&statics.players[self.player_index].name
	}

	pub fn is_me(&self, statics: &GameStaticInformation) -> bool {
		self.player_index == statics.player_index
	}
	pub fn is_active(&self, game: &GamePerspective) -> bool {
		self.player_index == game.turn.active_player_index()
	}
	pub fn choices(&self, choices: &Option<ChoicesPerspective>) -> Vec<ChoicePerspective> {
		if let Some(choices) = choices {
			choices.represented_by(&DisplayPath::Player(self.player_index))
		} else {
			Vec::new()
		}
	}
}
