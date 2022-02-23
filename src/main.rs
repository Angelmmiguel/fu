// External
use clap::Parser;

// Modules
mod logger;
mod size;

use crate::size::{
    entry_size,
    FileSize
};
use crate::logger::{
    print_size,
    print_headers
};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,

    /// Hide the headers from the output
    #[clap(long)]
    no_header: bool,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let calc = entry_size(&args.path)?;

    if !args.no_header {
        print_headers()
    }

    print_size(&args.path, calc);

    Ok(())
}
