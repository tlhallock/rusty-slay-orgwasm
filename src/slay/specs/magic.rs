use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, Sequence, PartialEq)]
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
impl MagicSpell {}
