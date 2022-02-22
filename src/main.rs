// External
use clap::Parser;

// Modules
mod logger;
mod size;

use crate::size::{
    entry_size,
    FileSize
};
use crate::logger::print_size;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let calc = entry_size(&args.path)?;
    print_size(&args.path, calc);

    Ok(())
}
