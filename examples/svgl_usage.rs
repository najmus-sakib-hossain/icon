use dx_icon::SvglReader;
use std::path::PathBuf;

fn main() {
    println!("=== SVGL Icons Example ===\n");

    // Load icons from the svgl directory
    let svgl_path = PathBuf::from("inspirations/svgl/static/library");
    
    match SvglReader::from_directory(svgl_path) {
        Ok(reader) => {
            println!("✓ Loaded {} SVGL icons\n", reader.count());

            // List first 10 icons
            println!("First 10 icons:");
            let mut icons: Vec<_> = reader.list_icons();
            icons.sort();
            
            for (i, icon_id) in icons.iter().take(10).enumerate() {
                println!("  {}. {}", i + 1, icon_id);
            }

            // Show details for a specific icon (if React is available)
            println!("\n--- Icon Details ---");
            if let Some(icon) = reader.get("react_dark").or_else(|| reader.get("react")) {
                println!("ID: {}", icon.id);
                println!("Filename: {}", icon.filename);
                println!("SVG Content Length: {} bytes", icon.svg_content.len());
                println!("\nFirst 200 characters of SVG:");
                println!("{}", &icon.svg_content.chars().take(200).collect::<String>());
            } else if let Some(first_icon_id) = icons.first() {
                if let Some(icon) = reader.get(first_icon_id) {
                    println!("Showing first available icon: '{}'", icon.id);
                    println!("Filename: {}", icon.filename);
                    println!("SVG Content Length: {} bytes", icon.svg_content.len());
                }
            }

            // Show some statistics
            println!("\n--- Statistics ---");
            println!("Total icons: {}", reader.count());
            
            let svg_sizes: Vec<usize> = icons
                .iter()
                .filter_map(|id| reader.get(id))
                .map(|icon| icon.svg_content.len())
                .collect();
            
            if !svg_sizes.is_empty() {
                let avg_size = svg_sizes.iter().sum::<usize>() / svg_sizes.len();
                let min_size = svg_sizes.iter().min().unwrap();
                let max_size = svg_sizes.iter().max().unwrap();
                
                println!("Average icon size: {} bytes", avg_size);
                println!("Smallest icon: {} bytes", min_size);
                println!("Largest icon: {} bytes", max_size);
            }
        }
        Err(e) => {
            eprintln!("✗ Error loading SVGL icons: {}", e);
            eprintln!("\nMake sure you run this from the project root directory.");
        }
    }
}
