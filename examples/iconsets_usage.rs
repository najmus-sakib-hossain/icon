use dx_icon::converters::iconsets::IconSetJson;
use walkdir::WalkDir;
use std::path::PathBuf;

fn main() {
    println!("=== Icon-Sets Example ===\n");

    let iconsets_path = PathBuf::from("inspirations/icon-sets/json");
    
    println!("Loading icon-sets from: {}\n", iconsets_path.display());

    let mut total_sets = 0;
    let mut total_icons = 0;
    let mut example_sets = Vec::new();

    // Load all icon-sets
    for entry in WalkDir::new(&iconsets_path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
    {
        match IconSetJson::from_file(entry.path()) {
            Ok(iconset) => {
                total_sets += 1;
                total_icons += iconset.icons.len();
                
                if example_sets.len() < 5 {
                    example_sets.push((
                        iconset.info.name.clone(),
                        iconset.prefix.clone(),
                        iconset.icons.len(),
                        iconset.info.version.clone(),
                    ));
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse {}: {}", entry.path().display(), e);
            }
        }
    }

    println!("✓ Loaded {} icon sets with {} total icons\n", total_sets, total_icons);

    // Show example sets
    println!("Example Icon Sets:");
    for (i, (name, prefix, count, version)) in example_sets.iter().enumerate() {
        println!(
            "  {}. {} (prefix: {}, icons: {}, version: {})",
            i + 1,
            name,
            prefix,
            count,
            version.as_deref().unwrap_or("N/A")
        );
    }

    // Show detailed example from one specific set
    println!("\n--- Detailed Example: ant-design.json ---");
    let antd_path = iconsets_path.join("ant-design.json");
    
    if let Ok(iconset) = IconSetJson::from_file(&antd_path) {
        println!("Name: {}", iconset.info.name);
        println!("Prefix: {}", iconset.prefix);
        println!("Total Icons: {}", iconset.info.total);
        println!("Version: {}", iconset.info.version.as_deref().unwrap_or("N/A"));
        
        if let Some(author) = &iconset.info.author {
            println!("Author: {}", author.name);
        }
        
        if let Some(license) = &iconset.info.license {
            println!("License: {} ({})", license.title, license.spdx);
        }

        // Show first few icons
        println!("\nFirst 5 icons:");
        let mut icon_names: Vec<_> = iconset.icons.keys().collect();
        icon_names.sort();
        
        for (i, icon_name) in icon_names.iter().take(5).enumerate() {
            if let Some(icon_data) = iconset.icons.get(*icon_name) {
                println!("  {}. {}", i + 1, icon_name);
                println!("     Body length: {} chars", icon_data.body.len());
                if let Some(width) = icon_data.width {
                    println!("     Width: {}px", width);
                }
                if let Some(height) = icon_data.height {
                    println!("     Height: {}px", height);
                }
            }
        }
    } else {
        println!("Could not load ant-design.json for detailed example");
    }

    println!("\n--- Statistics ---");
    println!("Total icon sets: {}", total_sets);
    println!("Total icons across all sets: {}", total_icons);
    if total_sets > 0 {
        println!("Average icons per set: {}", total_icons / total_sets);
    }
}
