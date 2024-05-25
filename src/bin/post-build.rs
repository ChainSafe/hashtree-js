use glob::glob;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
  // Get the root directory of the repository
  let root_dir = env::current_dir().unwrap();
  let root_path = PathBuf::from(&root_dir);

  // Define the prebuild output directory
  let prebuild_dir = root_path.join("prebuild");

  // Ensure the prebuild directory exists
  fs::create_dir_all(&prebuild_dir).expect("Failed to create prebuild directory");

  // Use glob to find the *.node file at the root of the repository
  let pattern = root_path.join("*.node").to_str().unwrap().to_string();
  for entry in glob(&pattern).expect("Failed to read glob pattern") {
    match entry {
      Ok(path) => {
        let file_name = path.file_name().unwrap();
        let destination = prebuild_dir.join(file_name);
        // Move the file to the prebuild directory
        fs::rename(&path, &destination).expect("Failed to move file to prebuild directory");
      }
      Err(e) => println!("{:?}", e),
    }
  }
}
