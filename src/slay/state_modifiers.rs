use crate::slay::errors::{SlayError, SlayResult};
use crate::slay::ids;
use crate::slay::state;

pub fn transfer_upto_n(num: u32, from: &mut state::Deck, to: &mut state::Deck) {
	for _ in 0..num {
		// Need to handle no more cards...
		if let Some(s) = from.stacks.pop_front() {
			to.stacks.push_back(s);
		}
	}
}

// There is also deck.take

pub fn transfer_a_top_card(
	card_id: ids::CardId, from: &mut state::Deck, to: &mut state::Deck,
) -> SlayResult<()> {
	// Probably could be more efficient...
	let position = from
		.stacks
		.iter()
		.position(|c| c.top.id == card_id)
		.ok_or_else(|| SlayError::new("Card not found."))?;
	let stack = from.stacks.remove(position).unwrap();
	to.stacks.push_back(stack);
	Ok(())
}

// pub fn move_card<'a>(
//     source: &'a mut state::Deck,
//     destination: &'a mut state::Deck,
//     card_id: ids::ElementId,
// ) -> Result<usize> {
//     // ) -> Result<&'a mut state::Card> {

//     let position = source
//         .stacks
//         .iter()
//         .position(|s| s.cards.iter().any(|c| c.id == card_id))
//         .ok_or_else(|| SlayError::new("Unable to find card in deck."))?;

//     // let mut position = None;
//     // 'outer:
//     // for (si, stack) in source.stacks.iter_mut().enumerate() {
//     //   for card in stack.cards.iter_mut() {
//     //     if card.id != card_id {
//     //       continue;
//     //     }
//     //     position = Some((si, card));
//     //     break 'outer;
//     //   }
//     // }
//     // let (stack_index, card) = position
//     //   .ok_or(SlayError{reason: "Card not found in deck.".to_string() })?;

//     let current_size = source.stacks.len();
//     let stack = source.stacks.remove(position);
//     destination.stacks.push(stack);
//     // stack.cards.iter_mut()
//     //   .find(|c| c.id == card_id)
//     //   .ok_or(SlayError::new("Wish this didnt have to be here, we already found it once."))
//     Ok(current_size)
// }
