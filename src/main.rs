pub mod backend;
pub mod common;
pub mod frontend;
pub mod slay;

#[macro_use]
extern crate guard;

pub fn main() {
	// There should be a way to tell if an action is not needed, as in, don't roll for something that you can't do
	// We need state on the player to determine if they have made a play this turn already
	// It would be nice to have a go back, part way through an action
	// Maybe a redo method on the actions...
	// Modifier should be item...
	// cards can only have one item...
	// Let users know that they cannot choose to destroy a hero from a player that can
	// View the hero leader...
	// Make players clickable?
	// make a strategy that just draws cards...
	// The only thing that should set choices are from that player's own task.
	// Shadow claw and piercing howl cannot be used in two player games\

	// full party + monster
	// 8 card hand limit
	// party leader does not count
	// 6 hero max party

	// 5 monsters to win, no hero win condition

	/*
	Emit the following notifications:


	PlayerIsChoosing(ids::PlayerIndex, ChoicesType),
	Modification,
	InitialRoll(i32),
	RollResult(bool), // show the roll value and threshold?
	OfferResult, // Option<ids::PlayerIndex>
	ChallengeResult(bool),

	*/

	slay::driver::game_loop().expect("oops");
	// frontend::view::render();
}
