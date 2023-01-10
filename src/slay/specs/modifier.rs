#[derive(Debug, Clone)]
pub enum ModifierKinds {
	Plus4,
	Plus3Minus1,
	Plus2Minus2,
	Plus1Minus3,
	Minus4,
}
impl ModifierKinds {
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
