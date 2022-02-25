use crate::FileSize;
use colored::*;

const KBYTES:u64 = 1024;
const MBYTES:u64 = 1048576;
const GBYTES:u64 = 1073741824;

pub fn print_headers() {
  println!("DISK\tBYTES\tPATH");
}

pub fn print_size(results: &Vec<FileSize>, no_colors: bool) {
  for result in results {
    let output = format!("{}\t{}\t{}", human_size(result.disk), human_size(result.bytes), result.path);

    if no_colors {
      println!("{}", output);
    } else {
      println!("{}", colorize_size(output, result.bytes));
    }
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

fn colorize_size(str: String, size: u64) -> String {
  let colored_string: ColoredString;

  if size > GBYTES {
    colored_string = str.red();
  } else if size > 512 * MBYTES {
    colored_string = str.yellow();
  } else if size > 128 * MBYTES {
    colored_string = str.magenta();
  } else if size > MBYTES {
    colored_string = str.cyan();
  } else {
    colored_string = str.green();
  }

  return colored_string.to_string();
}
