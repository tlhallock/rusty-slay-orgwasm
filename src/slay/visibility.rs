
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Visibility {
	Visible,
	Summary,
	//   Hidden,
	Invisible,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Perspective {
	Owner,
	Spectator,
	Jesus,
}

#[derive(Debug, Clone)]
pub struct VisibilitySpec {
	pub owner: Visibility,
	pub others: Visibility,
}

impl VisibilitySpec {
	pub fn summary() -> Self {
		Self {
			owner: Visibility::Visible,
			others: Visibility::Summary,
		}
	}
	pub fn invisible() -> Self {
		Self {
			owner: Visibility::Invisible,
			others: Visibility::Invisible,
		}
	}
	pub fn visible() -> Self {
		Self {
			owner: Visibility::Visible,
			others: Visibility::Visible,
		}
	}
	pub fn get(&self, perspective: &Perspective) -> &Visibility {
		match perspective {
			Perspective::Jesus => &self.owner,
			Perspective::Owner => &self.owner,
			Perspective::Spectator => &self.others,
		}
	}
	pub fn is_visible(&self, perspective: &Perspective) -> bool {
		self.get(perspective) != &Visibility::Invisible
	}
}
