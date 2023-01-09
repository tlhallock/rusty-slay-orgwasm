use std::collections::HashSet;
use std::vec;

use super::abilities::destroy::DestroyCardTask;
use super::abilities::destroy::DestroyModifiersDestination;
use super::abilities::heros;
use super::abilities::immediate::OfferPlayImmediately;
use super::abilities::params::ChooseCardFromPlayerParameterTask;
use super::abilities::params::ClearParamsTask;
use super::abilities::steal;
use super::abilities::steal::StealTask;
use super::errors::SlayResult;
use super::game_context::GameBookKeeping;
use super::ids;
use super::modifiers::ItemModifier;
use super::modifiers::ModifierOrigin;
use super::state::game::Game;
use super::tasks::TaskProgressResult;
use crate::slay::abilities::discard::Discard;
use crate::slay::abilities::heros::VictimDraws;
use crate::slay::abilities::params::ChoosePlayerParameterTask;
use crate::slay::abilities::pull::PullFromTask;
use crate::slay::abilities::sacrifice::Sacrifice;
use crate::slay::actions::DrawTask;
use crate::slay::modifiers::PlayerModifier;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::state::deck::DeckPath;
use crate::slay::tasks::PlayerTask;
use crate::slay::tasks::ReceiveModifier;
use crate::slay::tasks::TaskParamName;
use crate::slay::visibility::VisibilitySpec;

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

// Move this to the decks file?
#[derive(Debug, Clone)]
pub struct DeckSpec {
	pub visibility: VisibilitySpec,
	pub path: DeckPath,
}

impl DeckSpec {
	pub fn get_label(&self) -> String {
		match self.path {
			DeckPath::Draw => "Draw pile".to_string(),
			DeckPath::Discard => "Discard pile".to_string(),
			DeckPath::PartyLeaders => "Unused Party leaders".to_string(),
			DeckPath::ActiveMonsters => "Monsters".to_string(),
			DeckPath::NextMonsters => "Next monsters".to_string(),
			DeckPath::Hand(player_index) => format!("Player {}'s hand", player_index),
			DeckPath::Party(player_index) => format!("Player {}'s party", player_index),
			DeckPath::SlainMonsters(player_index) => format!("Player {}'s monsters", player_index),
		}
	}
}

#[derive(Debug, Clone)]
pub struct HeroAbility {
	pub condition: Condition,
	pub tasks: Vec<Box<dyn PlayerTask>>,
}

