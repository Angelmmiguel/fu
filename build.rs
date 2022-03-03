use std::process::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> std::io::Result<()> {
  // Obtain the current Git commit SHA
  let output = 
    Command::new("git")
      .args(&["rev-parse", "HEAD"])
      .output()
      .unwrap();
  let git_hash = String::from_utf8(output.stdout).unwrap();

  println!("cargo:rustc-env=FU_VERSION={} - Git commit: {}", VERSION, git_hash);

  Ok(())
}
