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
impl MagicSpell {
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
