
use std::io::Write;
use std::io::BufWriter;


pub trait Summarizable {
	fn summarize<W: Write>(
		&self, f: &mut BufWriter<W>, indentation_level: u32,
	) -> Result<(), std::io::Error>;
}
