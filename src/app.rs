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
	.long_about("Manage the system clipboard.\nCall bare to print the contents of the clipboard instead.")
	.setting(AppSettings::UnifiedHelpMessage);

	let clear = Arg::new("clear")
		.short('x')
		.long("clear")
		.about("Clear the contents of the clipboard.")
		.conflicts_with("file");

	let file = Arg::new("file").about("Copy the contents of a file to the clipboard.");

	app.arg(clear).arg(file)
}
