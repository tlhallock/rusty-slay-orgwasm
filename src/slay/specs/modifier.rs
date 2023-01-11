use enum_iterator::Sequence;

#[derive(Debug, Clone, Sequence)]
pub enum ModifierKinds {
	Plus4,
	Plus3Minus1,
	Plus2Minus2,
	Plus1Minus3,
	Minus4,
}
impl ModifierKinds {}
