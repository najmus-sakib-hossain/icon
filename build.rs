use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

// Include generated code
#[allow(dead_code, unused_imports)]
mod icon_generated {
    include!(concat!(env!("OUT_DIR"), "/icon_generated.rs"));
}

// Include converters
#[path = "src/converters/mod.rs"]
mod converters;

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
    
    // Process Icon Sets (JSON)
    let json_dir = Path::new("inspirations/icon-sets/json");
    if json_dir.exists() {
        println!("cargo:warning=Processing JSON icon sets from {}", json_dir.display());
        for entry in WalkDir::new(json_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                match converters::iconsets::IconSetJson::from_file(path) {
                    Ok(iconset) => {
                        let data = iconset.to_flatbuffer();
                        let filename = path.file_stem().unwrap().to_string_lossy();
                        let out_path = icons_dir.join(format!("{}.bin", filename));
                        fs::write(&out_path, data).expect("Failed to write iconset binary");
                    }
                    Err(e) => {
                        println!("cargo:warning=Failed to parse {}: {}", path.display(), e);
                    }
                }
            }
        }
    } else {
        println!("cargo:warning=JSON icon sets directory not found: {}", json_dir.display());
    }

    // Process SVGL Icons (SVG)
    let svgl_dir = Path::new("inspirations/svgl/static/library");
    if svgl_dir.exists() {
        println!("cargo:warning=Processing SVGL icons from {}", svgl_dir.display());
        let mut icons = Vec::new();
        for entry in WalkDir::new(svgl_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "svg") {
                match converters::svgl::SvgIcon::from_file(path) {
                    Ok(icon) => icons.push(icon),
                    Err(e) => {
                        println!("cargo:warning=Failed to parse SVG {}: {}", path.display(), e);
                    }
                }
            }
        }
        
        if !icons.is_empty() {
            let data = converters::svgl::SvgIcon::build_collection(&icons);
            let out_path = icons_dir.join("svgl.bin");
            fs::write(&out_path, data).expect("Failed to write svgl binary");
            println!("cargo:warning=Generated svgl.bin with {} icons", icons.len());
        }
    } else {
        println!("cargo:warning=SVGL directory not found: {}", svgl_dir.display());
    }

    println!("cargo:warning=Build script completed. Generated files in {}", out_dir.display());
}
