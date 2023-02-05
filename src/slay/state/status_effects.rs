use crate::slay::status_effects::{
	effect::PlayerStatusEffect,
	effect_entry::{EffectOrigin, PlayerStatusEffectEntry},
	temp_effect::{EffectDuration, TemporaryPlayerStatusEffect},
};

use super::turn::Turn;

#[derive(Clone, Debug, Default)]
pub struct PlayerStatusEffects {
	temporary_effects: Vec<TemporaryPlayerStatusEffect>,
}
impl PlayerStatusEffects {
	pub(crate) fn player_effects(&self) -> impl Iterator<Item = PlayerStatusEffectEntry> + '_ {
		self
			.temporary_effects
			.iter()
			.map(|temp_buff| temp_buff.status_effect.to_owned())
	}
	pub fn has_player_effect(&self, effect: PlayerStatusEffect) -> bool {
		self
			.temporary_effects
			.iter()
			.any(|temp_effect| temp_effect.status_effect.modifier == effect)
	}

	// pub(crate) fn tour_buffs(&self, visitor: &mut dyn ModifierVisitor) {
	// 	for buff in self.temporary_buffs.iter() {
	// 		visitor.visit_player_modifier(buff.modifier, buff.origin)
	// 	}
	// }

	pub(crate) fn clear_expired_modifiers(&mut self, turn: &Turn) {
		self
			.temporary_effects
			.retain(|b| turn.still_active(&b.duration))
	}

	pub fn add_buff(
		&mut self,
		duration: EffectDuration,
		modifier: PlayerStatusEffect,
		origin: EffectOrigin,
	) {
		self.temporary_effects.push(TemporaryPlayerStatusEffect {
			duration,
			status_effect: PlayerStatusEffectEntry { modifier, origin },
		});
	}
	pub fn add(&mut self, effect: TemporaryPlayerStatusEffect) {
		self.temporary_effects.push(effect);
	}
	// pub fn add(&mut self, duration: ModifierDuration, modifier: PlayerModifier) {
	// 	self.buffs.push(PlayerBuff { duration, modifier });
	// }
	// pub fn add_forever(&mut self, modifier: PlayerModifier, origin: ModifierOrigin) {
	// 	self.add(PlayerBuff {
	// 		duration: ModifierDuration::Forever,
	// 		modifier,
	// 		origin,
	// 	});
	// }

	// pub(crate) fn collect_roll_buffs(&self, ret: &mut Vec<RollModification>) {
	// 	for modifier in self.temporary_buffs.iter() {
	// 		if let PlayerModifier::AddToAllRolls(amount) = &modifier.modifier {
	// 			ret.push(RollModification {
	// 				modification_origin: ModificationOrigin::FromBuff(modifier.origin),
	// 				modification_amount: *amount,
	// 			})
	// 		}
	// 	}
	// }
}
