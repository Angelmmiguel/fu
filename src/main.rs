// External
use clap::Parser;
use colored::*;
use glob::glob;

// System
use std::process::exit;

// Modules
mod logger;
mod size;

use crate::logger::{print_headers, print_size};
use crate::size::{entry_size, FileSize};

/// Retrieve the size and disk usage of files and directories
#[derive(Parser)]
#[command(version = env!("FU_VERSION"))]
struct Cli {
    /// The path to the file to read
    #[arg()]
    path: std::path::PathBuf,

    /// Sort the output based on the size
    #[arg(long, short)]
    sort: bool,

    /// Hide the headers from the output
    #[arg(long)]
    no_header: bool,

    /// Disable the colors in the output
    #[arg(long)]
    no_colors: bool,

    /// Sort and limit the output to the N heaviest entries
    #[arg(long, short)]
    top: Option<usize>,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let mut results: Vec<FileSize> = vec![];

    // Decide which logic to follow
    let args_str = args.path.to_string_lossy();

    if args_str.contains('*') {
        // Glob
        for entry in glob(&args_str).expect("The glob pattern is invalid") {
            match entry {
                Ok(path) => match entry_size(&path) {
                    Ok(size) => results.push(size),
                    Err(e) => {
                        eprintln!("{}", e.to_string().red());
                        exit(1);
                    }
                },
                Err(e) => println!("{e:?}"),
            }
        }
    } else if args_str.ends_with('/') {
        // Use glob too
        let args_with_glob = format!("{args_str}*");
        // List directory
        for entry in glob(&args_with_glob).expect("The glob pattern is invalid") {
            match entry {
                Ok(path) => match entry_size(&path) {
                    Ok(size) => results.push(size),
                    Err(e) => {
                        eprintln!("{}", e.to_string().red());
                        exit(1);
                    }
                },
                Err(e) => println!("{e:?}"),
            }
        }
    } else {
        // Single file
        match entry_size(&args.path) {
            Ok(size) => results.push(size),
            Err(e) => {
                eprintln!("{}", e.to_string().red());
                exit(1);
            }
        }
    }

    // Output
    if !args.no_header {
        print_headers()
    }

    if args.sort || args.top.is_some() {
        results.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    }

    if let Some(num) = args.top {
        results.truncate(num);
    }

    print_size(&results, args.no_colors);

    Ok(())
}
