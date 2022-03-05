use crate::FileSize;
use colored::*;

const KBYTES:u64 = 1024;
const MBYTES:u64 = 1048576;
const GBYTES:u64 = 1073741824;

// Color limits
const RED_LIMIT:u64 = GBYTES;
const YELLOW_LIMIT:u64 = 512 * MBYTES;
const MAGENTA_LIMIT:u64 = 128 * MBYTES;
const CYAN_LIMIT:u64 = MBYTES;

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

  if size > RED_LIMIT {
    colored_string = str.red();
  } else if size > YELLOW_LIMIT {
    colored_string = str.yellow();
  } else if size > MAGENTA_LIMIT {
    colored_string = str.magenta();
  } else if size > CYAN_LIMIT {
    colored_string = str.cyan();
  } else {
    colored_string = str.green();
  }

  return colored_string.to_string();
}

#[cfg(test)]
mod tests {
  use colored::*;
  use crate::logger::*;

  #[test]
  fn test_human_size() {
    assert_eq!(human_size(4 * GBYTES), "4G");
    assert_eq!(human_size(2 * MBYTES), "2M");
    assert_eq!(human_size(3 * KBYTES), "3K");
    assert_eq!(human_size(4), "4B");
  }

  #[test]
  fn test_colorize_size() {
    assert_eq!(
      colorize_size("test".to_string(), RED_LIMIT + 1),
      "test".red().to_string()
    );
    assert_eq!(
      colorize_size("test".to_string(), YELLOW_LIMIT + 1),
      "test".yellow().to_string()
    );
    assert_eq!(
      colorize_size("test".to_string(), MAGENTA_LIMIT + 1),
      "test".magenta().to_string()
    );
    assert_eq!(
      colorize_size("test".to_string(), CYAN_LIMIT + 1),
      "test".cyan().to_string()
    );
  }
}
