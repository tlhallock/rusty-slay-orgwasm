use crate::slay::errors::{SlayError, SlayResult};
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::game::Game;

use rand::Rng;

// Emit logs like "Waiting for challenges..."

pub fn pick_a_random_choice(
	context: &mut GameBookKeeping, game: &mut Game,
) -> SlayResult<(ids::PlayerIndex, ids::ChoiceId)> {
	// reservoir sampling
	let mut count = 0;
	let mut ret = None;
	for player in game.players.iter_mut() {
		if let Some(choices) = player.choices.as_mut() {
			for choice in choices.options.iter_mut() {
				count += 1;
				if context.rng.gen::<f32>() < 1f32 / (count as f32) {
					ret = Some((player.player_index, choice.id));
				}
			}
		}
	}
	ret.ok_or_else(|| SlayError::new("No choices found."))
}

// pub fn pick_a_random_choice(
//     context: &mut GameBookKeeping,
//     game: &mut Game,
// ) -> Result<(ids::PlayerId, ids::ChoiceId)> {
//     // reservoir sampling
//     let mut count = 0;
//     let mut ret = None;
//     for player in game.players.iter_mut() {
//         if let Some(choices) = player.choices.as_mut() {
//             for choice in choices.options.iter_mut() {
//                 count += 1;
//                 if context.rng.gen::<f32>() < 1f32 / (count as f32) {
//                     ret = Some((player.id, choice.get_choice_information().id));
//                 }
//             }
//         }
//     }
//     println!("Chose among {count} active choices.");
//     ret.ok_or_else(|| SlayError::new("No choices found."))
// }
