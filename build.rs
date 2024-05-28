use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let hashtree_dir = PathBuf::from(&manifest_dir).join("hashtree");
  let out_dir = PathBuf::from(&hashtree_dir).join("build");
  let lib_dir = PathBuf::from(&hashtree_dir).join("lib");

  // make
  Command::new("make")
    .current_dir(&hashtree_dir)
    .env("OUT_DIR", &out_dir) // Pass OUT_DIR to makefile if needed
    .status()
    .unwrap_or_else(|e| panic!("Failed to execute make: {}", e));

  // Specify the path to the generated library so Rust can link it
  println!("cargo:rustc-link-search=native={}", lib_dir.display());
  println!("cargo:rustc-link-lib=static=hashtree");

  // setup napi
  napi_build::setup();
}
