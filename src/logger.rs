use crate::FileSize;

pub fn print_size(path: &std::path::PathBuf, calc: FileSize) {
  println!("{:?}\tDisk: {:?}\tBytes: {:?}", path, calc.disk, calc.bytes);
}