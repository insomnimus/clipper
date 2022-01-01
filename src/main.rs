mod cmd;

use std::process;

fn main() {
	if let Err(e) = cmd::Cmd::from_args().run() {
		eprintln!("error: {}", e);
		process::exit(2);
	}
}
