use crate::slay::modifiers::ItemModifier;
use crate::slay::showdown::consequences::Condition;
use crate::slay::specification::CardSpec;
use crate::slay::specification::CardType;
use crate::slay::specification::HeroType;
use crate::slay::specification::ItemType;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::specs::hero::HeroAbility;
use crate::slay::specs::hero::HeroAbilityType;
use crate::slay::specs::items::Item;
use crate::slay::specs::magic::MagicSpell;
use crate::slay::specs::modifier::ModifierKinds;
use crate::slay::specs::monster::Monster;

impl SlayCardSpec {
	pub fn get_card_spec_creation(self: &SlayCardSpec) -> CardSpec {
		match self {
        SlayCardSpec::HeroCard(hero) => match hero {
            HeroAbilityType::PlunderingPuma => CardSpec {
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
            HeroAbilityType::SlipperyPaws => CardSpec {
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
            HeroAbilityType::SmoothMimimeow => CardSpec {
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
            HeroAbilityType::Meowzio => CardSpec {
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
            HeroAbilityType::Shurikitty => CardSpec {
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
            HeroAbilityType::KitNapper => CardSpec {
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
            HeroAbilityType::SilentShadow => CardSpec {
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
            HeroAbilityType::SlyPickings => CardSpec {
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
            HeroAbilityType::HolyCurselifter => CardSpec {
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
            HeroAbilityType::IronResolve => CardSpec {
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
            HeroAbilityType::CalmingVoice => CardSpec {
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
            HeroAbilityType::VibrantGlow => CardSpec {
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


            HeroAbilityType::MightyBlade => CardSpec {
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
            HeroAbilityType::RadiantHorn => CardSpec {
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
            HeroAbilityType::GuidingLight => CardSpec {
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
            HeroAbilityType::WiseShield => CardSpec {
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
            HeroAbilityType::QiBear => CardSpec {
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
            HeroAbilityType::PanChucks => CardSpec {
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
            HeroAbilityType::HeavyBear => CardSpec {
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
            HeroAbilityType::BadAxe => CardSpec {
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
            HeroAbilityType::ToughTeddy => CardSpec {
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
            HeroAbilityType::BearClaw => CardSpec {
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
            HeroAbilityType::FuryKnuckle => CardSpec {
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


            HeroAbilityType::BearyWise => CardSpec {
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
            HeroAbilityType::Hook => CardSpec {
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
            HeroAbilityType::Wildshot => CardSpec {
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
            HeroAbilityType::SeriousGrey => CardSpec {
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
            HeroAbilityType::WilyRed => CardSpec {
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
            HeroAbilityType::QuickDraw => CardSpec {
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
            HeroAbilityType::LookieRookie => CardSpec {
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
            HeroAbilityType::Bullseye => CardSpec {
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
            HeroAbilityType::SharpFox => CardSpec {
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
            HeroAbilityType::FuzzyCheeks => CardSpec {
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
            HeroAbilityType::Peanut => CardSpec {
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
            HeroAbilityType::NappingNibbles => CardSpec {
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
            HeroAbilityType::TipsyTootie => CardSpec {
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
            HeroAbilityType::MellowDee => CardSpec {
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
            HeroAbilityType::LuckBucky => CardSpec {
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
            HeroAbilityType::DodgyDealer => CardSpec {
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
            HeroAbilityType::GreedyCheeks => CardSpec {
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
            HeroAbilityType::Fluffy => CardSpec {
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
            HeroAbilityType::Wiggles => CardSpec {
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
            HeroAbilityType::Spooky => CardSpec {
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
            HeroAbilityType::Snowball => CardSpec {
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
            HeroAbilityType::Buttons => CardSpec {
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
            HeroAbilityType::BunBun => CardSpec {
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
            HeroAbilityType::Hopper => CardSpec {
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
            HeroAbilityType::Whiskers => CardSpec {
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
        },
        SlayCardSpec::MonsterCard(monster) => match monster {
            Monster::AnuranCauldron => CardSpec {
              card_type: CardType::Monster,
              label: "Anuran Cauldron".to_string(),
              image_path: "cards/monsters/anuran_cauldron.jpg".to_string(),
              description: "Each time you roll, +1 to your roll.".to_string(),
              monster: Some(Monster::AnuranCauldron),
              ..Default::default()
          },
            Monster::TitanWyvern => CardSpec {
              card_type: CardType::Monster,
              label: "Titan Wyvern".to_string(),
              image_path: "cards/monsters/titan_wyvern.jpg".to_string(),
              description: "Each time you roll for a Challenge card, +1 to your roll.".to_string(),
              monster: Some(Monster::TitanWyvern),
              ..Default::default()
          },
            Monster::DarkDragonKing => CardSpec {
              card_type: CardType::Monster,
              label: "Dark Dragon King".to_string(),
              image_path: "cards/monsters/dark_dragon_king.jpg".to_string(),
              description: "Each time you roll for a Hero card's effect, +1 to your roll.".to_string(),
              monster: Some(Monster::DarkDragonKing),
              ..Default::default()
          },
            Monster::AbyssQueen => CardSpec {
              card_type: CardType::Monster,
              label: "Abyss Queen".to_string(),
              image_path: "cards/monsters/abyss_queen.jpg".to_string(),
              description: "Each time another player plays a Modifier card on one of your rolls, +1 to your roll.".to_string(),
              monster: Some(Monster::AbyssQueen),
              ..Default::default()
          },
            Monster::RexMajor => CardSpec {
              card_type: CardType::Monster,
              label: "Rex Major".to_string(),
              image_path: "cards/monsters/rex_major.jpg".to_string(),
              description: "Each time you DRAW a Modifier card, you may reveal it and DRAW a second card.".to_string(),
              monster: Some(Monster::RexMajor),
              ..Default::default()
          },
            Monster::CorruptedSabretooth => CardSpec {
              card_type: CardType::Monster,
              label: "Corrupted Sabretooth".to_string(),
              image_path: "cards/monsters/corrupted_sabretooth.jpg".to_string(),
              description: "Each time you would DESTROY a Hero card, you may STEAL that Hero card instead.".to_string(),
              monster: Some(Monster::CorruptedSabretooth),
              ..Default::default()
          },
            Monster::CrownedSerpent => CardSpec {
              card_type: CardType::Monster,
              label: "Crowned Serpent".to_string(),
              image_path: "cards/monsters/crowned_serpent.jpg".to_string(),
              description: "Each time any player (including you) plays a Modifier card, you may DRAW a card.".to_string(),
              monster: Some(Monster::CrownedSerpent),
              ..Default::default()
          },
            Monster::WarwornOwlbear => CardSpec {
              card_type: CardType::Monster,
              label: "Warworn Owlbear".to_string(),
              image_path: "cards/monsters/warworn_owlbear.jpg".to_string(),
              description: "Item cards you play cannot be challenged.".to_string(),
              monster: Some(Monster::WarwornOwlbear),
              ..Default::default()
          },
            Monster::Dracos => CardSpec {
              card_type: CardType::Monster,
              label: "Dracos".to_string(),
              image_path: "cards/monsters/dracos.jpg".to_string(),
              description: "Each time a Hero card in your Party is destroyed, you may DRAW a card.".to_string(),
              monster: Some(Monster::Dracos),
              ..Default::default()
          },
            Monster::Malammoth => CardSpec {
              card_type: CardType::Monster,
              label: "Malammoth".to_string(),
              image_path: "cards/monsters/malamammoth.jpg".to_string(),
              description: "Each time you DRAW an Item card, you may play it immediately.".to_string(),
              monster: Some(Monster::Malammoth),
              ..Default::default()
          },
            Monster::Bloodwing => CardSpec {
              card_type: CardType::Monster,
              label: "Bloodwing".to_string(),
              image_path: "cards/monsters/bloodwing.jpg".to_string(),
              description: "Each time another player CHALLENGES you, that player must DISCARD a card.".to_string(),
              monster: Some(Monster::Bloodwing),
              ..Default::default()
          },
            Monster::ArcticAries => CardSpec {
              card_type: CardType::Monster,
              label: "Arctic Aries".to_string(),
              image_path: "cards/monsters/arctic_aries.jpg".to_string(),
              description: "Each time you successfully roll to use a Hero card's effect, you may DRAW a card.".to_string(),
              monster: Some(Monster::ArcticAries),
              ..Default::default()
          },
            Monster::MegaSlime => CardSpec {
              card_type: CardType::Monster,
              label: "Mega Slime".to_string(),
              image_path: "cards/monsters/mega_slime.jpg".to_string(),
              description: "You may spend an extra action point on each of your turns.".to_string(),
              monster: Some(Monster::MegaSlime),
              ..Default::default()
          },
            Monster::Orthus => CardSpec {
              card_type: CardType::Monster,
              label: "Orthus".to_string(),
              image_path: "cards/monsters/orthus.jpg".to_string(),
              description: "Each time you DRAW a Magic card, you may play it immediately.".to_string(),
              monster: Some(Monster::Orthus),
              ..Default::default()
          },
            Monster::Terratuga => CardSpec {
              card_type: CardType::Monster,
              label: "Terratuga".to_string(),
              image_path: "cards/monsters/terratuga.jpg".to_string(),
              description: "Your Hero cards cannot be destroyed.".to_string(),
              monster: Some(Monster::Terratuga),
              ..Default::default()
          },
        },
        SlayCardSpec::MagicCard(magic) => match magic {
            MagicSpell::EnganglingTrap => CardSpec {
              card_type: CardType::Magic,
              label: "Entangling Trap".to_string(),
              image_path: "cards/magic/entangling_trap.jpg".to_string(),
              description: "DISCARD 2 cards, then STEAL a Hero card.".to_string(),
              spell: Some(MagicSpell::EnganglingTrap),
              repeat: 2,
              ..Default::default()
            },
            MagicSpell::CriticalBoost => CardSpec {
              card_type: CardType::Magic,
              label: "Critical Boost".to_string(),
              image_path: "cards/magic/critical_boost.jpg".to_string(),
              description: "DRAW 3 cards and DISCARD a card.".to_string(),
              spell: Some(MagicSpell::CriticalBoost),
              repeat: 2,
              ..Default::default()
            },
            MagicSpell::DestructiveSpell => CardSpec {
              card_type: CardType::Magic,
              label: "Destructive Spell".to_string(),
              image_path: "cards/magic/descructive_spell.jpg".to_string(),
              description: "DISCARD a card, then DESTROY a Hero card.".to_string(),
              spell: Some(MagicSpell::DestructiveSpell),
              repeat: 2,
              ..Default::default()
            },
            MagicSpell::WindsOfChange => CardSpec {
              card_type: CardType::Magic,
              label: "Winds of Change".to_string(),
              image_path: "cards/magic/winds_of_change.jpg".to_string(),
              description: "Return an Item card equipped to any player's Hero card to that player's hand, then DRAW a card.".to_string(),
              spell: Some(MagicSpell::WindsOfChange),
              repeat: 2,
              ..Default::default()
            },
            MagicSpell::EnchangedSpell => CardSpec {
              card_type: CardType::Magic,
              label: "Enchanted Spell".to_string(),
              image_path: "cards/magic/enchanged_spell.jpg".to_string(),
              description: "+2 to all of your rolls until the end of your turn".to_string(),
              spell: Some(MagicSpell::EnchangedSpell),
              repeat: 2,
              ..Default::default()
            },
            MagicSpell::ForcedExchange => CardSpec {
              card_type: CardType::Magic,
              label: "Forced Exchange".to_string(),
              image_path: "cards/magic/forced_exchange.jpg".to_string(),
              description: "Choose a player. STEAL a Hero card from that player's Party, then move a Hero card from your Party to that player's Party.".to_string(),
              spell: Some(MagicSpell::ForcedExchange),
              ..Default::default()
            },
            MagicSpell::ForcefulWinds => CardSpec {
              card_type: CardType::Magic,
              label: "Forceful Winds".to_string(),
              image_path: "cards/magic/forceful_winds.jpg".to_string(),
              description: "Return every equipped item card to its respective player's hand".to_string(),
              spell: Some(MagicSpell::ForcefulWinds),
              ..Default::default()
            },
            MagicSpell::CallToTheFallen => CardSpec {
              card_type: CardType::Magic,
              label: "Call to the Fallen".to_string(),
              image_path: "cards/magic/call_to_the_fallen.jpg".to_string(),
              description: "Search the discard pile for a Hero card and add it to your hand.".to_string(),
              spell: Some(MagicSpell::CallToTheFallen),
              ..Default::default()
            },
        },
        SlayCardSpec::ModifierCard(modifier) => match modifier {
            ModifierKinds::Plus4 => CardSpec {
              card_type: CardType::Modifier,
              label: "Modifier +4".to_string(),
              repeat: 4,
              image_path: "cards/modifier/4.jpg".to_string(),
              description: "Play this card after any player (including you) rolls the dice. +4 to that roll.".to_string(),
              modifiers: vec![4],
              ..Default::default()
          },
            ModifierKinds::Plus3Minus1 => CardSpec {
              card_type: CardType::Modifier,
              label: "Modifier +3/-1".to_string(),
              repeat: 4,
              image_path: "cards/modifier/3.jpg".to_string(),
              description: "Play this card after any player (including you) rolls the dice. +3 or -1 to that roll.".to_string(),
              modifiers: vec![3, -1],
              ..Default::default()
          },
            ModifierKinds::Plus2Minus2 => CardSpec {
              card_type: CardType::Modifier,
              label: "modifier +2/-2".to_string(),
              repeat: 9,
              image_path: "cards/modifier/2.jpg".to_string(),
              description: "Play this card after any player (including you) rolls the dice. +2 or -2 to that roll.".to_string(),
              modifiers: vec![2, -2],
              ..Default::default()
          },
            ModifierKinds::Plus1Minus3 => CardSpec {
              card_type: CardType::Modifier,
              label: "modifier +1/-3".to_string(),
              repeat: 4,
              image_path: "cards/modifier/1.jpg".to_string(),
              description: "Play this card after any player (including you) rolls the dice. +1 or -3 to that roll.".to_string(),
              modifiers: vec![1, -3],
              ..Default::default()
          },
            ModifierKinds::Minus4 => CardSpec {
              card_type: CardType::Modifier,
              label: "modifier -4".to_string(),
              repeat: 4,
              image_path: "cards/modifier/0.jpg".to_string(),
              description: "Play this card after any player (including you) rolls the dice. -4 to that roll.".to_string(),
              modifiers: vec![-4],
              ..Default::default()
          },
        },
        SlayCardSpec::Item(item2) => match item2 {
            super::items::AnotherItemType::MaskCard(mask_type) => match mask_type {
                HeroType::Bard => CardSpec {
                  card_type: CardType::Item(ItemType::Mask),
                  label: "Bard Mask".to_string(),
                  image_path: "cards/items/bard_mask.jpg".to_string(),
                  description: "The equipped Hero card is considered a Bard instead of its original class.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::Mask(HeroType::Bard)),
                  ..Default::default()
                },
                HeroType::Wizard => CardSpec {
                  card_type: CardType::Item(ItemType::Mask),
                  label: "Wizard Mask".to_string(),
                  image_path: "cards/items/wizard_mask.jpg".to_string(),
                  description: "The equipped Hero card is considered a Wizard instead of its original class.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::Mask(HeroType::Wizard)),
                  ..Default::default()
                },
                HeroType::Fighter => CardSpec {
                  card_type: CardType::Item(ItemType::Mask),
                  label: "Fighter Mask".to_string(),
                  image_path: "cards/items/fighter_mask.jpg".to_string(),
                  description: "The equipped Hero card is considered a Fighter instead of its original class.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::Mask(HeroType::Fighter)),
                  ..Default::default()
                },
                HeroType::Gaurdian => CardSpec {
                  card_type: CardType::Item(ItemType::Mask),
                  label: "Guardian Mask".to_string(),
                  image_path: "cards/items/guardian_mask.jpg".to_string(),
                  description: "The equipped Hero card is considered a Guardian instead of its original class.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::Mask(HeroType::Gaurdian)),
                  ..Default::default()
                },
                HeroType::Ranger => CardSpec {
                  card_type: CardType::Item(ItemType::Mask),
                  label: "Ranger Mask".to_string(),
                  image_path: "cards/items/ranger_mask.jpg".to_string(),
                  description: "The equipped Hero card is considered a Ranger instead of its original class.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::Mask(HeroType::Ranger)),
                  ..Default::default()
                },
                HeroType::Thief => CardSpec {
                  card_type: CardType::Item(ItemType::Mask),
                  label: "Thief Mask".to_string(),
                  image_path: "cards/items/thief_mask.jpg".to_string(),
                  description: "The equipped Hero card is considered a Thief instead of its original class.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::Mask(HeroType::Thief)),
                  ..Default::default()
                },
            },
            super::items::AnotherItemType::NotMask(item) => match item {
                Item::DecoyDoll => CardSpec {
                  card_type: CardType::Item(ItemType::Blessed),
                  label: "Decoy Doll".to_string(),
                  image_path: "cards/items/decoy_doll.jpg".to_string(),
                  description: "If the equipped Hero card would be sacrificed or destroyed, move Decoy Doll to the discard pile instead.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::SacrificeMeInstead),
                  ..Default::default()
                },
                Item::ReallyBigRing => CardSpec {
                  card_type: CardType::Item(ItemType::Blessed),
                  label: "Really Big Ring".to_string(),
                  image_path: "cards/items/really_big_ring.jpg".to_string(),
                  description: "Each time you roll to use the equipped Hero card's effect, +2 to your roll.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::AddToRollForAbility(2)),
                  repeat: 2,
                  ..Default::default()
                },
                Item::ParticularlyRustyCoin => CardSpec {
                  card_type: CardType::Item(ItemType::Blessed),
                  label: "Particularly Rusty Coin".to_string(),
                  image_path: "cards/items/particularly_rusty_coin.jpg".to_string(),
                  description: "If you unsuccessfully roll to use the equipped Hero card's effect, DRAW a card.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::DrawOnUnsuccessfulRollForAbility(1)),
                  repeat: 2,
                  ..Default::default()
                },
                Item::SealingKey => CardSpec {
                  card_type: CardType::Item(ItemType::Cursed),
                  label: "Sealing Key".to_string(),
                  image_path: "cards/cursed_items/sealing_key.jpg".to_string(),
                  description: "You cannot use the equipped Hero card's effect.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::RemoveAbility),
                  ..Default::default()
                },
                Item::SuspiciouslyShinyCoin => CardSpec {
                  card_type: CardType::Item(ItemType::Cursed),
                  label: "Suspiciously Shiny Coin".to_string(),
                  image_path: "cards/cursed_items/suspiciously_shiny_coin.jpg".to_string(),
                  description: "If you sucessfully roll to use the equipped Hero card's effect, DISCARD a card.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::DiscardOnSuccessfulRollForAbility(1)),
                  ..Default::default()
                },
                Item::CurseOfTheSnakesEyes => CardSpec {
                  card_type: CardType::Item(ItemType::Cursed),
                  label: "Curse of the Snake's Eyes".to_string(),
                  image_path: "cards/cursed_items/curse_of_the_snakes_eyes.jpg".to_string(),
                  description: "Each time you roll to use the equipped Hero card's effect, -2 to your roll.".to_string(),
                  spell: Some(MagicSpell::CallToTheFallen),
                  card_modifier: Some(ItemModifier::AddToRollForAbility(-2)),
                  repeat: 2,
                  ..Default::default()
                },
            },
        }
        SlayCardSpec::Challenge => CardSpec {
          card_type: CardType::Challenge,
          repeat: 8,
          label: "Challenge".to_string(),
          image_path: "cards/challenge/challenge.jpg".to_string(),
          description: "You may play this card when another player attempts to play a Hero, Item, or Magic card. CHALLENGE that card.".to_string(),
          ..Default::default()
      },
      SlayCardSpec::PartyLeader(hero_type) => match hero_type {
        HeroType::Bard => CardSpec {
          card_type: CardType::PartyLeader(HeroType::Bard),
          label: "The Charismatic Song".to_string(),
          image_path: "cards/party_leaders/charismatic_song.jpg".to_string(),
          description: "Each time you roll to use a Hero card's effect, +1 to your roll.".to_string(),
          ..Default::default()
      },
        HeroType::Wizard => CardSpec {
          card_type: CardType::PartyLeader(HeroType::Wizard),
          label: "The Cloaked Sage".to_string(),
          image_path: "cards/party_leaders/cloaked_sage.jpg".to_string(),
          description: "Each time you play a Magic card, DRAW a card.".to_string(),
          ..Default::default()
      },
        HeroType::Fighter => CardSpec {
          card_type: CardType::PartyLeader(HeroType::Fighter),
          label: "The Fist of Reason".to_string(),
          image_path: "cards/party_leaders/fist_of_reason.jpg".to_string(),
          description: "Each time you roll to CHALLENGE, +2 to your roll.".to_string(),
          ..Default::default()
      },
        HeroType::Gaurdian => CardSpec {
          card_type: CardType::PartyLeader(HeroType::Gaurdian),
          label: "The Protecting Horn".to_string(),
          image_path: "cards/party_leaders/protecting_horn.jpg".to_string(),
          description: "Each time you play a Modifier card on a roll, +1 or -1 to that roll.".to_string(),
          ..Default::default()
      },
        HeroType::Ranger => CardSpec {
          card_type: CardType::PartyLeader(HeroType::Ranger),
          label: "The Charismatic Song".to_string(),
          image_path: "cards/party_leaders/divine_arrow.jpg".to_string(),
          description: "Each time you roll to ATTACK a Monster card, +1 to your roll.".to_string(),
          ..Default::default()
      },
        HeroType::Thief => CardSpec {
          card_type: CardType::PartyLeader(HeroType::Thief),
          label: "The Shadow Claw".to_string(),
          image_path: "cards/party_leaders/shadow_claw.jpg".to_string(),
          description: "Once per turn on your turn, you may spend an action point to pull a card from another player's hand.".to_string(),
          ..Default::default()
      },
    },
    }
	}
}
