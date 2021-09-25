use clap::{
	crate_version,
	App,
	AppSettings,
	Arg,
};

pub fn new() -> App<'static> {
	let app = App::new("clipper")
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
		.setting(AppSettings::UnifiedHelpMessage);

	let clear = Arg::new("clear")
		.short('x')
		.long("clear")
		.about("Clear the contents of the clipboard.")
		.conflicts_with("file");

	let file = Arg::new("file").about("Copy the contents of a file to the clipboard.");

	app.arg(clear).arg(file)
}
