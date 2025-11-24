use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct License {
    pub title: String,
    pub spdx: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IconSetInfo {
    pub name: String,
    pub total: u32,
    pub version: Option<String>,
    pub author: Option<Author>,
    pub license: Option<License>,
    pub height: Option<u32>,
    pub category: Option<String>,
    pub palette: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IconData {
    pub body: String,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IconSetJson {
    pub prefix: String,
    pub info: IconSetInfo,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<u64>,
    pub icons: HashMap<String, IconData>,
}

impl IconSetJson {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let iconset: IconSetJson = serde_json::from_str(&content)?;
        Ok(iconset)
    }

    pub fn to_flatbuffer(&self) -> Vec<u8> {
        // This will be implemented to convert to FlatBuffers binary
        // For now, return empty vec as placeholder
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_icon_json() {
        // Test with sample JSON structure
        let json = r#"{
            "prefix": "test",
            "info": {
                "name": "Test Icons",
                "total": 1,
                "version": "1.0.0"
            },
            "icons": {
                "test-icon": {
                    "body": "<path d='M10 10'/>"
                }
            }
        }"#;
        
        let iconset: IconSetJson = serde_json::from_str(json).unwrap();
        assert_eq!(iconset.prefix, "test");
        assert_eq!(iconset.info.name, "Test Icons");
        assert_eq!(iconset.icons.len(), 1);
    }
}
