use crate::slay::ids;
use crate::slay::modifiers::ModifierOrigin;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::state::game::Game;

use crate::slay::modifiers::ItemModifier;
use crate::slay::modifiers::PlayerModifier;
use crate::slay::showdown::roll_modification::ModificationOrigin;
use crate::slay::showdown::roll_modification::RollModification;
use crate::slay::showdown::roll_state::RollReason;

pub fn create_roll_history(
	game: &Game,
	player_index: ids::PlayerIndex,
	reason: RollReason,
) -> Vec<RollModification> {
	let mut visitor = CreateRollModifications::new(reason);
	game.players[player_index].tour_buffs(&mut visitor);
	visitor.modifications
}

pub trait ModifierVisitor {
	fn visit_player_modifier(&mut self, _modifier: PlayerModifier, _origin: ModifierOrigin) {}

	fn visit_card_modifier(&mut self, _modifier: ItemModifier, _modified_spec: SlayCardSpec) {}
}

pub struct CountActionPoints {
	pub count: u32,
}

impl CountActionPoints {
	pub fn new() -> Self {
		Self { count: 3 }
	}
}

impl ModifierVisitor for CountActionPoints {
	fn visit_player_modifier(&mut self, modifier: PlayerModifier, _origin: ModifierOrigin) {
		if matches!(modifier, PlayerModifier::ExtraActionPoint) {
			self.count += 1;
		}
	}
}

pub struct CreateRollModifications {
	reason: RollReason,
	pub modifications: Vec<RollModification>,
}

impl CreateRollModifications {
	pub fn new(reason: RollReason) -> Self {
		Self {
			reason,
			modifications: Default::default(),
		}
	}
}

impl ModifierVisitor for CreateRollModifications {
	fn visit_player_modifier(&mut self, modifier: PlayerModifier, origin: ModifierOrigin) {
		match modifier {
			PlayerModifier::AddToAllRolls(amount) => self.modifications.push(RollModification {
				origin: ModificationOrigin::FromBuff(origin),
				amount,
			}),
			PlayerModifier::AddToRollForAnyAbility(amount) => match self.reason {
				RollReason::UseHeroAbility(_) => {
					self.modifications.push(RollModification {
						origin: ModificationOrigin::FromBuff(origin),
						amount,
					});
				}
				_ => {}
			},
			PlayerModifier::AddToRollForChallenge(amount) => match self.reason {
				// TODO: both?
				RollReason::Challenged | RollReason::Challenging => {
					self.modifications.push(RollModification {
						origin: ModificationOrigin::FromBuff(origin),
						amount,
					});
				}
				_ => {}
			},
			_ => {}
		}
	}

	fn visit_card_modifier(&mut self, modifier: ItemModifier, modified_spec: SlayCardSpec) {
		match modifier {
			ItemModifier::AddToRollForAbility(amount) => match self.reason {
				RollReason::UseHeroAbility(spec) => {
					if modified_spec == spec {
						self.modifications.push(RollModification {
							origin: ModificationOrigin::FromBuff(ModifierOrigin::FromItem),
							amount,
						});
					}
				}
				_ => {}
			},
			_ => {}
		}
	}
}

/*
elf
			.slain_monsters
			.tops()
			.map(|card| {
				if let Some(monster) = card.card_type.get_card_spec_creation().monster {
					monster
						.create_spec()
						.modifiers
						.iter()
						.map(|modifier| match modifier {
							PlayerModifier::ExtraActionPoint => 1,
							_ => 0,
						})
						.sum::<u32>()
				} else {
					unreachable!()
				}
			})
			.sum::<u32>()

			*/

pub struct PlayerHasModifier {
	modifier: PlayerModifier,
	pub has: bool,
}

impl PlayerHasModifier {
	pub fn new(modifier: PlayerModifier) -> Self {
		Self {
			modifier,
			has: false,
		}
	}
}

impl ModifierVisitor for PlayerHasModifier {
	fn visit_player_modifier(&mut self, modifier: PlayerModifier, _origin: ModifierOrigin) {
		self.has |= modifier == self.modifier;
	}
}

pub struct CardHasModifier {
	modifier: ItemModifier,
	pub has: bool,
}

impl CardHasModifier {
	pub fn new(modifier: ItemModifier) -> Self {
		Self {
			modifier,
			has: false,
		}
	}
}

impl ModifierVisitor for CardHasModifier {
	fn visit_card_modifier(&mut self, modifier: ItemModifier, _modified_spec: SlayCardSpec) {
		self.has |= modifier == self.modifier;
	}
}
