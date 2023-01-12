use crate::slay::ids;
use crate::slay::state::game::Game;

use super::modifier_visitors::PlayerHasModifier;
use super::modifiers::PlayerModifier;

pub fn player_drew() {}
pub fn player_modified() {}
pub fn player_was_challenged() {}
pub fn player_drew_magic_card() {}
pub fn player_drew_modifier() {}
pub fn player_drew_item_card() {}
pub fn player_successfully_rolled_for_ability() {}

pub fn hero_is_destroyable() {}

// DrawOnDestroy,
// StealInsteadOfSacrifice,
// DrawOnPlayMagic,
// ModifierBonus,

// DrawOnModify,
// AddOnModify,
