use crate::slay::{specification::{MonsterSpec, HeroType, MonsterRequirements}, abilities::{sacrifice::Sacrifice, discard::Discard}, modifiers::{PlayerModifier, ModifierOrigin}, showdown::consequences::{RollConsequence, Condition, RollConsequences}, tasks::ReceiveModifier, actions::DrawTask, state::player::HeroTypeCounter};




pub fn player_satisfies_requirements(
  hero_type_counts: &HeroTypeCounter,
  requirements: &Vec<MonsterRequirements>,
) -> bool {
  let remaining_heros  = 3;

  // This information is not enough: The party leader is not a hero...

  // let assignment = vec![];
  // First assign any specific variants

  false
}





#[derive(Debug, Clone)]
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
            loss: Some(
              RollConsequence {
                condition: Condition::le(6),
                tasks: vec![Sacrifice::create(1)],
              }
            )
        },
        requirements: vec![MonsterRequirements::Hero; 3],
    },
      Monster::TitanWyvern => MonsterSpec {
        consequences: RollConsequences {
            success: RollConsequence {
                condition: Condition::ge(8),
                tasks: vec![],
            },
            loss: Some(
              RollConsequence {
                condition: Condition::le(4),
                tasks: vec![Discard::create(2)],
              }
            )
        },
        requirements: vec![
          MonsterRequirements::HeroType(HeroType::Fighter),
          MonsterRequirements::Hero,
        ],
    },
      Monster::DarkDragonKing => MonsterSpec {
        consequences: RollConsequences {
            success: RollConsequence {
                condition: Condition::ge(400),
                tasks: vec![],
            },
            loss: Some(
              RollConsequence {
                condition: Condition::le(-1),
                tasks: vec![Discard::create(2)],
              }
            )
        },
        requirements: vec![
          MonsterRequirements::HeroType(HeroType::Bard),
          MonsterRequirements::Hero,
        ],
    },
      Monster::AbyssQueen => MonsterSpec {
        consequences: RollConsequences {
            success: RollConsequence {
                condition: Condition::ge(8),
                tasks: vec![],
            },
            loss: Some(
              RollConsequence {
                condition: Condition::le(5),
                tasks: vec![Sacrifice::create(1)],
              }
            )
        },
        requirements: vec![MonsterRequirements::Hero; 2],
    },
      Monster::RexMajor => MonsterSpec {
        consequences: RollConsequences {
            success: RollConsequence {
                condition: Condition::ge(8),
                tasks: vec![],
            },
            loss: Some(
              RollConsequence {
                condition: Condition::le(4),
                tasks: vec![Discard::create(2)],
              }
            )
        },
        requirements: vec![
          MonsterRequirements::HeroType(HeroType::Gaurdian),
          MonsterRequirements::Hero,
        ],
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
            loss: Some(
              RollConsequence {
                condition: Condition::le(6),
                tasks: vec![Sacrifice::create(1)],
              }
            )
        },
        requirements: vec![MonsterRequirements::Hero; 3],
    },
      Monster::CrownedSerpent => MonsterSpec {
        consequences: RollConsequences {
            success: RollConsequence {
                condition: Condition::ge(10),
                tasks: vec![],
            },
            loss: Some(
              RollConsequence {
                condition: Condition::le(7),
                tasks: vec![Sacrifice::create(1)],
              }
            )
        },
        requirements: vec![MonsterRequirements::Hero; 2],
    },
        Monster::WarwornOwlbear => MonsterSpec {
          consequences: RollConsequences {
              success: RollConsequence {
                  condition: Condition::ge(8),
                  tasks: vec![],
              },
              loss: Some(
                RollConsequence {
                  condition: Condition::le(4),
                  tasks: vec![Discard::create(2)],
                }
              )
          },
          requirements: vec![
            MonsterRequirements::HeroType(HeroType::Thief),
            MonsterRequirements::Hero,
          ],
      },
        Monster::Dracos => MonsterSpec {
          consequences: RollConsequences {
              success: RollConsequence {
                  condition: Condition::le(5),
                  tasks: vec![],
              },
              loss: Some(
                RollConsequence {
                  condition: Condition::ge(8),
                  tasks: vec![Sacrifice::create(1)],
                }
              )
          },
          requirements: vec![MonsterRequirements::Hero],
      },
        Monster::Malammoth => MonsterSpec {
          consequences: RollConsequences {
              success: RollConsequence {
                  condition: Condition::ge(8),
                  tasks: vec![],
              },
              loss: Some(
                RollConsequence {
                  condition: Condition::le(4),
                  tasks: vec![Discard::create(2)],
                }
              )
          },
          requirements: vec![
            MonsterRequirements::Hero,
            MonsterRequirements::HeroType(HeroType::Ranger)
          ],
      },
      Monster::Bloodwing => MonsterSpec {
        consequences: RollConsequences {
            success: RollConsequence {
                condition: Condition::ge(9),
                tasks: vec![],
            },
            loss: Some(
              RollConsequence {
                condition: Condition::le(6),
                tasks: vec![Sacrifice::create(1)],
              }
            )
        },
        requirements: vec![MonsterRequirements::Hero; 2],
    },
        Monster::ArcticAries => MonsterSpec {
          consequences: RollConsequences {
              success: RollConsequence {
                  condition: Condition::ge(10),
                  tasks: vec![],
              },
              loss: Some(
                RollConsequence {
                  condition: Condition::le(6),
                  tasks: vec![Sacrifice::create(1)],
                }
              )
          },
          requirements: vec![MonsterRequirements::Hero],
      },
        Monster::MegaSlime => MonsterSpec {
          consequences: RollConsequences {
            success: RollConsequence {
              condition: Condition::ge(8),
              tasks: vec![
                ReceiveModifier::create(PlayerModifier::ExtraActionPoint, ModifierOrigin::FromSlainMonster),
                DrawTask::create(2),
              ]
            },
            loss: Some(
              RollConsequence {
                condition: Condition::le(7),
                tasks: vec![Sacrifice::create(2)],
              }
            )
          },
          requirements: vec![MonsterRequirements::Hero; 4],
      },
      Monster::Orthus =>  MonsterSpec {
          consequences: RollConsequences {
            success: RollConsequence {
              condition: Condition::ge(8),
              tasks: vec![ReceiveModifier::create(PlayerModifier::PlayMagicOnDraw, ModifierOrigin::FromSlainMonster),]
            },
            loss: Some(
              RollConsequence {
                condition: Condition::le(4),
                tasks: vec![Sacrifice::create(1)],
              }
            )
          },
        requirements: vec![
            MonsterRequirements::Hero,
            MonsterRequirements::HeroType(HeroType::Wizard),
        ],
    },
    Monster::Terratuga => MonsterSpec {
          consequences: RollConsequences {
            success: RollConsequence {
              condition: Condition::ge(11),
              tasks: vec![ReceiveModifier::create(PlayerModifier::UndestroyableHeros, ModifierOrigin::FromSlainMonster)]
            },
            loss: Some(
              RollConsequence {
                condition: Condition::le(7),
                tasks: vec![Discard::create(2)],
              }
            )
          },
          requirements: vec![MonsterRequirements::Hero],
      },
    }
  }

pub(crate) fn label(&self) -> String {
    todo!()
}

pub(crate) fn description(&self) -> String {
    todo!()
}

pub(crate) fn image_path(&self) -> String {
    todo!()
}

}
