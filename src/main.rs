use std::{
	fs,
	fs::File,
	io::{
		self,
		ErrorKind,
		Read,
	},
	path::Path,
	process,
};

use anyhow::{
	anyhow,
	bail,
	ensure,
	Result,
};
use atty::Stream;
use clap::Parser;
use clipboard_win::{
	formats,
	Clipboard,
	Getter,
	Setter,
};
use fs_extra::dir::CopyOptions;
use normpath::BasePathBuf;

/// Manage the Windows clipboard
#[derive(Parser)]
#[command(
	version,
	after_help = "If no path is specified and no option is set, prints the clipboard if not piped to; if piped to, reads stdin and sets the clipboard"
)]
pub struct Cmd {
	/// Clear the clipboard
	#[arg(short = 'x', long, short_alias = 'c', exclusive = true)]
	clear: bool,

	/// Paste the contents of the clipboard to a file (use - for stdout)
	#[arg(short, long, short_alias = 'o', alias = "out", group = "action")]
	paste: bool,

	/// Copy files or folders to the clipboard like a file explorer would on
	/// ctrl+c
	#[arg(short, long, group = "action", requires = "path")]
	explorer: bool,

	/// List the files in the clipboard, if any
	#[arg(short, long, group = "action")]
	list: bool,

	/// The paths to operate on
	#[arg()]
	path: Vec<String>,
}

impl Cmd {
	fn run(&self) -> Result<()> {
		if self.clear {
			clear_clipboard()
		} else if self.list {
			list_clip()
		} else if self.explorer {
			copy_explorer(&self.path)
		} else if self.paste {
			for p in &self.path {
				paste_single(p)?;
			}
			Ok(())
		} else if self.path.is_empty() {
			if !atty::is(Stream::Stdin) {
				copy_file_contents("-")
			} else {
				paste_single("-")
			}
		} else if self.path.len() == 1 {
			copy_file_contents(&self.path[0])
		} else {
			Err(anyhow!(
				"you cannot copy more than one files contents into the clipboard"
			))
		}
	}
}

fn clear_clipboard() -> Result<()> {
	let _c = open_clip()?;
	clipboard_win::empty()?;
	Ok(())
}

fn copy_file_contents(p: &str) -> Result<()> {
	let data = if p == "-" {
		let mut data = String::with_capacity(4 << 10);
		io::stdin().lock().read_to_string(&mut data)?;
		if data.ends_with("\r\n") {
			data.pop();
			data.pop();
		}
		data
	} else {
		fs::read_to_string(p)?
	};

	let _c = open_clip()?;
	formats::Unicode.write_clipboard(&data)?;
	Ok(())
}

fn copy_explorer(files: &[String]) -> Result<()> {
	let mut fs = Vec::with_capacity(files.len());
	for f in files {
		let p = BasePathBuf::new(f).map_err(|e| anyhow!("failed to normalize {f}: {e}"))?;
		match fs::metadata(&p) {
			Err(e) if e.kind() == ErrorKind::NotFound => bail!("{f}: file not found"),
			_ => fs.push(p.into_os_string().into_string().map_err(|_| {
				anyhow!(
					"cannot convert {} into a valid unicode string (file-list)",
					BasePathBuf::new(f).unwrap().as_path().display()
				)
			})?),
		}
	}

	let _c = open_clip()?;
	formats::FileList.write_clipboard(&fs)?;
	Ok(())
}

fn paste_single(p: &str) -> Result<()> {
	let _c = open_clip()?;
	let mut files: Vec<String> = Vec::new();
	if formats::FileList.read_clipboard(&mut files).is_ok() {
		if p == "-" {
			return match files.as_slice() {
				[] => return Ok(()),
				[p] => {
					let md =
						fs::metadata(p).map_err(|e| anyhow!("cannot stat {p}: {e} (file-list)"))?;
					ensure!(!md.file_type().is_dir(), "{p}: is a directory (file-list)");
					let mut f = File::open(p)
						.map_err(|e| anyhow!("cannot open file {p}: {e} (file-list)"))?;
					io::copy(&mut f, &mut io::stdout().lock())?;
					Ok(())
				}
				_ => Err(anyhow!("cannot print multiple files (file-list)")),
			};
		} else {
			let is_dir = match fs::metadata(p) {
				Ok(md) => md.file_type().is_dir(),
				Err(e) if e.kind() == ErrorKind::NotFound => false,
				Err(e) => bail!("cannot stat {p}: {e} (file-list)"),
			};

			if is_dir {
				fs_extra::copy_items(
					&files,
					p,
					&CopyOptions::new().overwrite(true).copy_inside(true),
				)
				.map_err(|e| anyhow!("failed to copy items: {e} (file-list)"))?;

				return Ok(());
			}

			return match files.as_slice() {
				[] => Err(anyhow!("there are no files in the clipboard (file-list)")),
				[file] => fs_extra::copy_items(
					&files,
					p,
					&CopyOptions::new().overwrite(true).copy_inside(true),
				)
				.map(|_| ())
				.map_err(|e| anyhow!("error copying {file} to {p}: {e} (file-list)")),
				_ => Err(anyhow!("cannot copy multiple files into a single file")),
			};
		}
	}

	if Path::new(p).is_dir() {
		bail!("cannot write text contents to a directory");
	}

	let mut buf = String::with_capacity(4 << 10);
	let _ = formats::Unicode.read_clipboard(&mut buf);
	if p == "-" {
		if buf.ends_with('\n') {
			buf.pop();
		}

		println!("{buf}");
	} else {
		fs::write(p, &buf).map_err(|e| anyhow!("error writing to {p}: {e}"))?;
	}

	Ok(())
}

fn list_clip() -> Result<()> {
	let _c = open_clip()?;
	let mut files = Vec::<String>::with_capacity(64);
	let _ = formats::FileList.read_clipboard(&mut files);
	for f in files {
		println!("{f}");
	}

	Ok(())
}

fn open_clip() -> clipboard_win::SysResult<Clipboard> {
	Clipboard::new_attempts(50)
}

fn main() {
	if let Err(e) = Cmd::parse().run() {
		eprintln!("error: {e:?}");
		process::exit(1);
	}
}
