use std::fs;
use filesize::PathExt;

// Result for a size calculation
pub struct FileSize {
  // Real bytes
  pub bytes: u64,
  // Size on disk
  pub disk: u64
}

pub fn entry_size(path: &std::path::PathBuf) -> std::io::Result<FileSize> {
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