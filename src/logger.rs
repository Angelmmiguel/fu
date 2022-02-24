use crate::FileSize;

const KBYTES:u64 = 1024;
const MBYTES:u64 = 1048576;
const GBYTES:u64 = 1073741824;

pub fn print_headers() {
  println!("DISK\tBYTES\tPATH");
}

pub fn print_size(results: &Vec<FileSize>) {
  for result in results {
    println!("{}\t{}\t{}", human_size(result.disk), human_size(result.bytes), result.path);
  }
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
