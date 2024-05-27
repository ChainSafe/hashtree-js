use std::path::PathBuf;
use std::process::Command;

fn main() {
  // Define the path to the hashtree bindings directory
  let hashtree_bindings_dir = PathBuf::from("hashtree");

  // Run the build script of the hashtree library
  let status = Command::new("cargo")
    .current_dir(hashtree_bindings_dir)
    .args(&["build", "--release"])
    .status()
    .expect("Failed to build hashtree submodule");

  if !status.success() {
    panic!("Failed to build hashtree submodule");
  }

  // setup napi
  napi_build::setup();

  println!("cargo:rerun-if-changed=build.rs");
}
