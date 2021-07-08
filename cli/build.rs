// This build script depends on `alan` being in the $PATH and compiles the anycloud.ln file so it
// can be included as raw data within the anycloud binary
use std::process::Command;

fn main() {
  // Tell Cargo that if the anycloud.ln or alan-comple files change, rerun this build script
  println!("cargo:rerun-if-changed=alan/anycloud.ln");
  println!("cargo:rerun-if-changed=../../compiler/alan-compile");
  Command::new("sh")
    .arg("-c")
    .arg("alan compile anycloud.ln anycloud.agz")
    .output()
    .unwrap();
}
