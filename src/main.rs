use atty::Stream;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::error::Error;
use std::fs;
use std::io::{self, Read};
use std::process;

fn print_clip() -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    print!(
        "{}",
        ctx.get_contents().expect("could not access the clipboard")
    );
    Ok(())
}

fn set_clip(s: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(s.to_owned())
}

fn show_help() {
    eprintln!(
        "rs-clip, manage the system clipboard
copy:
	pipe the output of a command to rs-clip
	`cat main.rs | rs-clip`
paste:
	just call rs-clip bare, the contents of the clipboard will be printed
copy contents of a file:
	call rs-clip with a filename as the first argument
	`rs-clip main.rs`"
    );
    process::exit(0);
}

fn main() {
    // check if stdin is piped
    if !atty::is(Stream::Stdin) {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle
            .read_to_string(&mut buffer)
            .expect("failed to read the stdin");
        set_clip(&buffer[..]).unwrap_or_else(|e| {
            eprintln!("error writing to clipboard: {:?}", e);
            process::exit(1);
        });
        return;
    }
    let args: Vec<_> = std::env::args().collect();
    if args.len() <= 1 {
        print_clip().unwrap_or_else(|e| {
            eprintln!("error accessing the clipboard: {:?}", e);
            process::exit(1);
        });
        return;
    }
    // parse args
    let arg = &args[1];
    if arg == "-h" || arg == "--help" {
        show_help();
        return;
    }
    match fs::read_to_string(arg) {
        Err(e) => {
            eprintln!("error opening file {}:\n{:?}", &arg, &e);
            process::exit(1);
        }
        Ok(s) => {
            set_clip(&s[..]).unwrap_or_else(|e| {
                eprintln!("error writing to clipboard: {:?}", e);
                process::exit(1);
            });
        }
    };
}
