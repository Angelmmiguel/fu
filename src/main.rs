// External
use std::fs;
use clap::Parser;
use filesize::PathExt;

// Modules
mod logger;
mod size;

use crate::size::FileSize;
use crate::logger::print_size;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn entry_size(path: &std::path::PathBuf) -> std::io::Result<FileSize> {
    let metadata_entry = fs::metadata(path)?;
    let mut acc = 0;
    let mut disk_acc = 0;

    if metadata_entry.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let inner_path = entry.path();
            if inner_path.is_dir() {
                let inner_entry_size = entry_size(&inner_path)?;
                disk_acc = disk_acc + inner_entry_size.disk;
                acc = acc + inner_entry_size.bytes;
            } else {
                disk_acc = disk_acc + inner_path.size_on_disk()?;
                let metadata = fs::metadata(&inner_path)?;
                acc = acc + metadata.len();
            }
        }
    } else {
        disk_acc = disk_acc + path.size_on_disk()?;
        acc = acc + metadata_entry.len();
    }

    let res = FileSize {
        bytes: acc,
        disk: disk_acc
    };

    Ok(res)
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let calc = entry_size(&args.path)?;
    print_size(&args.path, calc);

    Ok(())
}
