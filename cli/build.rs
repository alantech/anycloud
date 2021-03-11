// This build script depends on `alan` being in the $PATH and compiles the anycloud.ln file so it
// can be included as raw data within the anycloud binary
use std::process::Command;

fn main() {
  //Get alan version
  Command::new("sh")
    .arg("-c")
    .arg("cd alan && alan --version | sed -e 's/alan //' > alan-version")
    .output()
    .unwrap();

  // Tell Cargo that if the anycloud.ln file changes, rerun this build script
  println!("cargo:rerun-if-changed=alan/anycloud.ln");
  let output = Command::new("sh")
    .arg("-c")
    .arg("cd alan && alan compile anycloud.ln anycloud.agz")
    .output()
    .unwrap();

  if output.status.code().unwrap() != 0 {
    // The `alan` command doesn't exist on this machine. We'll guarantee that it does for building
    // the `anycloud` binary, but for those using this repo as a library, we'll just bypass this
    // piece since only `main.rs` uses the `anycloud.agz` file.
    Command::new("sh").arg("-c").arg("cd alan && touch anycloud.agz");
  }
}