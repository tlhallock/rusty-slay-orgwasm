use crate::slay::specification::HeroType;




#[derive(Debug, Clone)]
pub enum Item {
  DecoyDoll,
  ReallyBigRing,
  ParticularlyRustyCoin,
  SealingKey,
  SuspiciouslyShinyCoin,
  CurseOfTheSnakesEyes,
}


#[derive(Debug, Clone)]
pub enum AnotherItemType {
  MaskCard(HeroType),
  NotMask(Item, bool),
}
impl AnotherItemType {
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