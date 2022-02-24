// External
use clap::Parser;
use glob::glob;

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
    let mut results: Vec<FileSize> = vec![];

    // Decide which logic to follow
    let args_str = args.path.to_string_lossy();

    if args_str.contains("*") {
        // Glob
        for entry in glob(&args_str).expect("The glob pattern is invalid") {
            match entry {
                Ok(path) => results.push(entry_size(&path)?),
                Err(e) => println!("{:?}", e),
            }
        }
    } else if args_str.ends_with("/") {
        // Use glob too
        let args_with_glob = format!("{}*", args_str);
        // List directory
        for entry in glob(&args_with_glob).expect("The glob pattern is invalid") {
            match entry {
                Ok(path) => results.push(entry_size(&path)?),
                Err(e) => println!("{:?}", e),
            }
        }
    } else {
        // Single file
        results.push(entry_size(&args.path)?);
    }

    if !args.no_header {
        print_headers()
    }

    print_size(&results);

    Ok(())
}
