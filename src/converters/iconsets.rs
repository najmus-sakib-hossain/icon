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
        use crate::icon_generated::dx_icon::{
            Icon, IconArgs, IconInfo, IconInfoArgs, IconSet, IconSetArgs, 
            Author as FbAuthor, AuthorArgs, License as FbLicense, LicenseArgs
        };
        use flatbuffers::FlatBufferBuilder;

        let mut builder = FlatBufferBuilder::new();

        // Create Author
        let author = if let Some(a) = &self.info.author {
            let name = builder.create_string(&a.name);
            let url = a.url.as_ref().map(|u| builder.create_string(u));
            Some(FbAuthor::create(&mut builder, &AuthorArgs {
                name: Some(name),
                url,
            }))
        } else {
            None
        };

        // Create License
        let license = if let Some(l) = &self.info.license {
            let title = builder.create_string(&l.title);
            let spdx = builder.create_string(&l.spdx);
            let url = l.url.as_ref().map(|u| builder.create_string(u));
            Some(FbLicense::create(&mut builder, &LicenseArgs {
                title: Some(title),
                spdx: Some(spdx),
                url,
            }))
        } else {
            None
        };

        // Create IconInfo
        let name = builder.create_string(&self.info.name);
        let version = self.info.version.as_ref().map(|v| builder.create_string(v));
        let category = self.info.category.as_ref().map(|c| builder.create_string(c));
        
        let info = IconInfo::create(&mut builder, &IconInfoArgs {
            name: Some(name),
            total: self.info.total,
            version,
            author,
            license,
            height: self.info.height.unwrap_or(16),
            category,
            palette: self.info.palette.unwrap_or(false),
        });

        // Create Icons
        let mut icons_vec = Vec::new();
        // Sort keys for deterministic output
        let mut keys: Vec<&String> = self.icons.keys().collect();
        keys.sort();
        
        for key in keys {
            let value = &self.icons[key];
            let id = builder.create_string(key);
            let body = builder.create_string(&value.body);
            let icon = Icon::create(&mut builder, &IconArgs {
                id: Some(id),
                body: Some(body),
                width: value.width.unwrap_or(0),
                height: value.height.unwrap_or(0),
            });
            icons_vec.push(icon);
        }
        let icons = builder.create_vector(&icons_vec);

        // Create IconSet
        let prefix = builder.create_string(&self.prefix);
        let icon_set = IconSet::create(&mut builder, &IconSetArgs {
            prefix: Some(prefix),
            info: Some(info),
            icons: Some(icons),
        });

        builder.finish(icon_set, None);
        builder.finished_data().to_vec()
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
