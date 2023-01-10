use std::collections::HashSet;
use std::vec;

use super::errors::SlayResult;
use super::game_context::GameBookKeeping;
use super::hero_abilities::HeroAbilityType;
use super::ids;
use super::modifiers::ItemModifier;
use super::specs::cards::SlayCardSpec;
use super::specs::magic::MagicSpell;
use super::specs::monster::Monster;
use super::state::game::Game;
use super::tasks::TaskProgressResult;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::state::deck::DeckPath;
use crate::slay::tasks::PlayerTask;

/*


(||{
	#[derive(Clone, Debug)]
	struct Anonymous {}
	impl PlayerTask for Anonymous {
		fn make_progress(
			 &mut self, _context: &mut GameBookKeeping, _game: &mut Game, _player_index: ids::PlayerIndex,
		) -> SlayResult<TaskProgressResult> {
			/////////////////

			/////////////////
			Ok(TaskProgressResult::TaskComplete)
		}
		fn label(&self) -> String {
			"anonymous task".to_owned()
		}
	}
	Box::new(Anonymous {}) as Box<dyn PlayerTask>
})(),


*/

pub const MAX_TURNS: u32 = 1000;


#[derive(Debug, Clone)]
pub struct HeroAbility {
	pub condition: Condition,
	pub ability: HeroAbilityType,
}

impl HeroAbility {
	pub fn to_consequences(&self) -> RollConsequences {
		RollConsequences {
			success: RollConsequence {
				condition: self.condition.to_owned(),
				tasks: self.ability.create_tasks(),
			},
			loss: None,
		}
	}
}


// type ActionsCreator = Box<dyn Fn(ids::PlayerIndex) -> Vec<Box<TasksChoice>>>;

// Rename this to generation spec...
#[derive(Debug, Clone)]
pub struct CardSpec {
	card_type: CardType,
	pub repeat: u32,
	pub label: String,
	pub description: String,
	pub image_path: String,

	// challenge tasks...
	pub monster: Option<Monster>,
	pub modifiers: Vec<i32>,
	pub hero_ability: Option<HeroAbility>,
	pub spell: Option<MagicSpell>,
	pub card_modifier: Option<ItemModifier>,


  pub real_spec: SlayCardSpec,
	// pub hand_actions: ActionsCreator,
	// pub party_actions: ActionsCreator,
}

impl CardSpec {
  pub fn get_initial_deck(&self) -> DeckPath {
    match self.card_type {
        CardType::PartyLeader(_) => DeckPath::PartyLeaders,
        CardType::Monster => DeckPath::NextMonsters,
        _ => DeckPath::Draw,
    }
  }
  pub fn get_unmodified_hero_type(&self) -> Option<HeroType> {
    match &self.card_type {
        CardType::Hero(hero_type) => Some(*hero_type),
        CardType::PartyLeader(hero_type) => Some(*hero_type),
        _ => None,
    }
  }

  pub(crate) fn is_magic(&self) -> bool {
    match &self.card_type {
      CardType::Magic => true,
      _ => false,
    }
  }

  pub(crate) fn is_hero(&self) -> bool {
    match &self.card_type {
      CardType::Hero(_) => true,
      _ => false,
    }
  }

pub(crate) fn is_challenge(&self) -> bool {
  match &self.card_type {
    CardType::Challenge => true,
    _ => false,
  }
}
}

