mod integration {
  use std::process::Command;

  static WITHOUT_ARGS_OUTPUT: &'static str = "error: The following required arguments were not provided:
    <PATH>

USAGE:
    fu [OPTIONS] <PATH>

For more information try --help
";

  static WITH_FOLDER: &'static str = "DISK\tBYTES\tPATH
8K\t15B\t./tests/example
";

  static WITH_FOLDER_AND_SLASH: &'static str = "DISK\tBYTES\tPATH
4K\t3B\ttests/example/0_test.txt
4K\t12B\ttests/example/1_bigger.txt
";

  static WITH_FOLDER_AND_SORTED: &'static str = "DISK\tBYTES\tPATH
4K\t12B\ttests/example/1_bigger.txt
4K\t3B\ttests/example/0_test.txt
";

  static WITH_FOLDER_AND_TOP: &'static str = "DISK\tBYTES\tPATH
4K\t12B\ttests/example/1_bigger.txt
";

  #[test]
  fn calling_fu_without_args() {
    let output = Command::new("./target/debug/fu")
      .output()
      .expect("failed to execute process");

    assert_eq!(String::from_utf8_lossy(&output.stderr), WITHOUT_ARGS_OUTPUT);
  }

  #[test]
  fn calling_fu_with_folder() {
    let output = Command::new("./target/debug/fu")
      .arg("./tests/example")
      .output()
      .expect("");

    assert_eq!(String::from_utf8_lossy(&output.stdout), WITH_FOLDER);
  }

  #[test]
  fn calling_fu_with_folder_and_slash() {
    let output = Command::new("./target/debug/fu")
      .arg("./tests/example/")
      .output()
      .expect("");

    assert_eq!(String::from_utf8_lossy(&output.stdout), WITH_FOLDER_AND_SLASH);
  }

  #[test]
  fn calling_fu_with_folder_and_sorted() {
    let output = Command::new("./target/debug/fu")
      .arg("--sort")
      .arg("./tests/example/")
      .output()
      .expect("");

    assert_eq!(String::from_utf8_lossy(&output.stdout), WITH_FOLDER_AND_SORTED);
  }

  #[test]
  fn calling_fu_with_folder_and_top() {
    let output = Command::new("./target/debug/fu")
      .arg("--top")
      .arg("1")
      .arg("./tests/example/")
      .output()
      .expect("");

    assert_eq!(String::from_utf8_lossy(&output.stdout), WITH_FOLDER_AND_TOP);
  }
}