impl HeroAbility {
	pub fn to_consequences(&self) -> RollConsequences {
		RollConsequences {
			success: RollConsequence {
				condition: self.condition.to_owned(),
				tasks: self.tasks.to_vec(),
			},
			loss: None,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum MagicSpell {
	EnganglingTrap,
	CriticalBoost,
	DestructiveSpell,
	WindsOfChange,
	EnchangedSpell,
	ForcedExchange,
	ForcefulWinds,
	CallToTheFallen,
}

// type ActionsCreator = Box<dyn Fn(ids::PlayerIndex) -> Vec<Box<TasksChoice>>>;

#[derive(Debug, Clone)]
pub struct CardSpec {
	pub card_type: CardType,
	pub repeat: u32,
	pub label: String,
	pub description: String,
	pub image_path: String,

	// challenge tasks...
	pub monster: Option<MonsterSpec>,
	pub modifiers: Vec<i32>,
	pub hero_ability: Option<HeroAbility>,
	pub spell: Option<MagicSpell>,
	pub item_modifier: Option<ItemModifier>,
	// pub hand_actions: ActionsCreator,
	// pub party_actions: ActionsCreator,
}

impl Default for CardSpec {
	fn default() -> Self {
		Self {
			card_type: CardType::Blank,
			repeat: 1,
			label: "Please set the name.".to_string(),
			description: "Please set the description.".to_string(),
			image_path: "Please set the image_path.".to_string(),
			monster: None,
			modifiers: Vec::new(),
			hero_ability: None,
			spell: None,
			item_modifier: None,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardType {
	Blank,

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

pub fn get_card_specs() -> [CardSpec; 41] {
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
            tasks: vec![
              ChoosePlayerParameterTask::exclude_self(
                TaskParamName::PlayerToPullFrom,
                "Choose a player to pull from.",
              ),
              PullFromTask::create(TaskParamName::PlayerToPullFrom),
              PullFromTask::create(TaskParamName::PlayerToPullFrom),
              VictimDraws::create(TaskParamName::PlayerToPullFrom),
              ClearParamsTask::create(),
            ],
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
            tasks: vec![
              ChoosePlayerParameterTask::exclude_self(
                TaskParamName::SlipperyPawsVictim,
                "Choose a player to pull 2 cards from, you will have to discard one of them.",
              ),
              PullFromTask::record_pulled(
                TaskParamName::SlipperyPawsVictim,
                Some(TaskParamName::SlipperyPawsVictimPulledCard1),
              ),
              PullFromTask::record_pulled(
                TaskParamName::SlipperyPawsVictim,
                Some(TaskParamName::SlipperyPawsVictimPulledCard2),
              ),
              heros::SlipperyPaws::create(),
              ClearParamsTask::create(),
            ],
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
            tasks: vec![
              heros::Mimimeow::create(),
              ClearParamsTask::create(),
            ],
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
            tasks: vec![
              ChoosePlayerParameterTask::exclude_self(
                TaskParamName::MeowzioVictim,
                "Choose a player to steal and pull from.",
              ),
              PullFromTask::create(TaskParamName::MeowzioVictim),
        			ChooseCardFromPlayerParameterTask::from_party(
                TaskParamName::MeowzioVictim,
                TaskParamName::MeowzioCard,
                "Which hero card would you like to steal?"
              ),
              steal::StealCardFromTask::create(TaskParamName::MeowzioVictim, TaskParamName::MeowzioCard),
              ClearParamsTask::create(),
            ],
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
            tasks: vec![
              ChoosePlayerParameterTask::exclude_self(
                TaskParamName::PlayerToDestroy,
                "to destroy a hero card (Shurikitty)",
              ),
              ChooseCardFromPlayerParameterTask::from_party(
                TaskParamName::PlayerToDestroy,
                TaskParamName::CardToSteal,
                "Which hero card would you like to destroy?"
              ),
              DestroyCardTask::create(
                TaskParamName::PlayerToDestroy,
                TaskParamName::CardToSteal,
                DestroyModifiersDestination::Myself,
              ),
              ClearParamsTask::create(),
            ],
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
            tasks: vec![
              StealTask::create(),
            ],
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
            tasks: vec![
              ChoosePlayerParameterTask::exclude_self(
                TaskParamName::SilentShadowVictim,
                "Who's hand do you want to see?",
              ),
              ChooseCardFromPlayerParameterTask::from_party(
                TaskParamName::SilentShadowVictim,
                TaskParamName::SilentShadowCard,
                "Which hero card would you like to take?"
              ),
              // TODO
              ClearParamsTask::create(),
            ],
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
            tasks: vec![
              ChoosePlayerParameterTask::exclude_self(
                TaskParamName::SlyPickinsVictim,
                "Sly Pickings: Who do you want to steal from?",
              ),
              PullFromTask::record_pulled(
                TaskParamName::SlyPickinsVictim,
                Some(TaskParamName::SlyPickinsCard),
              ),
              OfferPlayImmediately::create(
                TaskParamName::SlyPickinsCard,
                Some(CardType::item_types()),
              ),
              ClearParamsTask::create(),
            ],
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
            tasks: vec![

            ],
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
            tasks: vec![
            ],
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
            tasks: vec![
            ],
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
            tasks: vec![
            ],
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
            tasks: vec![
            ],
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
            tasks: vec![
            ],
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
            tasks: vec![
            ],
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
            tasks: vec![
            ],
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
            tasks: vec![
            ],
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
            tasks: vec![
            ],
          }
        ),
        ..Default::default()
    },

    /*
- label: "Heavy Bear"
  image: cards/heros/fighter/heavy_bear.jpg
  deck: draw
  description: "Choose a player. That player must DISCARD 2 cards."
  categories:
    - hero
  params:
    hero-type: fighter
    effect-roll: "5+"
- label: "Bad Axe"
  image: cards/heros/fighter/bad_axe.jpg
  deck: draw
  description: "DESTROY a Hero card."
  categories:
    - hero
  params:
    hero-type: fighter
    effect-roll: "8+"
- label: "Tough Teddy"
  image: cards/heros/fighter/tough_teddy.jpg
  deck: draw
  description: "Each other player with a Fighter in their Party must DISCARD a card."
  categories:
    - hero
  params:
    hero-type: fighter
    effect-roll: "4+"
- label: "Bear Claw"
  image: cards/heros/fighter/bear_claw.jpg
  deck: draw
  description: "Pull a card from another player's hand. If it is a Hero card, pull a second card from that player's hand."
  categories:
    - hero
  params:
    hero-type: fighter
    effect-roll: "7+"
- label: "Fury Knuckle"
  image: cards/heros/fighter/fury_knuckle.jpg
  deck: draw
  description: "Pull a card from another player's hand. If it is a Challenge card, pull a second card from that player's hand."
  categories:
    - hero
  params:
    hero-type: fighter
    effect-roll: "5+"
- label: "Beary Wise"
  image: cards/heros/fighter/beary_wise.jpg
  deck: draw
  description: "Each other player must DISCARD a card. Choose one of the discarded cards and add it to your hand."
  categories:
    - hero
  params:
    hero-type: fighter
    effect-roll: "7+"
- label: "Hook"
  image: cards/heros/ranger/hook.jpg
  deck: draw
  description: "Play an Item card from your hand immediately and DRAW a card."
  categories:
    - hero
  params:
    hero-type: ranger
    effect-roll: "6+"
- label: "Wildshot"
  image: cards/heros/ranger/wildshot.jpg
  deck: draw
  description: "DRAW 3 cards and DISCARD a card."
  categories:
    - hero
  params:
    hero-type: ranger
    effect-roll: "8+"
- label: "Serious Grey"
  image: cards/heros/ranger/serious_grey.jpg
  deck: draw
  description: "DESTROY a Hero card and DRAW a card."
  categories:
    - hero
  params:
    hero-type: ranger
    effect-roll: "9+"
- label: "Wily Red"
  image: cards/heros/ranger/wily_red.jpg
  deck: draw
  description: "DRAW cards until you have 7 cards in your hand."
  categories:
    - hero
  params:
    hero-type: ranger
    effect-roll: "10+"
- label: "Quick Draw"
  image: cards/heros/ranger/quick_draw.jpg
  deck: draw
  description: "DRAW 2 cards. If at least one of those cards is an item card, you may play one of them immediately."
  categories:
    - hero
  params:
    hero-type: ranger
    effect-roll: "8+"
- label: "Lookie Rookie"
  image: cards/heros/ranger/lookie_rookie.jpg
  deck: draw
  description: "Search the discard pile for an Item card and add it to your hand."
  categories:
    - hero
  params:
    hero-type: ranger
    effect-roll: "5+"
- label: "Bullseye"
  image: cards/heros/ranger/bullseye.jpg
  deck: draw
  description: "Look at the top 3 cards of the deck. Add one to your hand, then return the other two to the top of the deck in any order."
  categories:
    - hero
  params:
    hero-type: ranger
    effect-roll: "7+"
- label: "Sharp Fox"
  image: cards/heros/ranger/sharp_fox.jpg
  deck: draw
  description: "Look at another player's hand."
  categories:
    - hero
  params:
    hero-type: ranger
    effect-roll: "5+"
- label: "Fuzzy Cheeks"
  image: cards/heros/bards/fuzzy_cheeks.jpg
  deck: draw
  description: "DRAW a card and play a Hero card from you hand immediately."
  categories:
    - hero
  params:
    hero-type: bard
    effect-roll: "8+"
- label: "Peanut"
  image: cards/heros/bards/peanut.jpg
  deck: draw
  description: "DRAW 2 cards."
  categories:
    - hero
  params:
    hero-type: bard
    effect-roll: "7+"
- label: "Napping Nibbles"
  image: cards/heros/bards/napping_nibbles.jpg
  deck: draw
  description: "Do nothing."
  categories:
    - hero
  params:
    hero-type: bard
    effect-roll: "2+"
- label: "Tipsy Tootie"
  image: cards/heros/bards/tipsy_tootie.jpg
  deck: draw
  description: "Choose a player. STEAL a Hero card from that player's Party and move this card to that player's Party."
  categories:
    - hero
  params:
    hero-type: bard
    effect-roll: "6+"
- label: "Mellow Dee"
  image: cards/heros/bards/mellow_dee.jpg
  deck: draw
  description: "DRAW a card. If that card is a Hero card, you may play it immediately."
  categories:
    - hero
  params:
    hero-type: bard
    effect-roll: "7+"
- label: "Luck Bucky"
  image: cards/heros/bards/lucky_bucky.jpg
  deck: draw
  description: "Pull a card from another player's hand. If that card is a Hero card, you may play it immediately."
  categories:
    - hero
  params:
    hero-type: bard
    effect-roll: "7+"
- label: "Dodgy Dealer"
  image: cards/heros/bards/dodgy_dealer.jpg
  deck: draw
  description: "Trade hands with another player."
  categories:
    - hero
  params:
    hero-type: bard
    effect-roll: "9+"
- label: "Greedy Cheeks"
  image: cards/heros/bards/greedy_cheeks.jpg
  deck: draw
  description: "Each other player must give you a card from their hand."
  categories:
    - hero
  params:
    hero-type: bard
    effect-roll: "8+"
- label: "Fluffy"
  image: cards/heros/wizard/fluffy.jpg
  deck: draw
  description: "DESTROY 2 Hero cards."
  categories:
    - hero
  params:
    hero-type: wizard
    effect-roll: "10+"
- label: "Wiggles"
  image: cards/heros/wizard/wiggles.jpg
  deck: draw
  description: "STEAL a Hero card and roll to use its effect immediately."
  categories:
    - hero
  params:
    hero-type: wizard
    effect-roll: "10+"
- label: "Spooky"
  image: cards/heros/wizard/spooky.jpg
  deck: draw
  description: "Each other player must SACRIFICE a Hero card."
  categories:
    - hero
  params:
    hero-type: wizard
    effect-roll: "10+"
- label: "Snowball"
  image: cards/heros/wizard/snowball.jpg
  deck: draw
  description: "DRAW a card. If it is a Magic card, you may play it immediately and DRAW a second card."
  categories:
    - hero
  params:
    hero-type: wizard
    effect-roll: "6+"
- label: "Buttons"
  image: cards/heros/wizard/buttons.jpg
  deck: draw
  description: "Pull a card from another player's hand. If it is a Magic card, you may play it immediately."
  categories:
    - hero
  params:
    hero-type: wizard
    effect-roll: "6+"
- label: "Bun Bun"
  image: cards/heros/wizard/bun_bun.jpg
  deck: draw
  description: "Search the discard pile for a Magic card and add it to your hand."
  categories:
    - hero
  params:
    hero-type: wizard
    effect-roll: "5+"
- label: "Hopper"
  image: cards/heros/wizard/hopper.jpg
  deck: draw
  description: "Choose a player. That player must SACRIFICE a Hero card."
  categories:
    - hero
  params:
    hero-type: wizard
    effect-roll: "7+"
- label: "Whiskers"
  image: cards/heros/wizard/whiskers.jpg
  deck: draw
  description: "STEAL a Hero card and DESTROY a Hero card."
  categories:
    - hero
  params:
    hero-type: wizard
    effect-roll: "11+"
     */
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
/*
- label: "Bard Mask"
  image: cards/items/bard_mask.jpg
  deck: draw
  description: "The equipped Hero card is considered a Bard instead of its original class."
  categories:
    - item
    - mask
  params:
    mask: bard
- label: "Ranger Mask"
  image: cards/items/ranger_mask.jpg
  deck: draw
  description: "The equipped Hero card is considered a Ranger instead of its original class."
  categories:
    - item
    - mask
  params:
    mask: ranger
- label: "Fighter Mask"
  image: cards/items/fighter_mask.jpg
  deck: draw
  description: "The equipped Hero card is considered a Fighter instead of its original class."
  categories:
    - item
    - mask
  params:
    mask: fighter
- label: "Thief Mask"
  image: cards/items/thief_mask.jpg
  deck: draw
  description: "The equipped Hero card is considered a Thief instead of its original class."
  categories:
    - item
    - mask
  params:
    mask: thief
- label: "Guardian Mask"
  image: cards/items/guardian_mask.jpg
  deck: draw
  description: "The equipped Hero card is considered a Guardian instead of its original class."
  categories:
    - item
    - mask
  params:
    mask: guardian
- label: "Wizard Mask"
  image: cards/items/wizard_mask.jpg
  deck: draw
  description: "The equipped Hero card is considered a Wizard instead of its original class."
  categories:
    - item
    - mask
  params:
    mask: wizard
- label: "Decoy Doll"
  image: cards/items/decoy_doll.jpg
  deck: draw
  description: "If the equipped Hero card would be sacrificed or destroyed, move Decoy Doll to the discard pile instead."
  categories:
    - item
- label: "Really Big Ring"
  image: cards/items/really_big_ring.jpg
  deck: draw
  description: "Each time you roll to use the equipped Hero card's effect, +2 to your roll."
  categories:
    - item
  repeat: 2
- label: "Particularly Rusty Coin"
  image: cards/items/particularly_rusty_coin.jpg
  deck: draw
  description: "If you unsuccessfully roll to use the equipped Hero card's effect, DRAW a card."
  categories:
    - item
  repeat: 2
- label: "Sealing Key"
  image: cards/cursed_items/sealing_key.jpg
  deck: draw
  description: "You cannot use the equipped Hero card's effect."
  categories:
    - "cursed item"
- label: "Suspiciously Shiny Coin"
  image: cards/cursed_items/suspiciously_shiny_coin.jpg
  deck: draw
  description: "If you sucessfully roll to use the equipped Hero card's effect, DISCARD a card."
  categories:
    - "cursed item"
- label: "Curse of the Snake's Eyes"
  image: cards/cursed_items/curse_of_the_snakes_eyes.jpg
  deck: draw
  description: "Each time you roll to use the equipped Hero card's effect, -2 to your roll."
  categories:
    - "cursed item"
  repeat: 2 */
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
        label: "Mega Slime".to_string(),
        image_path: "cards/monsters/mega_slime.jpg".to_string(),
        description: "You may spend an extra action point on each of your turns.".to_string(),
        monster: Some(
          MonsterSpec {
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
        }),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Monster,
        label: "Orthus".to_string(),
        image_path: "cards/monsters/orthus.jpg".to_string(),
        description: "Each time you DRAW a Magic card, you may play it immediately.".to_string(),
        monster: Some(
            MonsterSpec {
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
        }),
        ..Default::default()
    },
    CardSpec {
        card_type: CardType::Monster,
        label: "Terratuga".to_string(),
        image_path: "cards/monsters/terratuga.jpg".to_string(),
        description: "Your Hero cards cannot be destroyed.".to_string(),
        monster: Some(
          MonsterSpec {
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
        }),
        ..Default::default()
    },
]
}
