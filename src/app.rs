use clap::{
	crate_version,
	App,
	Arg,
	arg,
};

pub fn new() -> App<'static> {
	App::new("clipper")
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
}
