use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=schema/icon.fbs");
    println!("cargo:rerun-if-changed=inspirations/icon-sets/json");
    println!("cargo:rerun-if-changed=inspirations/svgl/static/library");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let schema_path = Path::new("schema/icon.fbs");

    // Compile FlatBuffers schema
    let status = Command::new("flatc")
        .args(&[
            "--rust",
            "-o",
            out_dir.to_str().unwrap(),
            schema_path.to_str().unwrap(),
        ])
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("cargo:warning=FlatBuffers schema compiled successfully");
        }
        Ok(status) => {
            panic!("flatc failed with status: {:?}", status);
        }
        Err(e) => {
            eprintln!("Warning: flatc not found or failed to run: {}", e);
            eprintln!("Make sure flatc is installed and in PATH");
            eprintln!("Install from: https://github.com/google/flatbuffers/releases");
            panic!("flatc is required to build this project");
        }
    }

    // Create output directory for icon binaries
    let icons_dir = out_dir.join("icons");
    fs::create_dir_all(&icons_dir).expect("Failed to create icons directory");

    println!("cargo:warning=Icon binaries will be generated at: {}", icons_dir.display());
    
    // Note: The actual conversion will be done by separate modules
    // We'll include the generated FlatBuffers code
    println!("cargo:warning=Build script completed. Generated files in {}", out_dir.display());
}
