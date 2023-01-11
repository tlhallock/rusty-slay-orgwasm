use std::io::BufWriter;
use std::io::Write;

pub trait Summarizable {
	fn summarize<W: Write>(
		&self,
		f: &mut BufWriter<W>,
		indentation_level: u32,
	) -> Result<(), std::io::Error>;
}
