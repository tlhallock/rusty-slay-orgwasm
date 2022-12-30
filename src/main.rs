pub mod backend;
pub mod common;
pub mod frontend;
pub mod slay;

pub fn main() {
	frontend::view::render();
}
