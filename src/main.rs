use slay::driver;

pub mod backend;
pub mod common;
pub mod frontend;
pub mod slay;

pub fn main() {
	driver::game_loop().expect("oops");
	// frontend::view::render();
}
