use dx_icon::converters::iconsets::IconSetJson;
use dx_icon::converters::svgl::SvgIcon;
use std::path::PathBuf;

fn main() {
    println!("=== Framework Code Generation Test ===\n");

    // 1. Test SVGL Conversion
    println!("1. Testing SVGL Conversion...");
    let svgl_path = PathBuf::from("inspirations/svgl/static/library/react_light.svg");
    if svgl_path.exists() {
        match SvgIcon::from_file(&svgl_path) {
            Ok(icon) => {
                println!("   ✓ Loaded SVGL icon: {}", icon.filename);
                
                // React
                let react = icon.to_react_component(true);
                println!("\n   [React Component Preview]");
                println!("   {}", react.lines().take(10).collect::<Vec<_>>().join("\n   "));
                println!("   ...");

                // Vue
                let vue = icon.to_vue_component(true);
                println!("\n   [Vue Component Preview]");
                println!("   {}", vue.lines().take(10).collect::<Vec<_>>().join("\n   "));
                println!("   ...");

                // Svelte
                let svelte = icon.to_svelte_component(true);
                println!("\n   [Svelte Component Preview]");
                println!("   {}", svelte.lines().take(10).collect::<Vec<_>>().join("\n   "));
                println!("   ...");

                // React Native
                let rn = icon.to_react_native_component(true);
                println!("\n   [React Native Component Preview]");
                println!("   {}", rn.lines().take(10).collect::<Vec<_>>().join("\n   "));
                println!("   ...");

                // Qwik
                let qwik = icon.to_qwik_component(true);
                println!("\n   [Qwik Component Preview]");
                println!("   {}", qwik.lines().take(10).collect::<Vec<_>>().join("\n   "));
                println!("   ...");

                // Solid
                let solid = icon.to_solid_component(true);
                println!("\n   [Solid Component Preview]");
                println!("   {}", solid.lines().take(10).collect::<Vec<_>>().join("\n   "));
                println!("   ...");

                // Astro
                let astro = icon.to_astro_component();
                println!("\n   [Astro Component Preview]");
                println!("   {}", astro.lines().take(10).collect::<Vec<_>>().join("\n   "));
                println!("   ...");
            }
            Err(e) => println!("   ✗ Failed to load SVGL icon: {}", e),
        }
    } else {
        println!("   ⚠ SVGL test file not found at {:?}", svgl_path);
    }

    // 2. Test IconSet Conversion
    println!("\n2. Testing IconSet Conversion...");
    let json_path = PathBuf::from("inspirations/icon-sets/json/ant-design.json");
    if json_path.exists() {
        match IconSetJson::from_file(&json_path) {
            Ok(iconset) => {
                println!("   ✓ Loaded IconSet: {}", iconset.info.name);
                
                if let Some((name, data)) = iconset.icons.iter().next() {
                    println!("   ✓ Testing icon: {}", name);
                    
                    // React
                    let react = data.to_react_component(name, 32, 32, true);
                    println!("\n   [React Component Preview]");
                    println!("   {}", react.lines().take(10).collect::<Vec<_>>().join("\n   "));
                    println!("   ...");

                    // Vue
                    let vue = data.to_vue_component(32, 32, true);
                    println!("\n   [Vue Component Preview]");
                    println!("   {}", vue.lines().take(10).collect::<Vec<_>>().join("\n   "));
                    println!("   ...");

                    // React Native
                    let rn = data.to_react_native_component(name, 32, 32, true);
                    println!("\n   [React Native Component Preview]");
                    println!("   {}", rn.lines().take(10).collect::<Vec<_>>().join("\n   "));
                    println!("   ...");

                    // Qwik
                    let qwik = data.to_qwik_component(name, 32, 32, true);
                    println!("\n   [Qwik Component Preview]");
                    println!("   {}", qwik.lines().take(10).collect::<Vec<_>>().join("\n   "));
                    println!("   ...");

                    // Solid
                    let solid = data.to_solid_component(name, 32, 32, true);
                    println!("\n   [Solid Component Preview]");
                    println!("   {}", solid.lines().take(10).collect::<Vec<_>>().join("\n   "));
                    println!("   ...");

                    // Astro
                    let astro = data.to_astro_component(32, 32);
                    println!("\n   [Astro Component Preview]");
                    println!("   {}", astro.lines().take(10).collect::<Vec<_>>().join("\n   "));
                    println!("   ...");
                }
            }
            Err(e) => println!("   ✗ Failed to load IconSet: {}", e),
        }
    } else {
        println!("   ⚠ IconSet test file not found at {:?}", json_path);
    }
}
