// This build script depends on `alan` being in the $PATH and compiles the anycloud.ln file so it
// can be included as raw data within the anycloud binary
use std::process::Command;

fn main() {
  // Tell Cargo that if the anycloud.ln file changes, rerun this build script
  println!("cargo:rerun-if-changed=alan/anycloud.ln");
  let output = Command::new("sh")
    .arg("-c")
    .arg("cd alan && alan compile anycloud.ln anycloud.agz")
    .output()
    .unwrap();

  std::process::exit(output.status.code().unwrap());
}