impl Default for CardSpec {
	fn default() -> Self {
		Self {
			card_type: CardType::Challenge,
			repeat: 1,
			label: "Please set the name.".to_string(),
			description: "Please set the description.".to_string(),
			image_path: "Please set the image_path.".to_string(),
			monster: None,
			modifiers: Vec::new(),
			hero_ability: None,
			spell: None,
			card_modifier: None,

      real_spec: SlayCardSpec::Challenge,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardType {
	Hero(HeroType),
	PartyLeader(HeroType),
	Monster,
	Challenge,
	Modifier,
	Item(ItemType),
	Magic,
}

impl CardType {
	pub fn is_hero_card(&self) -> bool {
		// Supposed to be a matches!()...
		match self {
			CardType::Hero(_) => true,
			_ => false,
		}
	}

	pub fn item_types() -> HashSet<CardType> {
		HashSet::from([
			CardType::Item(ItemType::Blessed),
			CardType::Item(ItemType::Cursed),
		])
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum HeroType {
	Bard,
	Wizard,
	Fighter,
	Gaurdian,
	Ranger,
	Thief,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemType {
	Cursed,
	Blessed,
	Mask, // Could have a hero type here as well...
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonsterRequirements {
	Hero,
	HeroType(HeroType),
}

#[derive(Debug, Clone)]
struct MonsterSlainTask {
	card_id: ids::CardId,
}

impl PlayerTask for MonsterSlainTask {
	fn make_progress(
		&mut self, _context: &mut GameBookKeeping, game: &mut Game, player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.move_card(
			DeckPath::ActiveMonsters,
			DeckPath::SlainMonsters(player_index),
			self.card_id,
		)?;

		if let Some(stack) = game.deck_mut(DeckPath::NextMonsters).maybe_deal() {
			game.deck_mut(DeckPath::ActiveMonsters).add(stack);
		}
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Slay monster card {}.", self.card_id)
	}
}

// #[derive(Debug, Clone)]
// pub enum RollConsequenceType {
//     MonsterSlain(ids::CardId),
//     DefeatedByMonster(ids::CardId),
// }

#[derive(Debug, Clone)]
pub struct MonsterSpec {
	pub consequences: RollConsequences,
	pub requirements: Vec<MonsterRequirements>,
}

impl MonsterSpec {
	pub fn get_consequences(&self, card_id: ids::CardId) -> RollConsequences {
		let mut tasks = self.consequences.success.tasks.clone();
		tasks.push(Box::new(MonsterSlainTask { card_id }));
		RollConsequences {
			success: RollConsequence {
				condition: self.consequences.success.condition.to_owned(),
				tasks,
			},
			loss: self.consequences.loss.to_owned(),
		}
	}
}

// const spec: MonsterSpec = MonsterSpec {
//   win_condition: Condition { ge: true, threshold: 11 },
//   loss_condition: Condition { ge: false, threshold: 7 },
//   player_modifiers: vec![UndestroyableHeros],
//   loss_consequence: Sacrifice(1),
// }

pub fn get_card_specs() -> [CardSpec; 95] {
	[
    ////////////////////////////////////////////////////////////////////////////
    // Challenge
    ////////////////////////////////////////////////////////////////////////////
    CardSpec {
        card_type: CardType::Challenge,
        repeat: 8,
        label: "Challenge".to_string(),
        image_path: "cards/challenge/challenge.jpg".to_string(),
        description: "You may play this card when another player attempts to play a Hero, Item, or Magic card. CHALLENGE that card.".to_string(),
        ..Default::default()
    },
    ////////////////////////////////////////////////////////////////////////////
    // Modifiers
    ////////////////////////////////////////////////////////////////////////////
    CardSpec {
        card_type: CardType::Modifier,
        label: "Modifier +4".to_string(),
        repeat: 4,
        image_path: "cards/modifier/4.jpg".to_string(),
        description: "Play this card after any player (including you) rolls the dice. +4 to that roll.".to_string(),
        modifiers: vec![4],
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Modifier,
        label: "Modifier +3/-1".to_string(),
        repeat: 4,
        image_path: "cards/modifier/3.jpg".to_string(),
        description: "Play this card after any player (including you) rolls the dice. +3 or -1 to that roll.".to_string(),
        modifiers: vec![3, -1],
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Modifier,
        label: "modifier +2/-2".to_string(),
        repeat: 9,
        image_path: "cards/modifier/2.jpg".to_string(),
        description: "Play this card after any player (including you) rolls the dice. +2 or -2 to that roll.".to_string(),
        modifiers: vec![2, -2],
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Modifier,
        label: "modifier +1/-3".to_string(),
        repeat: 4,
        image_path: "cards/modifier/1.jpg".to_string(),
        description: "Play this card after any player (including you) rolls the dice. +1 or -3 to that roll.".to_string(),
        modifiers: vec![1, -3],
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Modifier,
        label: "modifier -4".to_string(),
        repeat: 4,
        image_path: "cards/modifier/0.jpg".to_string(),
        description: "Play this card after any player (including you) rolls the dice. -4 to that roll.".to_string(),
        modifiers: vec![-4],
        ..Default::default()
    },
    ////////////////////////////////////////////////////////////////////////////
    // Heros
    ////////////////////////////////////////////////////////////////////////////
    CardSpec {
        card_type: CardType::Hero(HeroType::Thief),
        label: "Plundering Puma".to_string(),
        image_path: "cards/heros/thief/plundering_puma.jpg".to_string(),
        description: "Pull 2 cards from another player's hand. That player may DRAW a card.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(6),
            ability: HeroAbilityType::PlunderingPuma,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Thief),
        label: "Slippery Paws".to_string(),
        image_path: "cards/heros/thief/slippery_paws.jpg".to_string(),
        description: "Pull 2 cards from another player's hand, then DISCARD one of those cards.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(6),
            ability: HeroAbilityType::SlipperyPaws,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Thief),
        label: "Smooth Mimimeow".to_string(),
        image_path: "cards/heros/thief/smooth_mimimeow.jpg".to_string(),
        description: "Pull a card from the hand of each other player with a Thief in their Party.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(7),
            ability: HeroAbilityType::SmoothMimimeow,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Thief),
        label: "Meowzio".to_string(),
        image_path: "cards/heros/thief/meowzio.jpg".to_string(),
        description: "Choose a player. STEAL a Hero card from that player's Party and pull a card from that player's hand.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(10),
            ability: HeroAbilityType::Meowzio,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Thief),
        label: "Shurikitty".to_string(),
        image_path: "cards/heros/thief/shurikitty.jpg".to_string(),
        description: "DESTROY a Hero card. If that Hero card had an item card equipped to it, add that Item card to your hand instead of moving it to the discard pile.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(9),
            ability: HeroAbilityType::Shurikitty,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Thief),
        label: "Kit Napper".to_string(),
        image_path: "cards/heros/thief/kit_napper.jpg".to_string(),
        description: "Steal a Hero card.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(9),
            ability: HeroAbilityType::KitNapper,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Thief),
        label: "Silent Shadow".to_string(),
        image_path: "cards/heros/thief/silent_shadow.jpg".to_string(),
        description: "Look at another player's hand. Choose a card and add it to your hand.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(8),
            ability: HeroAbilityType::SilentShadow,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Thief),
        label: "Sly Pickings".to_string(),
        image_path: "cards/heros/thief/sly_pickings.jpg".to_string(),
        description: "Pull a card from another player's hand. If that card is an Item card, you may play it immediately.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(6),
            ability: HeroAbilityType::SlyPickings,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Gaurdian),
        label: "Holy Curselifter".to_string(),
        image_path: "cards/heros/guardian/holy_curse_lifter.jpg".to_string(),
        description: "Return a Cursed Item card equipped to a Hero card in your Party to your hand.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(5),
            ability: HeroAbilityType::HolyCurselifter,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Gaurdian),
        label: "Iron Resolve".to_string(),
        image_path: "cards/heros/guardian/iron_resolve.jpg".to_string(),
        description: "Cards you play cannot be challenged for the rest of your turn.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(8),
            ability: HeroAbilityType::IronResolve,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Gaurdian),
        label: "Calming Voice".to_string(),
        image_path: "cards/heros/guardian/calming_voice.jpg".to_string(),
        description: "Hero cards in your Party cannot be stolen until your next turn.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(9),
            ability: HeroAbilityType::CalmingVoice,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Gaurdian),
        label: "Vibrant Glow".to_string(),
        image_path: "cards/heros/guardian/vibrant_glow.jpg".to_string(),
        description: "+5 to all of your rolls until the end of your turn.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(9),
            ability: HeroAbilityType::VibrantGlow,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Gaurdian),
        label: "Mighty Blade".to_string(),
        image_path: "cards/heros/guardian/mighty_blade.jpg".to_string(),
        description: "Hero cards in your Party cannot be destroyed until your next turn.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(8),
            ability: HeroAbilityType::MightyBlade,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Gaurdian),
        label: "Radiant Horn".to_string(),
        image_path: "cards/heros/guardian/radiant_horn.jpg".to_string(),
        description: "Search the discard pile for a Modifier card and add it to your hand.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(6),
            ability: HeroAbilityType::RadiantHorn,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Gaurdian),
        label: "Guiding Light".to_string(),
        image_path: "cards/heros/guardian/guiding_light.jpg".to_string(),
        description: "Search the discard pile for a Hero card and add it to your hand.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(7),
            ability: HeroAbilityType::GuidingLight,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Gaurdian),
        label: "Wise Shield".to_string(),
        image_path: "cards/heros/guardian/wise_shield.jpg".to_string(),
        description: "+3 to all of your rolls until the end of your turn.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(6),
            ability: HeroAbilityType::WiseShield,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Fighter),
        label: "Qi Bear".to_string(),
        image_path: "cards/heros/fighter/chi_bear.jpg".to_string(),
        description: "DISCARD up to 3 cards. For each card discarded, DESTROY a Hero card.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(10),
            ability: HeroAbilityType::QiBear,
          }
        ),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Hero(HeroType::Fighter),
        label: "Pan Chucks".to_string(),
        image_path: "cards/heros/fighter/pan_chucks.jpg".to_string(),
        description: "DRAW 2 cards. If at least one of those cards is a Challenge card, you may reveal it, then DESTROY a Hero card.".to_string(),
        hero_ability: Some(
          HeroAbility {
            condition: Condition::ge(8),
            ability: HeroAbilityType::PanChucks,
          }
        ),
        ..Default::default()
    },
    CardSpec {
      card_type: CardType::Hero(HeroType::Fighter),
      label: "Heavy Bear".to_string(),
      image_path: "cards/heros/fighter/heavy_bear.jpg".to_string(),
      description: "Choose a player. That player must DISCARD 2 cards.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(5),
          ability: HeroAbilityType::HeavyBear,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Fighter),
      label: "Bad Axe".to_string(),
      image_path: "cards/heros/fighter/bad_axe.jpg".to_string(),
      description: "DESTROY a Hero card.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(8),
          ability: HeroAbilityType::BadAxe,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Fighter),
      label: "Tough Teddy".to_string(),
      image_path: "cards/heros/fighter/tough_teddy.jpg".to_string(),
      description: "Each other player with a Fighter in their Party must DISCARD a card.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(4),
          ability: HeroAbilityType::ToughTeddy,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Fighter),
      label: "Bear Claw".to_string(),
      image_path: "cards/heros/fighter/bear_claw.jpg".to_string(),
      description: "Pull a card from another player's hand. If it is a Hero card, pull a second card from that player's hand.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(7),
          ability: HeroAbilityType::BearClaw,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Fighter),
      label: "Fury Knuckle".to_string(),
      image_path: "cards/heros/fighter/fury_knuckle.jpg".to_string(),
      description: "Pull a card from another player's hand. If it is a Challenge card, pull a second card from that player's hand.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(5),
          ability: HeroAbilityType::FuryKnuckle,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Fighter),
      label: "Beary Wise".to_string(),
      image_path: "cards/heros/fighter/beary_wise.jpg".to_string(),
      description: "Each other player must DISCARD a card. Choose one of the discarded cards and add it to your hand.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(7),
          ability: HeroAbilityType::BearyWise,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Ranger),
      label: "Hook".to_string(),
      image_path: "cards/heros/ranger/hook.jpg".to_string(),
      description: "Play an Item card from your hand immediately and DRAW a card.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(6),
          ability: HeroAbilityType::Hook,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Ranger),
      label: "Wildshot".to_string(),
      image_path: "cards/heros/ranger/wildshot.jpg".to_string(),
      description: "DRAW 3 cards and DISCARD a card.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(8),
          ability: HeroAbilityType::Wildshot,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Ranger),
      label: "Serious Grey".to_string(),
      image_path: "cards/heros/ranger/serious_grey.jpg".to_string(),
      description: "DESTROY a Hero card and DRAW a card.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(9),
          ability: HeroAbilityType::SeriousGrey,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Ranger),
      label: "Wily Red".to_string(),
      image_path: "cards/heros/ranger/wily_red.jpg".to_string(),
      description: "DRAW cards until you have 7 cards in your hand.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(10),
          ability: HeroAbilityType::WilyRed,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Ranger),
      label: "Quick Draw".to_string(),
      image_path: "cards/heros/ranger/quick_draw.jpg".to_string(),
      description: "DRAW 2 cards. If at least one of those cards is an item card, you may play one of them immediately.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(8),
          ability: HeroAbilityType::QuickDraw,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Ranger),
      label: "Lookie Rookie".to_string(),
      image_path: "cards/heros/ranger/lookie_rookie.jpg".to_string(),
      description: "Search the discard pile for an Item card and add it to your hand.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(5),
          ability: HeroAbilityType::LookieRookie,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Ranger),
      label: "Bullseye".to_string(),
      image_path: "cards/heros/ranger/bullseye.jpg".to_string(),
      description: "Look at the top 3 cards of the deck. Add one to your hand, then return the other two to the top of the deck in any order.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(7),
          ability: HeroAbilityType::Bullseye,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Ranger),
      label: "Sharp Fox".to_string(),
      image_path: "cards/heros/ranger/sharp_fox.jpg".to_string(),
      description: "Look at another player's hand.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(5),
          ability: HeroAbilityType::SharpFox,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Bard),
      label: "Fuzzy Cheeks".to_string(),
      image_path: "cards/heros/bards/fuzzy_cheeks.jpg".to_string(),
      description: "DRAW a card and play a Hero card from you hand immediately.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(8),
          ability: HeroAbilityType::FuzzyCheeks,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Bard),
      label: "Peanut".to_string(),
      image_path: "cards/heros/bards/peanut.jpg".to_string(),
      description: "DRAW 2 cards.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(7),
          ability: HeroAbilityType::Peanut,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Bard),
      label: "Napping Nibbles".to_string(),
      image_path: "cards/heros/bards/napping_nibbles.jpg".to_string(),
      description: "Do nothing.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(2),
          ability: HeroAbilityType::NappingNibbles,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Bard),
      label: "Tipsy Tootie".to_string(),
      image_path: "cards/heros/bards/tipsy_tootie.jpg".to_string(),
      description: "Choose a player. STEAL a Hero card from that player's Party and move this card to that player's Party.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(6),
          ability: HeroAbilityType::TipsyTootie,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Bard),
      label: "Mellow Dee".to_string(),
      image_path: "cards/heros/bards/mellow_dee.jpg".to_string(),
      description: "DRAW a card. If that card is a Hero card, you may play it immediately.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(7),
          ability: HeroAbilityType::MellowDee,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Bard),
      label: "Luck Bucky".to_string(),
      image_path: "cards/heros/bards/lucky_bucky.jpg".to_string(),
      description: "Pull a card from another player's hand. If that card is a Hero card, you may play it immediately.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(7),
          ability: HeroAbilityType::LuckBucky,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Bard),
      label: "Dodgy Dealer".to_string(),
      image_path: "cards/heros/bards/dodgy_dealer.jpg".to_string(),
      description: "Trade hands with another player.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(9),
          ability: HeroAbilityType::DodgyDealer,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Bard),
      label: "Greedy Cheeks".to_string(),
      image_path: "cards/heros/bards/greedy_cheeks.jpg".to_string(),
      description: "Each other player must give you a card from their hand.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(8),
          ability: HeroAbilityType::GreedyCheeks,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Wizard),
      label: "Fluffy".to_string(),
      image_path: "cards/heros/wizard/fluffy.jpg".to_string(),
      description: "DESTROY 2 Hero cards.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(10),
          ability: HeroAbilityType::Fluffy,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Wizard),
      label: "Wiggles".to_string(),
      image_path: "cards/heros/wizard/wiggles.jpg".to_string(),
      description: "STEAL a Hero card and roll to use its effect immediately.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(10),
          ability: HeroAbilityType::Wiggles,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Wizard),
      label: "Spooky".to_string(),
      image_path: "cards/heros/wizard/spooky.jpg".to_string(),
      description: "Each other player must SACRIFICE a Hero card.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(10),
          ability: HeroAbilityType::Spooky,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Wizard),
      label: "Snowball".to_string(),
      image_path: "cards/heros/wizard/snowball.jpg".to_string(),
      description: "DRAW a card. If it is a Magic card, you may play it immediately and DRAW a second card.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(6),
          ability: HeroAbilityType::Snowball,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Wizard),
      label: "Buttons".to_string(),
      image_path: "cards/heros/wizard/buttons.jpg".to_string(),
      description: "Pull a card from another player's hand. If it is a Magic card, you may play it immediately.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(6),
          ability: HeroAbilityType::Buttons,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Wizard),
      label: "Bun Bun".to_string(),
      image_path: "cards/heros/wizard/bun_bun.jpg".to_string(),
      description: "Search the discard pile for a Magic card and add it to your hand.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(5),
          ability: HeroAbilityType::BunBun,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Wizard),
      label: "Hopper".to_string(),
      image_path: "cards/heros/wizard/hopper.jpg".to_string(),
      description: "Choose a player. That player must SACRIFICE a Hero card.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(7),
          ability: HeroAbilityType::Hopper,
        }
      ),
      ..Default::default()
  },
    CardSpec {
      card_type: CardType::Hero(HeroType::Wizard),
      label: "Whiskers".to_string(),
      image_path: "cards/heros/wizard/whiskers.jpg".to_string(),
      description: "STEAL a Hero card and DESTROY a Hero card.".to_string(),
      hero_ability: Some(
        HeroAbility {
          condition: Condition::ge(11),
          ability: HeroAbilityType::Whiskers,
        }
      ),
      ..Default::default()
  },
    ////////////////////////////////////////////////////////////////////////////
    // Magic
    ////////////////////////////////////////////////////////////////////////////
    CardSpec {
      card_type: CardType::Magic,
      label: "Entangling Trap".to_string(),
      image_path: "cards/magic/entangling_trap.jpg".to_string(),
      description: "DISCARD 2 cards, then STEAL a Hero card.".to_string(),
      spell: Some(MagicSpell::EnganglingTrap),
      repeat: 2,
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Magic,
      label: "Critical Boost".to_string(),
      image_path: "cards/magic/critical_boost.jpg".to_string(),
      description: "DRAW 3 cards and DISCARD a card.".to_string(),
      spell: Some(MagicSpell::CriticalBoost),
      repeat: 2,
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Magic,
      label: "Destructive Spell".to_string(),
      image_path: "cards/magic/descructive_spell.jpg".to_string(),
      description: "DISCARD a card, then DESTROY a Hero card.".to_string(),
      spell: Some(MagicSpell::DestructiveSpell),
      repeat: 2,
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Magic,
      label: "Winds of Change".to_string(),
      image_path: "cards/magic/winds_of_change.jpg".to_string(),
      description: "Return an Item card equipped to any player's Hero card to that player's hand, then DRAW a card.".to_string(),
      spell: Some(MagicSpell::WindsOfChange),
      repeat: 2,
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Magic,
      label: "Enchanted Spell".to_string(),
      image_path: "cards/magic/enchanged_spell.jpg".to_string(),
      description: "+2 to all of your rolls until the end of your turn".to_string(),
      spell: Some(MagicSpell::EnchangedSpell),
      repeat: 2,
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Magic,
      label: "Forced Exchange".to_string(),
      image_path: "cards/magic/forced_exchange.jpg".to_string(),
      description: "Choose a player. STEAL a Hero card from that player's Party, then move a Hero card from your Party to that player's Party.".to_string(),
      spell: Some(MagicSpell::ForcedExchange),
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Magic,
      label: "Forceful Winds".to_string(),
      image_path: "cards/magic/forceful_winds.jpg".to_string(),
      description: "Return every equipped item card to its respective player's hand".to_string(),
      spell: Some(MagicSpell::ForcefulWinds),
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Magic,
      label: "Call to the Fallen".to_string(),
      image_path: "cards/magic/call_to_the_fallen.jpg".to_string(),
      description: "Search the discard pile for a Hero card and add it to your hand.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      ..Default::default()
    },


    ////////////////////////////////////////////////////////////////////////////
    // Items
    ////////////////////////////////////////////////////////////////////////////
    CardSpec {
      card_type: CardType::Item(ItemType::Mask),
      label: "Bard Mask".to_string(),
      image_path: "cards/items/bard_mask.jpg".to_string(),
      description: "The equipped Hero card is considered a Bard instead of its original class.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::Mask(HeroType::Bard)),
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Item(ItemType::Mask),
      label: "Ranger Mask".to_string(),
      image_path: "cards/items/ranger_mask.jpg".to_string(),
      description: "The equipped Hero card is considered a Ranger instead of its original class.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::Mask(HeroType::Ranger)),
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Item(ItemType::Mask),
      label: "Fighter Mask".to_string(),
      image_path: "cards/items/fighter_mask.jpg".to_string(),
      description: "The equipped Hero card is considered a Fighter instead of its original class.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::Mask(HeroType::Fighter)),
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Item(ItemType::Mask),
      label: "Thief Mask".to_string(),
      image_path: "cards/items/thief_mask.jpg".to_string(),
      description: "The equipped Hero card is considered a Thief instead of its original class.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::Mask(HeroType::Thief)),
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Item(ItemType::Mask),
      label: "Guardian Mask".to_string(),
      image_path: "cards/items/guardian_mask.jpg".to_string(),
      description: "The equipped Hero card is considered a Guardian instead of its original class.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::Mask(HeroType::Gaurdian)),
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Item(ItemType::Mask),
      label: "Wizard Mask".to_string(),
      image_path: "cards/items/wizard_mask.jpg".to_string(),
      description: "The equipped Hero card is considered a Wizard instead of its original class.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::Mask(HeroType::Wizard)),
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Item(ItemType::Blessed),
      label: "Decoy Doll".to_string(),
      image_path: "cards/items/decoy_doll.jpg".to_string(),
      description: "If the equipped Hero card would be sacrificed or destroyed, move Decoy Doll to the discard pile instead.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::SacrificeMeInstead),
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Item(ItemType::Blessed),
      label: "Really Big Ring".to_string(),
      image_path: "cards/items/really_big_ring.jpg".to_string(),
      description: "Each time you roll to use the equipped Hero card's effect, +2 to your roll.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::AddToRollForAbility(2)),
      repeat: 2,
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Item(ItemType::Blessed),
      label: "Particularly Rusty Coin".to_string(),
      image_path: "cards/items/particularly_rusty_coin.jpg".to_string(),
      description: "If you unsuccessfully roll to use the equipped Hero card's effect, DRAW a card.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::DrawOnUnsuccessfulRollForAbility(1)),
      repeat: 2,
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Item(ItemType::Cursed),
      label: "Sealing Key".to_string(),
      image_path: "cards/cursed_items/sealing_key.jpg".to_string(),
      description: "You cannot use the equipped Hero card's effect.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::RemoveAbility),
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Item(ItemType::Cursed),
      label: "Suspiciously Shiny Coin".to_string(),
      image_path: "cards/cursed_items/suspiciously_shiny_coin.jpg".to_string(),
      description: "If you sucessfully roll to use the equipped Hero card's effect, DISCARD a card.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::DiscardOnSuccessfulRollForAbility(1)),
      ..Default::default()
    },
    CardSpec {
      card_type: CardType::Item(ItemType::Cursed),
      label: "Curse of the Snake's Eyes".to_string(),
      image_path: "cards/cursed_items/curse_of_the_snakes_eyes.jpg".to_string(),
      description: "Each time you roll to use the equipped Hero card's effect, -2 to your roll.".to_string(),
      spell: Some(MagicSpell::CallToTheFallen),
      card_modifier: Some(ItemModifier::AddToRollForAbility(-2)),
      repeat: 2,
      ..Default::default()
    },
    ////////////////////////////////////////////////////////////////////////////
    // Leaders
    ////////////////////////////////////////////////////////////////////////////
    CardSpec {
        card_type: CardType::PartyLeader(HeroType::Fighter),
        label: "The Fist of Reason".to_string(),
        image_path: "cards/party_leaders/fist_of_reason.jpg".to_string(),
        description: "Each time you roll to CHALLENGE, +2 to your roll.".to_string(),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::PartyLeader(HeroType::Wizard),
        label: "The Cloaked Sage".to_string(),
        image_path: "cards/party_leaders/cloaked_sage.jpg".to_string(),
        description: "Each time you play a Magic card, DRAW a card.".to_string(),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::PartyLeader(HeroType::Gaurdian),
        label: "The Protecting Horn".to_string(),
        image_path: "cards/party_leaders/protecting_horn.jpg".to_string(),
        description: "Each time you play a Modifier card on a roll, +1 or -1 to that roll.".to_string(),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::PartyLeader(HeroType::Thief),
        label: "The Shadow Claw".to_string(),
        image_path: "cards/party_leaders/shadow_claw.jpg".to_string(),
        description: "Once per turn on your turn, you may spend an action point to pull a card from another player's hand.".to_string(),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::PartyLeader(HeroType::Bard),
        label: "The Charismatic Song".to_string(),
        image_path: "cards/party_leaders/charismatic_song.jpg".to_string(),
        description: "Each time you roll to use a Hero card's effect, +1 to your roll.".to_string(),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::PartyLeader(HeroType::Ranger),
        label: "The Charismatic Song".to_string(),
        image_path: "cards/party_leaders/divine_arrow.jpg".to_string(),
        description: "Each time you roll to ATTACK a Monster card, +1 to your roll.".to_string(),
        ..Default::default()
    },

    ////////////////////////////////////////////////////////////////////////////
    // Monsters
    ////////////////////////////////////////////////////////////////////////////
          CardSpec {
            card_type: CardType::Monster,
            label: "Anuran Cauldron".to_string(),
            image_path: "cards/monsters/anuran_cauldron.jpg".to_string(),
            description: "Each time you roll, +1 to your roll.".to_string(),
            monster: Some(Monster::AnuranCauldron),
            ..Default::default()
        },
          CardSpec {
            card_type: CardType::Monster,
            label: "Titan Wyvern".to_string(),
            image_path: "cards/monsters/titan_wyvern.jpg".to_string(),
            description: "Each time you roll for a Challenge card, +1 to your roll.".to_string(),
            monster: Some(Monster::TitanWyvern),
            ..Default::default()
        },
          CardSpec {
            card_type: CardType::Monster,
            label: "Dark Dragon King".to_string(),
            image_path: "cards/monsters/dark_dragon_king.jpg".to_string(),
            description: "Each time you roll for a Hero card's effect, +1 to your roll.".to_string(),
            monster: Some(Monster::DarkDragonKing),
            ..Default::default()
        },
          CardSpec {
            card_type: CardType::Monster,
            label: "Abyss Queen".to_string(),
            image_path: "cards/monsters/abyss_queen.jpg".to_string(),
            description: "Each time another player plays a Modifier card on one of your rolls, +1 to your roll.".to_string(),
            monster: Some(Monster::AbyssQueen),
            ..Default::default()
        },
          CardSpec {
            card_type: CardType::Monster,
            label: "Rex Major".to_string(),
            image_path: "cards/monsters/rex_major.jpg".to_string(),
            description: "Each time you DRAW a Modifier card, you may reveal it and DRAW a second card.".to_string(),
            monster: Some(Monster::RexMajor),
            ..Default::default()
        },
          CardSpec {
            card_type: CardType::Monster,
            label: "Corrupted Sabretooth".to_string(),
            image_path: "cards/monsters/corrupted_sabretooth.jpg".to_string(),
            description: "Each time you would DESTROY a Hero card, you may STEAL that Hero card instead.".to_string(),
            monster: Some(Monster::CorruptedSabretooth),
            ..Default::default()
        },
          CardSpec {
            card_type: CardType::Monster,
            label: "Crowned Serpent".to_string(),
            image_path: "cards/monsters/crowned_serpent.jpg".to_string(),
            description: "Each time any player (including you) plays a Modifier card, you may DRAW a card.".to_string(),
            monster: Some(Monster::CrownedSerpent),
            ..Default::default()
        },
          CardSpec {
            card_type: CardType::Monster,
            label: "Warworn Owlbear".to_string(),
            image_path: "cards/monsters/warworn_owlbear.jpg".to_string(),
            description: "Item cards you play cannot be challenged.".to_string(),
            monster: Some(Monster::WarwornOwlbear),
            ..Default::default()
        },
          CardSpec {
            card_type: CardType::Monster,
            label: "Dracos".to_string(),
            image_path: "cards/monsters/dracos.jpg".to_string(),
            description: "Each time a Hero card in your Party is destroyed, you may DRAW a card.".to_string(),
            monster: Some(Monster::Dracos),
            ..Default::default()
        },
          CardSpec {
            card_type: CardType::Monster,
            label: "Malammoth".to_string(),
            image_path: "cards/monsters/malamammoth.jpg".to_string(),
            description: "Each time you DRAW an Item card, you may play it immediately.".to_string(),
            monster: Some(Monster::Malammoth),
            ..Default::default()
        },
          CardSpec {
            card_type: CardType::Monster,
            label: "Bloodwing".to_string(),
            image_path: "cards/monsters/bloodwing.jpg".to_string(),
            description: "Each time another player CHALLENGES you, that player must DISCARD a card.".to_string(),
            monster: Some(Monster::Bloodwing),
            ..Default::default()
        },
    CardSpec {
      card_type: CardType::Monster,
      label: "Arctic Aries".to_string(),
      image_path: "cards/monsters/arctic_aries.jpg".to_string(),
      description: "Each time you successfully roll to use a Hero card's effect, you may DRAW a card.".to_string(),
      monster: Some(Monster::ArcticAries),
      ..Default::default()
  },
    CardSpec {
        card_type: CardType::Monster,
        label: "Mega Slime".to_string(),
        image_path: "cards/monsters/mega_slime.jpg".to_string(),
        description: "You may spend an extra action point on each of your turns.".to_string(),
        monster: Some(Monster::MegaSlime),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Monster,
        label: "Orthus".to_string(),
        image_path: "cards/monsters/orthus.jpg".to_string(),
        description: "Each time you DRAW a Magic card, you may play it immediately.".to_string(),
        monster: Some(Monster::Orthus),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Monster,
        label: "Terratuga".to_string(),
        image_path: "cards/monsters/terratuga.jpg".to_string(),
        description: "Your Hero cards cannot be destroyed.".to_string(),
        monster: Some(Monster::Terratuga),
        ..Default::default()
    },
]
}
