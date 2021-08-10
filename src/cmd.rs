use std::{
	error::Error,
	fs,
	io::{
		self,
		Read,
	},
	path::PathBuf,
};

use atty::Stream;
use clipboard::{
	ClipboardContext,
	ClipboardProvider,
};

use crate::app;

pub struct Cmd {
	clear: bool,
	file: Option<PathBuf>,
}

impl Cmd {
	pub fn from_args() -> Self {
		let m = app::new().get_matches();

		let clear = m.is_present("clear");
		let file = m.value_of("file").map(PathBuf::from);

		Self { clear, file }
	}

	pub fn run(&self) -> Result<(), Box<dyn Error>> {
		if self.clear {
			set_clip(String::new())
		} else if let Some(p) = &self.file {
			fs::read_to_string(&p).map(|s| {
				if !atty::is(Stream::Stdout) {
					println!("{}", &s);
				}
				set_clip(s)
			})?
		} else if !atty::is(Stream::Stdin) {
			let stdin = io::stdin();
			let mut stdin = stdin.lock();
			let mut buf = String::new();
			stdin.read_to_string(&mut buf)?;
			if !atty::is(Stream::Stdout) {
				println!("{}", &buf);
			}
			set_clip(buf)
		} else {
			print_clip()
		}
	}
}

fn print_clip() -> Result<(), Box<dyn Error>> {
	let mut ctx: ClipboardContext = ClipboardProvider::new()?;
	print!("{}", ctx.get_contents()?);
	Ok(())
}

fn set_clip(s: String) -> Result<(), Box<dyn Error>> {
	let mut ctx: ClipboardContext = ClipboardProvider::new()?;
	ctx.set_contents(s)
}
