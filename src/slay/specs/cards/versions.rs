use crate::slay::specification::{GameVersion};
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::specs::items::{AnotherItemType};


impl SlayCardSpec {
	pub fn game_version(&self) -> GameVersion {
		match self {
			SlayCardSpec::HeroCard(hero_card) => hero_card.hero_type().game_version(),
			SlayCardSpec::MonsterCard(monster) => match monster {
				_ => GameVersion::Original,
			},
			SlayCardSpec::MagicCard(spell) => match spell {
				_ => GameVersion::Original,
			},
			SlayCardSpec::ModifierCard(modifier_kind) => match modifier_kind {
				_ => GameVersion::Original,
			},
			SlayCardSpec::Item(item_card) => match item_card {
				AnotherItemType::MaskCard(hero_type) => hero_type.game_version(),
				AnotherItemType::NotMask(item) => match item {
					_ => GameVersion::Original,
				},
			},
			SlayCardSpec::Challenge => GameVersion::Original,
			SlayCardSpec::PartyLeader(hero_type) => hero_type.game_version(),
		}
	}
}
