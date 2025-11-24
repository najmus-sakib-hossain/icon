use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::converters::utils::{svg_to_jsx, to_pascal_case, svg_to_react_native, svg_to_qwik, svg_to_solid, svg_to_astro};

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
    pub width: Option<f32>,
    #[serde(default)]
    pub height: Option<f32>,
}

impl IconData {
    #[allow(dead_code)]
    pub fn to_svg(&self, default_width: u32, default_height: u32) -> String {
        let w = self.width.unwrap_or(default_width as f32);
        let h = self.height.unwrap_or(default_height as f32);
        format!(
            "<svg viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\" fill=\"currentColor\">{}</svg>",
            w, h, w, h, self.body
        )
    }

    #[allow(dead_code)]
    pub fn to_react_component(&self, name: &str, default_width: u32, default_height: u32, typescript: bool) -> String {
        let name = to_pascal_case(name);
        let svg = self.to_svg(default_width, default_height);
        let jsx = svg_to_jsx(&svg);
        
        if typescript {
            format!(
                "import * as React from 'react';\n\n\
                const {} = (props: React.SVGProps<SVGSVGElement>) => (\n\
                {}\n\
                );\n\n\
                export default {};",
                name, jsx, name
            )
        } else {
            format!(
                "import * as React from 'react';\n\n\
                const {} = (props) => (\n\
                {}\n\
                );\n\n\
                export default {};",
                name, jsx, name
            )
        }
    }

    #[allow(dead_code)]
    pub fn to_vue_component(&self, default_width: u32, default_height: u32, typescript: bool) -> String {
        let svg = self.to_svg(default_width, default_height);
        let script_lang = if typescript { " lang=\"ts\"" } else { "" };

        format!(
            "<script setup{}>\n</script>\n\n\
            <template>\n\
            {}\n\
            </template>",
            script_lang, svg
        )
    }

    #[allow(dead_code)]
    pub fn to_svelte_component(&self, default_width: u32, default_height: u32, typescript: bool) -> String {
        let svg = self.to_svg(default_width, default_height);
        let script_lang = if typescript { " lang=\"ts\"" } else { "" };

        format!(
            "<script{}>\n</script>\n\n\
            {}",
            script_lang, svg
        )
    }

    #[allow(dead_code)]
    pub fn to_react_native_component(&self, name: &str, default_width: u32, default_height: u32, _typescript: bool) -> String {
        let name = to_pascal_case(name);
        let svg = self.to_svg(default_width, default_height);
        svg_to_react_native(&svg, &name, false)
    }

    #[allow(dead_code)]
    pub fn to_qwik_component(&self, name: &str, default_width: u32, default_height: u32, _typescript: bool) -> String {
        let name = to_pascal_case(name);
        let svg = self.to_svg(default_width, default_height);
        svg_to_qwik(&svg, &name, false)
    }

    #[allow(dead_code)]
    pub fn to_solid_component(&self, name: &str, default_width: u32, default_height: u32, _typescript: bool) -> String {
        let name = to_pascal_case(name);
        let svg = self.to_svg(default_width, default_height);
        svg_to_solid(&svg, &name, false)
    }

    #[allow(dead_code)]
    pub fn to_astro_component(&self, default_width: u32, default_height: u32) -> String {
        let svg = self.to_svg(default_width, default_height);
        svg_to_astro(&svg)
    }
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
                width: value.width.unwrap_or(0.0) as u32,
                height: value.height.unwrap_or(0.0) as u32,
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
