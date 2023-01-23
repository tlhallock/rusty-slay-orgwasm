use slay::driver;

pub mod backend;
pub mod common;
pub mod frontend;
pub mod slay;

pub fn main() {
	// driver::game_loop().expect("oops");
	// There should be a way to tell if an action is not needed, as in, don't roll for something that you can't do
	// We need state on the player to determine if they have made a play this turn already
	// It would be nice to have a go back, part way through an action
	// Maybe a redo method on the actions...
	frontend::view::render();
}
