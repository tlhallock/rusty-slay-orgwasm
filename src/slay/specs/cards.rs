

use crate::slay::modifiers::ItemModifier;
use crate::slay::specification::CardType;
use crate::slay::specification::HeroAbility;
use crate::slay::specs::monster::Monster;
use crate::slay::specs::modifier::ModifierKinds;
use crate::slay::specs::magic::MagicSpell;
use crate::slay::specs::items::AnotherItemType;






#[derive(Debug, Clone)]
pub enum SlayCardSpec {
  MonsterCard(Monster),
  MagicCard(MagicSpell),
  ModifierCard(ModifierKinds),
  Item(AnotherItemType),
  Challenge,
}


impl SlayCardSpec {
  pub fn label(&self) -> String {
    match self {
        SlayCardSpec::MonsterCard(c) => c.label(),
        SlayCardSpec::MagicCard(c) => c.label(),
        SlayCardSpec::ModifierCard(c) => c.label(),
        SlayCardSpec::Item(c) => c.label(),
        SlayCardSpec::Challenge => String::from("Challenge"),
    }
  }
  pub fn description(&self) -> String {
    match self {
        SlayCardSpec::MonsterCard(c) => c.description(),
        SlayCardSpec::MagicCard(c) => c.description(),
        SlayCardSpec::ModifierCard(c) => c.description(),
        SlayCardSpec::Item(c) => c.description(),
        SlayCardSpec::Challenge => String::from("You may play this card when another player attempts to play a Hero, Item, or Magic card. CHALLENGE that card."),
    }
  }
  pub fn image_path(&self) -> String {
    match self {
        SlayCardSpec::MonsterCard(c) => c.image_path(),
        SlayCardSpec::MagicCard(c) => c.image_path(),
        SlayCardSpec::ModifierCard(c) => c.image_path(),
        SlayCardSpec::Item(c) => c.image_path(),
        SlayCardSpec::Challenge => String::from("cards/challenge/challenge.jpg"),
    }
  }
}



