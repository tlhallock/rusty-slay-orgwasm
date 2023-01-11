use enum_iterator::all;
use slay::{
	driver,
	specs::{cards::SlayCardSpec, monster::Monster},
};

pub mod backend;
pub mod common;
pub mod frontend;
pub mod slay;

pub fn main() {
	let monsters = all::<SlayCardSpec>().collect::<Vec<_>>();
	for monster in monsters.iter() {
		println!("{:?}", monster)
	}

	// driver::game_loop().expect("oops");
	// frontend::view::render();
}
