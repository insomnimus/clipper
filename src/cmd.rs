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
use clap::{
	arg,
	crate_version,
	App,
	Arg,
};
use clipboard::{
	ClipboardContext,
	ClipboardProvider,
};

pub struct Cmd {
	clear: bool,
	no_print: bool,
	file: Option<PathBuf>,
}

impl Cmd {
	pub fn from_args() -> Self {
		let m = App::new("clipper")
			.version(crate_version!())
			.about("Manage the system clipboard.")
			.after_help(
				"\
If there is no input (either from a file or from stdin)
or if the stdout is piped,
the contents of the clipboard will be written to stdout.

This command only supports UTF-8 pipes.\
",
			)
			.args(&[
				arg!(-n --noprint "Do not print the clipboard even if stdout is piped."),
				arg!(clear: -x --clear "Clear the clipboard.").conflicts_with("file"),
				Arg::new("file").help("Copy the contents of a file to the clipboard."),
			])
			.get_matches();

		let clear = m.is_present("clear");
		let file = m.value_of("file").map(PathBuf::from);
		let no_print = m.is_present("noprint");

		Self {
			clear,
			file,
			no_print,
		}
	}

	pub fn run(&self) -> Result<(), Box<dyn Error>> {
		if self.clear {
			set_clip(String::new())
		} else if let Some(p) = &self.file {
			fs::read_to_string(&p).map(|s| {
				if !self.no_print && !atty::is(Stream::Stdout) {
					println!("{}", &s);
				}
				set_clip(s)
			})?
		} else if !atty::is(Stream::Stdin) {
			let stdin = io::stdin();
			let mut stdin = stdin.lock();
			let mut buf = String::new();
			stdin.read_to_string(&mut buf)?;
			if !self.no_print && !atty::is(Stream::Stdout) {
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
