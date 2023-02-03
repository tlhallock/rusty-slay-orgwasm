use crate::slay::specification::GameVersion;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::specs::items::AnotherItemType;

impl SlayCardSpec {
	pub fn game_version(&self) -> GameVersion {
		match self {
			SlayCardSpec::HeroCard(hero_card) => hero_card.hero_type().game_version(),
			SlayCardSpec::MonsterCard(_monster) => GameVersion::Original,
			SlayCardSpec::MagicCard(_spell) => GameVersion::Original,
			SlayCardSpec::ModifierCard(_modifier_kind) => GameVersion::Original,
			SlayCardSpec::Item(item_card) => match item_card {
				AnotherItemType::MaskCard(hero_type) => hero_type.game_version(),
				AnotherItemType::NotMask(_item) => GameVersion::Original,
			},
			SlayCardSpec::Challenge => GameVersion::Original,
			SlayCardSpec::PartyLeader(hero_type) => hero_type.game_version(),
		}
	}
}
