extern crate napi_build;

use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
  let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

  // Define the path to the C dependency source
  let c_dep_path = "hashtree";

  // Change to the directory containing the C dependency and its Makefile
  assert!(env::set_current_dir(&PathBuf::from(c_dep_path)).is_ok());

  // make
  Command::new("make")
      .status()
      .unwrap_or_else(|e| panic!("Failed to execute make: {}", e));

  // make install
  Command::new("make")
    .arg("install")
    .arg(format!("DESTDIR={}",out_dir ))
    .status()
    .unwrap_or_else(|e| panic!("Failed to execute make install: {}", e));

  // Specify the path to the generated library so Rust can link it
  println!("cargo:rustc-link-search=native={}/usr/lib", out_dir);
  println!("cargo:rustc-link-lib=static=hashtree");

  // setup napi
  napi_build::setup();
}
