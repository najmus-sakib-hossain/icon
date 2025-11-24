use dx_icon::{converters::iconsets::IconSetJson, SvglReader};
use std::path::PathBuf;

fn main() {
    println!("=== dx-icon: FlatBuffers Icon Library ===\n");
    
    // Example 1: SVGL Icons
    println!("1. SVGL Icons (SVG files)");
    println!("   Run: cargo run --example svgl_usage\n");
    
    let svgl_path = PathBuf::from("inspirations/svgl/static/library");
    match SvglReader::from_directory(svgl_path) {
        Ok(reader) => {
            println!("   ✓ {} SVGL icons available", reader.count());
        }
        Err(e) => {
            println!("   ✗ Error: {}", e);
        }
    }
    
    // Example 2: Icon-Sets
    println!("\n2. Icon-Sets (JSON collections)");
    println!("   Run: cargo run --example iconsets_usage\n");
    
    let example_path = PathBuf::from("inspirations/icon-sets/json/ant-design.json");
    match IconSetJson::from_file(&example_path) {
        Ok(iconset) => {
            println!("   ✓ Example: {} with {} icons", iconset.info.name, iconset.icons.len());
        }
        Err(e) => {
            println!("   ✗ Error loading example: {}", e);
        }
    }
    
    println!("\n--- Next Steps ---");
    println!("The converters are ready. Next, we'll:");
    println!("• Implement FlatBuffers binary conversion");
    println!("• Generate binary files at build time");
    println!("• Add fast binary lookups");
    println!("\nFor now, run the examples to see icon parsing in action!");
}
