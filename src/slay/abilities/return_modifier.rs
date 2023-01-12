// #[derive(Debug, Clone)]
// pub struct ReturnModifier {
// 	num: u32,
// 	include: Option<HashSet<ids::CardId>>,
// }

// impl Discard {
// 	pub fn new(num: u32) -> Self {
// 		Self { num, include: None }
// 	}
// 	pub fn discard_one_of(include: HashSet<ids::CardId>) -> Self {
// 		Self {
// 			num: 1,
// 			include: Some(include),
// 		}
// 	}

// 	pub fn should_include(&self, card_id: ids::CardId) -> bool {
// 		if let Some(include) = self.include {
// 			include.contains(&card_id)
// 		} else {
// 			true
// 		}
// 	}
// }
