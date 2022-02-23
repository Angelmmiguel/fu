use crate::FileSize;

const KBYTES:u64 = 1024;
const MBYTES:u64 = 1048576;
const GBYTES:u64 = 1073741824;

pub fn print_headers() {
  println!("DISK\tBYTES\tPATH");
}

pub fn print_size(path: &std::path::PathBuf, calc: FileSize) {
  let path_string = String::from(path.to_string_lossy());
  println!("{}\t{}\t{}", human_size(calc.disk), human_size(calc.bytes), path_string);
}

fn human_size(size: u64) -> String {
  if size > GBYTES {
    return format!("{}G", size / GBYTES);
  } else if size > MBYTES {
    return format!("{}M", size / MBYTES);
  } else if size > KBYTES {
    return format!("{}K", size / KBYTES);
  } else {
    return format!("{}B", size);
  }
}
