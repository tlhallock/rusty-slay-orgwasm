use crate::slay::specification::HeroType;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::specs::hero::HeroAbilityType;
use crate::slay::specs::items::{AnotherItemType, Item};
use crate::slay::specs::magic::MagicSpell;
use crate::slay::specs::modifier::ModifierKinds;
use crate::slay::specs::monster::Monster;

impl SlayCardSpec {
	pub fn label(&self) -> &'static str {
		match self {
			SlayCardSpec::HeroCard(hero_card) => match hero_card {
				HeroAbilityType::PlunderingPuma => "Plundering Puma",
				HeroAbilityType::SlipperyPaws => "Slippery Paws",
				HeroAbilityType::SmoothMimimeow => "Smooth Mimimeow",
				HeroAbilityType::Meowzio => "Meowzio",
				HeroAbilityType::Shurikitty => "Shurikitty",
				HeroAbilityType::KitNapper => "Kit Napper",
				HeroAbilityType::SilentShadow => "Silent Shadow",
				HeroAbilityType::SlyPickings => "Sly Pickings",
				HeroAbilityType::HolyCurselifter => "Holy Curselifter",
				HeroAbilityType::IronResolve => "Iron Resolve",
				HeroAbilityType::CalmingVoice => "Calming Voice",
				HeroAbilityType::VibrantGlow => "Vibrant Glow",
				HeroAbilityType::MightyBlade => "Mighty Blade",
				HeroAbilityType::RadiantHorn => "Radiant Horn",
				HeroAbilityType::GuidingLight => "Guiding Light",
				HeroAbilityType::WiseShield => "Wise Shield",
				HeroAbilityType::QiBear => "Qi Bear",
				HeroAbilityType::PanChucks => "Pan Chucks",
				HeroAbilityType::HeavyBear => "Heavy Bear",
				HeroAbilityType::BadAxe => "Bad Axe",
				HeroAbilityType::ToughTeddy => "Tough Teddy",
				HeroAbilityType::BearClaw => "Bear Claw",
				HeroAbilityType::FuryKnuckle => "Fury Knuckle",
				HeroAbilityType::BearyWise => "Beary Wise",
				HeroAbilityType::Hook => "Hook",
				HeroAbilityType::Wildshot => "Wildshot",
				HeroAbilityType::SeriousGrey => "Serious Grey",
				HeroAbilityType::WilyRed => "Wily Red",
				HeroAbilityType::QuickDraw => "Quick Draw",
				HeroAbilityType::LookieRookie => "Lookie Rookie",
				HeroAbilityType::Bullseye => "Bullseye",
				HeroAbilityType::SharpFox => "Sharp Fox",
				HeroAbilityType::FuzzyCheeks => "Fuzzy Cheeks",
				HeroAbilityType::Peanut => "Peanut",
				HeroAbilityType::NappingNibbles => "Napping Nibbles",
				HeroAbilityType::TipsyTootie => "Tipsy Tootie",
				HeroAbilityType::MellowDee => "Mellow Dee",
				HeroAbilityType::LuckBucky => "Luck Bucky",
				HeroAbilityType::DodgyDealer => "Dodgy Dealer",
				HeroAbilityType::GreedyCheeks => "Greedy Cheeks",
				HeroAbilityType::Fluffy => "Fluffy",
				HeroAbilityType::Wiggles => "Wiggles",
				HeroAbilityType::Spooky => "Spooky",
				HeroAbilityType::Snowball => "Snowball",
				HeroAbilityType::Buttons => "Buttons",
				HeroAbilityType::BunBun => "Bun Bun",
				HeroAbilityType::Hopper => "Hopper",
				HeroAbilityType::Whiskers => "Whiskers",
			},
			SlayCardSpec::PartyLeader(leader_type) => match leader_type {
				HeroType::Bard => "The Charismatic Song",
				HeroType::Wizard => "The Cloaked Sage",
				HeroType::Fighter => "The Fist of Reason",
				HeroType::Gaurdian => "The Protecting Horn",
				HeroType::Ranger => "The Charismatic Song",
				HeroType::Thief => "The Shadow Claw",
				HeroType::Beserker => todo!(),
				HeroType::Necromancer => todo!(),
				HeroType::Druid => todo!(),
				HeroType::Warrior => todo!(),
				HeroType::Sorcerer => todo!(),
			},
			SlayCardSpec::MonsterCard(monster) => match monster {
				Monster::AnuranCauldron => "Anuran Cauldron",
				Monster::TitanWyvern => "Titan Wyvern",
				Monster::DarkDragonKing => "Dark Dragon King",
				Monster::AbyssQueen => "Abyss Queen",
				Monster::RexMajor => "Rex Major",
				Monster::CorruptedSabretooth => "Corrupted Sabretooth",
				Monster::CrownedSerpent => "Crowned Serpent",
				Monster::WarwornOwlbear => "Warworn Owlbear",
				Monster::Dracos => "Dracos",
				Monster::Malammoth => "Malammoth",
				Monster::Bloodwing => "Bloodwing",
				Monster::ArcticAries => "Arctic Aries",
				Monster::MegaSlime => "Mega Slime",
				Monster::Orthus => "Orthus",
				Monster::Terratuga => "Terratuga",
			},
			SlayCardSpec::MagicCard(spell) => match spell {
				MagicSpell::EnganglingTrap => "Entangling Trap",
				MagicSpell::CriticalBoost => "Critical Boost",
				MagicSpell::DestructiveSpell => "Destructive Spell",
				MagicSpell::WindsOfChange => "Winds of Change",
				MagicSpell::EnchangedSpell => "Enchanted Spell",
				MagicSpell::ForcedExchange => "Forced Exchange",
				MagicSpell::ForcefulWinds => "Forceful Winds",
				MagicSpell::CallToTheFallen => "Call to the Fallen",
			},
			SlayCardSpec::ModifierCard(modifier_kind) => match modifier_kind {
				ModifierKinds::Plus4 => "Modifier +4",
				ModifierKinds::Plus3Minus1 => "Modifier +3/-1",
				ModifierKinds::Plus2Minus2 => "Modifier +2/-2",
				ModifierKinds::Plus1Minus3 => "Modifier +3/-3",
				ModifierKinds::Minus4 => "Modifier -4",
			},
			SlayCardSpec::Item(item_card) => match item_card {
				AnotherItemType::MaskCard(hero_type) => match hero_type {
					HeroType::Bard => "Bard Mask",
					HeroType::Wizard => "Wizard Mask",
					HeroType::Fighter => "Fighter Mask",
					HeroType::Gaurdian => "Guardian Mask",
					HeroType::Ranger => "Ranger Mask",
					HeroType::Thief => "Thief Mask",
					HeroType::Beserker => todo!(),
					HeroType::Necromancer => todo!(),
					HeroType::Druid => todo!(),
					HeroType::Warrior => todo!(),
					HeroType::Sorcerer => todo!(),
				},
				AnotherItemType::NotMask(item) => match item {
					Item::DecoyDoll => "Decoy Doll",
					Item::ReallyBigRing => "Really Big Ring",
					Item::ParticularlyRustyCoin => "Particularly Rusty Coin",
					Item::SealingKey => "Sealing Key",
					Item::SuspiciouslyShinyCoin => "Suspiciously Shiny Coin",
					Item::CurseOfTheSnakesEyes => "Curse of the Snake's Eyes",
				},
			},
			SlayCardSpec::Challenge => "Challenge",
		}
	}
}
