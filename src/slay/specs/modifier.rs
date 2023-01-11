use enum_iterator::Sequence;

#[derive(Debug, Clone, Sequence, PartialEq, Copy)]
pub enum ModifierKinds {
	Plus4,
	Plus3Minus1,
	Plus2Minus2,
	Plus1Minus3,
	Minus4,
}
impl ModifierKinds {
	pub fn list_amounts(&self) -> Vec<i32> {
		match self {
			ModifierKinds::Plus4 => vec![4],
			ModifierKinds::Plus3Minus1 => vec![3, -1],
			ModifierKinds::Plus2Minus2 => vec![2, -2],
			ModifierKinds::Plus1Minus3 => vec![1, -3],
			ModifierKinds::Minus4 => vec![-4],
		}
	}
}
