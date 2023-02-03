use enum_iterator::Sequence;

use crate::slay::ids;
use crate::slay::modifiers::PlayerModifier;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::specification::HeroType;
use crate::slay::specification::MonsterRequirements;
use crate::slay::specification::MonsterSpec;
use crate::slay::state::player::HeroTypeCounter;
use crate::slay::tasks::core::discard::Discard;
use crate::slay::tasks::core::draw::DrawTask;
use crate::slay::tasks::core::sacrifice::Sacrifice;

use super::cards::card_type::SlayCardSpec;

pub fn player_satisfies_requirements(
	hero_type_counts: &mut HeroTypeCounter,
	requirements: &mut Vec<MonsterRequirements>,
) -> bool {
	// First try to use the party leader if possible:

	hero_type_counts.leader_type.unwrap();

	for i in 0..requirements.len() {
		if requirements[i].satisfied_by_party_leader(hero_type_counts.leader_type.unwrap()) {
			requirements.remove(i);
			hero_type_counts.leader_type = None;
			break;
		}
	}

	// assign hero type requirements
	'outer: loop {
		for i in 0..requirements.len() {
			if let MonsterRequirements::HeroType(hero_type) = requirements[i] {
				if hero_type_counts.try_to_take_one_away(hero_type) {
					requirements.remove(i);
					continue 'outer;
				}
				return false;
			}
		}
		break;
	}
	// Just checkin...
	for i in 0..requirements.len() {
		if let MonsterRequirements::HeroType(_hero_type) = requirements[i] {
			unreachable!()
		}
	}

	requirements.len() < hero_type_counts.sum()
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
	pub fn label(&self) -> &'static str {
		SlayCardSpec::MonsterCard(*self).label()
	}

	pub fn get_consequences(&self, card_id: ids::CardId) -> RollConsequences {
		self.create_spec().get_consequences(card_id)
	}

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
