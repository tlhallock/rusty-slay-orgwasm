use crate::slay::ids;
use crate::slay::showdown::roll_modification::RollModification;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::state::game::Game;

// Move to roll reason?
pub fn create_roll_history(
	game: &Game,
	player_index: ids::PlayerIndex,
	reason: RollReason,
) -> Vec<RollModification> {
	// TODO: DRY, how to do this?
	if let RollReason::UseHeroAbility(hero) = reason {
		////////////////////////////////////////////////////////////////////////////
		game.players[player_index]
			.player_effects()
			.flat_map(|effect| effect.create_roll_modification(reason))
			////////////////////////////////////////////////////////////////////////////
			.chain(
				game.players[player_index]
					.hero_effects()
					.filter(|item| item.hero == hero)
					.flat_map(|item| item.effect.create_roll_modification(reason)),
			)
			.collect()
	} else {
		////////////////////////////////////////////////////////////////////////////
		game.players[player_index]
			.player_effects()
			.flat_map(|effect| effect.create_roll_modification(reason))
			////////////////////////////////////////////////////////////////////////////
			.collect()
	}
}

// pub struct CountActionPoints {
// 	pub count: u32,
// }

// impl CountActionPoints {
// 	pub fn new() -> Self {
// 		Self { count: 3 }
// 	}
// }

// impl ModifierVisitor for CountActionPoints {
// 	fn visit_player_modifier(&mut self, modifier: PlayerStatusEffect, _origin: EffectOrigin) {
// 		if matches!(modifier, PlayerStatusEffect::ExtraActionPoint) {
// 			self.count += 1;
// 		}
// 	}
// }

// pub struct CreateRollModifications {
// 	reason: RollReason,
// 	pub modifications: Vec<RollModification>,
// }

// impl CreateRollModifications {
// 	pub fn new(reason: RollReason) -> Self {
// 		Self {
// 			reason,
// 			modifications: Default::default(),
// 		}
// 	}
// }

// pub fn visit_player_modifier(
// 	effect: PlayerStatusEffectEntry,
// 	modifications: Vec<RollModification>,
// ) {
// 	match effect.modifier {
// 		PlayerStatusEffect::AddToAllRolls(amount) => modifications.push(RollModification {
// 			origin: ModificationOrigin::FromBuff(origin),
// 			amount,
// 		}),
// 		PlayerStatusEffect::AddToRollForAnyAbility(amount) => match self.reason {
// 			RollReason::UseHeroAbility(_) => {
// 				self.modifications.push(RollModification {
// 					origin: ModificationOrigin::FromBuff(origin),
// 					amount,
// 				});
// 			}
// 			_ => {}
// 		},
// 		PlayerStatusEffect::AddToRollForChallenge(amount) => match self.reason {
// 			// TODO: both?
// 			RollReason::Challenged | RollReason::Challenging => {
// 				self.modifications.push(RollModification {
// 					origin: ModificationOrigin::FromBuff(origin),
// 					amount,
// 				});
// 			}
// 			_ => {}
// 		},
// 		_ => {}
// 	}
// }

// impl ModifierVisitor for CreateRollModifications {
// 	fn visit_player_modifier(&mut self, modifier: PlayerStatusEffect, origin: EffectOrigin) {

// 	}

// 	fn visit_card_modifier(&mut self, modifier: HeroStatusEffect, modified_spec: SlayCardSpec) {

// 	}
// }

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

// pub struct PlayerHasModifier {
// 	modifier: PlayerStatusEffect,
// 	pub has: bool,
// }

// impl PlayerHasModifier {
// 	pub fn new(modifier: PlayerStatusEffect) -> Self {
// 		Self {
// 			modifier,
// 			has: false,
// 		}
// 	}
// }

// impl ModifierVisitor for PlayerHasModifier {
// 	fn visit_player_modifier(&mut self, modifier: PlayerStatusEffect, _origin: EffectOrigin) {
// 		self.has |= modifier == self.modifier;
// 	}
// }

// pub struct CardHasModifier {
// 	modifier: HeroStatusEffect,
// 	pub has: bool,
// }

// impl CardHasModifier {
// 	pub fn new(modifier: HeroStatusEffect) -> Self {
// 		Self {
// 			modifier,
// 			has: false,
// 		}
// 	}
// }

// impl ModifierVisitor for CardHasModifier {
// 	fn visit_card_modifier(&mut self, modifier: HeroStatusEffect, _modified_spec: SlayCardSpec) {
// 		self.has |= modifier == self.modifier;
// 	}
// }
