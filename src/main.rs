use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use clipb::{format_paths, resolve_paths, VERSION};

#[derive(Parser)]
#[command(
    name = "clipb",
    version = VERSION,
    about = "A lightweight clipboard utility for developers."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Copy file paths to the clipboard and echo them to stdout.
    Copy {
        /// One or more file paths to copy.
        #[arg(required = true, value_name = "PATH")]
        paths: Vec<PathBuf>,
    },
    /// Print the current clipboard contents to stdout.
    Paste,
    /// Clear the clipboard.
    Clear,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Copy { paths } => copy(paths),
        Command::Paste => paste(),
        Command::Clear => clear(),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("clipb: {error}");
            ExitCode::FAILURE
        }
    }
}

fn copy(paths: Vec<PathBuf>) -> Result<(), String> {
    let paths = resolve_paths(&paths);
    let text = format_paths(&paths);
    let mut clipboard =
        arboard::Clipboard::new().map_err(|error| format!("cannot open clipboard: {error}"))?;
    clipboard
        .set_text(text)
        .map_err(|error| format!("cannot write clipboard: {error}"))?;
    for path in &paths {
        println!("{}", path.display());
    }
    Ok(())
}

fn paste() -> Result<(), String> {
    let mut clipboard =
        arboard::Clipboard::new().map_err(|error| format!("cannot open clipboard: {error}"))?;
    let text = clipboard
        .get_text()
        .map_err(|error| format!("cannot read clipboard: {error}"))?;
    println!("{text}");
    Ok(())
}

fn clear() -> Result<(), String> {
    let mut clipboard =
        arboard::Clipboard::new().map_err(|error| format!("cannot open clipboard: {error}"))?;
    clipboard
        .clear()
        .map_err(|error| format!("cannot clear clipboard: {error}"))
}
