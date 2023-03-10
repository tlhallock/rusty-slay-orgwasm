use crate::slay::specification::HeroType;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::specs::hero::HeroAbilityType;
use crate::slay::specs::items::{AnotherItemType, Item};
use crate::slay::specs::magic::MagicSpell;
use crate::slay::specs::modifier::ModifierKinds;
use crate::slay::specs::monster::Monster;

impl SlayCardSpec {
	pub fn image_path(&self) -> &'static str {
		match self {
			SlayCardSpec::HeroCard(hero_card) => match hero_card {
				HeroAbilityType::PlunderingPuma => "imgs/cards/heros/thief/plundering_puma.jpg",
				HeroAbilityType::SlipperyPaws => "imgs/cards/heros/thief/slippery_paws.jpg",
				HeroAbilityType::SmoothMimimeow => "imgs/cards/heros/thief/smooth_mimimeow.jpg",
				HeroAbilityType::Meowzio => "imgs/cards/heros/thief/meowzio.jpg",
				HeroAbilityType::Shurikitty => "imgs/cards/heros/thief/shurikitty.jpg",
				HeroAbilityType::KitNapper => "imgs/cards/heros/thief/kit_napper.jpg",
				HeroAbilityType::SilentShadow => "imgs/cards/heros/thief/silent_shadow.jpg",
				HeroAbilityType::SlyPickings => "imgs/cards/heros/thief/sly_pickings.jpg",
				HeroAbilityType::HolyCurselifter => "imgs/cards/heros/guardian/holy_curse_lifter.jpg",
				HeroAbilityType::IronResolve => "imgs/cards/heros/guardian/iron_resolve.jpg",
				HeroAbilityType::CalmingVoice => "imgs/cards/heros/guardian/calming_voice.jpg",
				HeroAbilityType::VibrantGlow => "imgs/cards/heros/guardian/vibrant_glow.jpg",
				HeroAbilityType::MightyBlade => "imgs/cards/heros/guardian/mighty_blade.jpg",
				HeroAbilityType::RadiantHorn => "imgs/cards/heros/guardian/radiant_horn.jpg",
				HeroAbilityType::GuidingLight => "imgs/cards/heros/guardian/guiding_light.jpg",
				HeroAbilityType::WiseShield => "imgs/cards/heros/guardian/wise_shield.jpg",
				HeroAbilityType::QiBear => "imgs/cards/heros/fighter/chi_bear.jpg",
				HeroAbilityType::PanChucks => "imgs/cards/heros/fighter/pan_chucks.jpg",
				HeroAbilityType::HeavyBear => "imgs/cards/heros/fighter/heavy_bear.jpg",
				HeroAbilityType::BadAxe => "imgs/cards/heros/fighter/bad_axe.jpg",
				HeroAbilityType::ToughTeddy => "imgs/cards/heros/fighter/tough_teddy.jpg",
				HeroAbilityType::BearClaw => "imgs/cards/heros/fighter/bear_claw.jpg",
				HeroAbilityType::FuryKnuckle => "imgs/cards/heros/fighter/fury_knuckle.jpg",
				HeroAbilityType::BearyWise => "imgs/cards/heros/fighter/beary_wise.jpg",
				HeroAbilityType::Hook => "imgs/cards/heros/ranger/hook.jpg",
				HeroAbilityType::Wildshot => "imgs/cards/heros/ranger/wildshot.jpg",
				HeroAbilityType::SeriousGrey => "imgs/cards/heros/ranger/serious_grey.jpg",
				HeroAbilityType::WilyRed => "imgs/cards/heros/ranger/wily_red.jpg",
				HeroAbilityType::QuickDraw => "imgs/cards/heros/ranger/quick_draw.jpg",
				HeroAbilityType::LookieRookie => "imgs/cards/heros/ranger/lookie_rookie.jpg",
				HeroAbilityType::Bullseye => "imgs/cards/heros/ranger/bullseye.jpg",
				HeroAbilityType::SharpFox => "imgs/cards/heros/ranger/sharp_fox.jpg",
				HeroAbilityType::FuzzyCheeks => "imgs/cards/heros/bards/fuzzy_cheeks.jpg",
				HeroAbilityType::Peanut => "imgs/cards/heros/bards/peanut.jpg",
				HeroAbilityType::NappingNibbles => "imgs/cards/heros/bards/napping_nibbles.jpg",
				HeroAbilityType::TipsyTootie => "imgs/cards/heros/bards/tipsy_tootie.jpg",
				HeroAbilityType::MellowDee => "imgs/cards/heros/bards/mellow_dee.jpg",
				HeroAbilityType::LuckBucky => "imgs/cards/heros/bards/lucky_bucky.jpg",
				HeroAbilityType::DodgyDealer => "imgs/cards/heros/bards/dodgy_dealer.jpg",
				HeroAbilityType::GreedyCheeks => "imgs/cards/heros/bards/greedy_cheeks.jpg",
				HeroAbilityType::Fluffy => "imgs/cards/heros/wizard/fluffy.jpg",
				HeroAbilityType::Wiggles => "imgs/cards/heros/wizard/wiggles.jpg",
				HeroAbilityType::Spooky => "imgs/cards/heros/wizard/spooky.jpg",
				HeroAbilityType::Snowball => "imgs/cards/heros/wizard/snowball.jpg",
				HeroAbilityType::Buttons => "imgs/cards/heros/wizard/buttons.jpg",
				HeroAbilityType::BunBun => "imgs/cards/heros/wizard/bun_bun.jpg",
				HeroAbilityType::Hopper => "imgs/cards/heros/wizard/hopper.jpg",
				HeroAbilityType::Whiskers => "imgs/cards/heros/wizard/whiskers.jpg",
			},
			SlayCardSpec::PartyLeader(leader_type) => match leader_type {
				HeroType::Bard => "imgs/cards/party_leaders/charismatic_song.jpg",
				HeroType::Wizard => "imgs/cards/party_leaders/cloaked_sage.jpg",
				HeroType::Fighter => "imgs/cards/party_leaders/fist_of_reason.jpg",
				HeroType::Gaurdian => "imgs/cards/party_leaders/protecting_horn.jpg",
				HeroType::Ranger => "imgs/cards/party_leaders/divine_arrow.jpg",
				HeroType::Thief => "imgs/cards/party_leaders/shadow_claw.jpg",

				HeroType::Necromancer => todo!(),
				HeroType::Druid => todo!(),
				HeroType::Warrior => todo!(),
				HeroType::Sorcerer => todo!(),
				HeroType::Beserker => todo!(),
			},
			SlayCardSpec::MonsterCard(monster) => match monster {
				Monster::AnuranCauldron => "imgs/cards/monsters/anuran_cauldron.jpg",
				Monster::TitanWyvern => "imgs/cards/monsters/titan_wyvern.jpg",
				Monster::DarkDragonKing => "imgs/cards/monsters/dark_dragon_king.jpg",
				Monster::AbyssQueen => "imgs/cards/monsters/abyss_queen.jpg",
				Monster::RexMajor => "imgs/cards/monsters/rex_major.jpg",
				Monster::CorruptedSabretooth => "imgs/cards/monsters/corrupted_sabretooth.jpg",
				Monster::CrownedSerpent => "imgs/cards/monsters/crowned_serpent.jpg",
				Monster::WarwornOwlbear => "imgs/cards/monsters/warworn_owlbear.jpg",
				Monster::Dracos => "imgs/cards/monsters/dracos.jpg",
				Monster::Malammoth => "imgs/cards/monsters/malamammoth.jpg",
				Monster::Bloodwing => "imgs/cards/monsters/bloodwing.jpg",
				Monster::ArcticAries => "imgs/cards/monsters/arctic_aries.jpg",
				Monster::MegaSlime => "imgs/cards/monsters/mega_slime.jpg",
				Monster::Orthus => "imgs/cards/monsters/orthus.jpg",
				Monster::Terratuga => "imgs/cards/monsters/terratuga.jpg",
			},
			SlayCardSpec::MagicCard(spell) => match spell {
				MagicSpell::EnganglingTrap => "imgs/cards/magic/entangling_trap.jpg",
				MagicSpell::CriticalBoost => "imgs/cards/magic/critical_boost.jpg",
				MagicSpell::DestructiveSpell => "imgs/cards/magic/descructive_spell.jpg",
				MagicSpell::WindsOfChange => "imgs/cards/magic/winds_of_change.jpg",
				MagicSpell::EnchangedSpell => "imgs/cards/magic/enchanged_spell.jpg",
				MagicSpell::ForcedExchange => "imgs/cards/magic/forced_exchange.jpg",
				MagicSpell::ForcefulWinds => "imgs/cards/magic/forceful_winds.jpg",
				MagicSpell::CallToTheFallen => "imgs/cards/magic/call_to_the_fallen.jpg",
			},
			SlayCardSpec::ModifierCard(modifier_kind) => match modifier_kind {
				ModifierKinds::Plus4 => "imgs/cards/modifier/4.jpg",
				ModifierKinds::Plus3Minus1 => "imgs/cards/modifier/3.jpg",
				ModifierKinds::Plus2Minus2 => "imgs/cards/modifier/2.jpg",
				ModifierKinds::Plus1Minus3 => "imgs/cards/modifier/1.jpg",
				ModifierKinds::Minus4 => "imgs/cards/modifier/0.jpg",
			},
			SlayCardSpec::Item(item_card) => match item_card {
				AnotherItemType::MaskCard(hero_type) => match hero_type {
					HeroType::Bard => "imgs/cards/items/bard_mask.jpg",
					HeroType::Wizard => "imgs/cards/items/wizard_mask.jpg",
					HeroType::Fighter => "imgs/cards/items/fighter_mask.jpg",
					HeroType::Gaurdian => "imgs/cards/items/guardian_mask.jpg",
					HeroType::Ranger => "imgs/cards/items/ranger_mask.jpg",
					HeroType::Thief => "imgs/cards/items/thief_mask.jpg",
					HeroType::Sorcerer => todo!(),
					HeroType::Beserker => todo!(),
					HeroType::Necromancer => todo!(),
					HeroType::Druid => todo!(),
					HeroType::Warrior => todo!(),
				},
				AnotherItemType::NotMask(item) => match item {
					Item::DecoyDoll => "imgs/cards/items/decoy_doll.jpg",
					Item::ReallyBigRing => "imgs/cards/items/really_big_ring.jpg",
					Item::ParticularlyRustyCoin => "imgs/cards/items/particularly_rusty_coin.jpg",
					Item::SealingKey => "imgs/cards/cursed_items/sealing_key.jpg",
					Item::SuspiciouslyShinyCoin => "imgs/cards/cursed_items/suspiciously_shiny_coin.jpg",
					Item::CurseOfTheSnakesEyes => "imgs/cards/cursed_items/curse_of_the_snakes_eyes.jpg",
				},
			},
			SlayCardSpec::Challenge => "imgs/cards/challenge/challenge.jpg",
		}
	}
}
