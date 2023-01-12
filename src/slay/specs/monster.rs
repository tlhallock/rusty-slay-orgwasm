use enum_iterator::Sequence;

use crate::slay::{
	abilities::{discard::Discard, sacrifice::Sacrifice},
	actions::DrawTask,
	modifiers::PlayerModifier,
	showdown::consequences::{Condition, RollConsequence, RollConsequences},
	specification::{HeroType, MonsterRequirements, MonsterSpec},
	state::player::HeroTypeCounter,
};

pub fn player_satisfies_requirements(
	_hero_type_counts: &HeroTypeCounter,
	_requirements: &Vec<MonsterRequirements>,
) -> bool {
	let _remaining_heros = 3;

	// This information is not enough: The party leader is not a hero...

	// let assignment = vec![];
	// First assign any specific variants

	false
}

#[derive(Debug, Clone, Sequence, PartialEq, Copy)]
pub enum Monster {
	AnuranCauldron,
	TitanWyvern,
	DarkDragonKing,
	AbyssQueen,
	RexMajor,
	CorruptedSabretooth,
	CrownedSerpent,
	WarwornOwlbear,
	Dracos,
	Malammoth,
	Bloodwing,
	ArcticAries,
	MegaSlime,
	Orthus,
	Terratuga,
}

impl Monster {
	pub fn create_spec(&self) -> MonsterSpec {
		match self {
			Monster::AnuranCauldron => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(7),
						tasks: vec![],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(6),
						tasks: vec![Sacrifice::create(1)],
					}),
				},
				requirements: vec![MonsterRequirements::Hero; 3],
				modifiers: vec![PlayerModifier::AddToAllRolls(1)],
			},
			Monster::TitanWyvern => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(8),
						tasks: vec![],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(4),
						tasks: vec![Discard::create(2)],
					}),
				},
				requirements: vec![
					MonsterRequirements::HeroType(HeroType::Fighter),
					MonsterRequirements::Hero,
				],
				modifiers: vec![PlayerModifier::AddToRollForChallenge(1)],
			},
			Monster::DarkDragonKing => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(400),
						tasks: vec![],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(-1),
						tasks: vec![Discard::create(2)],
					}),
				},
				requirements: vec![
					MonsterRequirements::HeroType(HeroType::Bard),
					MonsterRequirements::Hero,
				],
				modifiers: vec![PlayerModifier::AddToRollForAnyAbility(1)],
			},
			Monster::AbyssQueen => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(8),
						tasks: vec![],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(5),
						tasks: vec![Sacrifice::create(1)],
					}),
				},
				requirements: vec![MonsterRequirements::Hero; 2],
				modifiers: vec![PlayerModifier::AddOnModify],
			},
			Monster::RexMajor => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(8),
						tasks: vec![],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(4),
						tasks: vec![Discard::create(2)],
					}),
				},
				requirements: vec![
					MonsterRequirements::HeroType(HeroType::Gaurdian),
					MonsterRequirements::Hero,
				],
				modifiers: vec![PlayerModifier::RevealModifiersAndDrawAgain],
			},
			Monster::CorruptedSabretooth => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(9),
						tasks: vec![
							DrawTask::create(1),
							// TODO...
						],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(6),
						tasks: vec![Sacrifice::create(1)],
					}),
				},
				requirements: vec![MonsterRequirements::Hero; 3],
				modifiers: vec![PlayerModifier::StealInsteadOfSacrifice],
			},
			Monster::CrownedSerpent => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(10),
						tasks: vec![],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(7),
						tasks: vec![Sacrifice::create(1)],
					}),
				},
				requirements: vec![MonsterRequirements::Hero; 2],
				modifiers: vec![PlayerModifier::DrawOnModify],
			},
			Monster::WarwornOwlbear => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(8),
						tasks: vec![],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(4),
						tasks: vec![Discard::create(2)],
					}),
				},
				requirements: vec![
					MonsterRequirements::HeroType(HeroType::Thief),
					MonsterRequirements::Hero,
				],
				modifiers: vec![PlayerModifier::ItemsCannotBeChallenged],
			},
			Monster::Dracos => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::le(5),
						tasks: vec![],
					},
					loss: Some(RollConsequence {
						condition: Condition::ge(8),
						tasks: vec![Sacrifice::create(1)],
					}),
				},
				requirements: vec![MonsterRequirements::Hero],
				modifiers: vec![PlayerModifier::DrawOnDestroy],
			},
			Monster::Malammoth => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(8),
						tasks: vec![],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(4),
						tasks: vec![Discard::create(2)],
					}),
				},
				requirements: vec![
					MonsterRequirements::Hero,
					MonsterRequirements::HeroType(HeroType::Ranger),
				],
				modifiers: vec![PlayerModifier::PlayItemOnDraw],
			},
			Monster::Bloodwing => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(9),
						tasks: vec![],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(6),
						tasks: vec![Sacrifice::create(1)],
					}),
				},
				requirements: vec![MonsterRequirements::Hero; 2],
				modifiers: vec![PlayerModifier::DiscardOnChallenge],
			},
			Monster::ArcticAries => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(10),
						tasks: vec![],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(6),
						tasks: vec![Sacrifice::create(1)],
					}),
				},
				requirements: vec![MonsterRequirements::Hero],
				modifiers: vec![PlayerModifier::DrawOnSuccessfulAbility],
			},
			Monster::MegaSlime => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(8),
						tasks: vec![DrawTask::create(2)],
					},
					loss: Some(RollConsequence {
						condition: Condition::le(7),
						tasks: vec![Sacrifice::create(2)],
					}),
				},
				requirements: vec![MonsterRequirements::Hero; 4],
				modifiers: vec![PlayerModifier::ExtraActionPoint],
			},
			Monster::Orthus => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(8),
						tasks: Vec::new(),
					},
					loss: Some(RollConsequence {
						condition: Condition::le(4),
						tasks: vec![Sacrifice::create(1)],
					}),
				},
				requirements: vec![
					MonsterRequirements::Hero,
					MonsterRequirements::HeroType(HeroType::Wizard),
				],
				modifiers: vec![PlayerModifier::PlayMagicOnDraw],
			},
			Monster::Terratuga => MonsterSpec {
				consequences: RollConsequences {
					success: RollConsequence {
						condition: Condition::ge(11),
						tasks: Vec::new(),
					},
					loss: Some(RollConsequence {
						condition: Condition::le(7),
						tasks: vec![Discard::create(2)],
					}),
				},
				requirements: vec![MonsterRequirements::Hero],
				modifiers: vec![PlayerModifier::UndestroyableHeros],
			},
		}
	}
}
