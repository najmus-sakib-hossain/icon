use walkdir::WalkDir;
use std::collections::HashMap;
use std::path::PathBuf;

/// Reader for icon-sets based icons (JSON sources)
pub struct IconSetsReader {
    icons: HashMap<String, IconSetData>,
}

pub struct IconSetData {
    pub prefix: String,
    pub name: String,
    pub total: u32,
    pub version: String,
}

impl IconSetsReader {
    pub fn new() -> Self {
        // For now, return empty reader
        // Later we'll load from embedded binaries
        IconSetsReader {
            icons: HashMap::new(),
        }
    }

    pub fn get(&self, id: &str) -> Option<&IconSetData> {
        self.icons.get(id)
    }

    pub fn list_sets(&self) -> Vec<&str> {
        self.icons.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for IconSetsReader {
    fn default() -> Self {
        Self::new()
    }
}

/// Reader for SVGL icons (SVG file sources)
pub struct SvglReader {
    icons: HashMap<String, SvgIconData>,
}

pub struct SvgIconData {
    pub id: String,
    pub filename: String,
    pub svg_content: String,
}

impl SvglReader {
    pub fn new() -> Self {
        // For now, return empty reader
        // We'll implement actual loading from embedded binaries
        SvglReader {
            icons: HashMap::new(),
        }
    }

    /// Load icons from the svgl directory for testing/building
    pub fn from_directory(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        use crate::converters::svgl::SvgIcon;
        
        let mut icons = HashMap::new();
        
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("svg"))
        {
            match SvgIcon::from_file(entry.path()) {
                Ok(icon) => {
                    let id = icon.filename.clone();
                    icons.insert(
                        id.clone(),
                        SvgIconData {
                            id: id.clone(),
                            filename: icon.filename,
                            svg_content: icon.svg_content,
                        },
                    );
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", entry.path().display(), e);
                }
            }
        }
        
        Ok(SvglReader { icons })
    }

    pub fn get(&self, id: &str) -> Option<&SvgIconData> {
        self.icons.get(id)
    }

    pub fn list_icons(&self) -> Vec<&str> {
        self.icons.keys().map(|s| s.as_str()).collect()
    }

    pub fn count(&self) -> usize {
        self.icons.len()
    }
}

impl Default for SvglReader {
    fn default() -> Self {
        Self::new()
    }
}
