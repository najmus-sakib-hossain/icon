use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;
use std::path::Path;
use crate::converters::utils::{extract_styles, svg_to_jsx, to_pascal_case, svg_to_react_native, svg_to_qwik, svg_to_solid, svg_to_astro};

#[derive(Debug, Clone)]
pub struct SvgIcon {
    pub filename: String,
    pub svg_content: String,
    pub viewbox: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl SvgIcon {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path_ref = path.as_ref();
        let filename = path_ref
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let svg_content = fs::read_to_string(path_ref)?;
        
        // Parse SVG to extract viewBox and dimensions
        let mut reader = Reader::from_str(&svg_content);
        reader.config_mut().trim_text(true);

        let mut viewbox = None;
        let mut width = None;
        let mut height = None;

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) if e.name().as_ref() == b"svg" => {
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"viewBox" => {
                                    viewbox = Some(
                                        String::from_utf8_lossy(attr.value.as_ref()).to_string(),
                                    );
                                }
                                b"width" => {
                                    let w_str = String::from_utf8_lossy(attr.value.as_ref());
                                    // Try to parse, removing "px" suffix if present
                                    width = w_str
                                        .trim_end_matches("px")
                                        .parse::<u32>()
                                        .ok();
                                }
                                b"height" => {
                                    let h_str = String::from_utf8_lossy(attr.value.as_ref());
                                    height = h_str
                                        .trim_end_matches("px")
                                        .parse::<u32>()
                                        .ok();
                                }
                                _ => {}
                            }
                        }
                    }
                    break;
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    eprintln!("Error parsing SVG {}: {:?}", filename, e);
                    break;
                }
                _ => {}
            }
            buf.clear();
        }

        Ok(SvgIcon {
            filename,
            svg_content,
            viewbox,
            width,
            height,
        })
    }

    #[allow(dead_code)]
    pub fn to_react_component(&self, typescript: bool) -> String {
        let name = to_pascal_case(&self.filename);
        let jsx = svg_to_jsx(&self.svg_content);
        
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
    pub fn to_vue_component(&self, typescript: bool) -> String {
        let (template, styles) = extract_styles(&self.svg_content);
        let script_lang = if typescript { " lang=\"ts\"" } else { "" };
        let style_block = if !styles.is_empty() {
            format!("\n<style scoped>\n{}\n</style>", styles)
        } else {
            String::new()
        };

        format!(
            "<script setup{}>\n</script>\n\n\
            <template>\n\
            {}\n\
            </template>\n\
            {}",
            script_lang, template, style_block
        )
    }

    #[allow(dead_code)]
    pub fn to_svelte_component(&self, typescript: bool) -> String {
        let (template, styles) = extract_styles(&self.svg_content);
        let script_lang = if typescript { " lang=\"ts\"" } else { "" };
        let style_block = if !styles.is_empty() {
            format!("\n<style>\n{}\n</style>", styles)
        } else {
            String::new()
        };

        format!(
            "<script{}>\n</script>\n\n\
            {}\n\
            {}",
            script_lang, template, style_block
        )
    }

    #[allow(dead_code)]
    pub fn to_react_native_component(&self, _typescript: bool) -> String {
        let name = to_pascal_case(&self.filename);
        svg_to_react_native(&self.svg_content, &name, false)
    }

    #[allow(dead_code)]
    pub fn to_qwik_component(&self, _typescript: bool) -> String {
        let name = to_pascal_case(&self.filename);
        svg_to_qwik(&self.svg_content, &name, false)
    }

    #[allow(dead_code)]
    pub fn to_solid_component(&self, _typescript: bool) -> String {
        let name = to_pascal_case(&self.filename);
        svg_to_solid(&self.svg_content, &name, false)
    }

    #[allow(dead_code)]
    pub fn to_astro_component(&self) -> String {
        svg_to_astro(&self.svg_content)
    }

    pub fn build_collection(icons: &[SvgIcon]) -> Vec<u8> {
        use crate::icon_generated::dx_icon::{SvglIcon, SvglIconArgs, SvglCollection, SvglCollectionArgs};
        use flatbuffers::FlatBufferBuilder;

        let mut builder = FlatBufferBuilder::new();
        
        let mut fb_icons = Vec::new();
        // Sort by filename for determinism
        let mut sorted_icons: Vec<&SvgIcon> = icons.iter().collect();
        sorted_icons.sort_by(|a, b| a.filename.cmp(&b.filename));

        for icon in sorted_icons {
            let id = builder.create_string(&icon.filename);
            let filename = builder.create_string(&icon.filename);
            let svg_content = builder.create_string(&icon.svg_content);
            let viewbox = icon.viewbox.as_ref().map(|v| builder.create_string(v));
            
            let fb_icon = SvglIcon::create(&mut builder, &SvglIconArgs {
                id: Some(id),
                filename: Some(filename),
                svg_content: Some(svg_content),
                viewbox,
                width: icon.width.unwrap_or(0),
                height: icon.height.unwrap_or(0),
            });
            fb_icons.push(fb_icon);
        }
        
        let icons_vec = builder.create_vector(&fb_icons);
        let collection = SvglCollection::create(&mut builder, &SvglCollectionArgs {
            icons: Some(icons_vec),
        });
        
        builder.finish(collection, None);
        builder.finished_data().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_svg() {
        let svg = r#"<svg width="100" height="100" viewBox="0 0 100 100"><path d="M10 10"/></svg>"#;
        
        // Create a temp file for testing
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test.svg");
        std::fs::write(&temp_file, svg).unwrap();
        
        let icon = SvgIcon::from_file(&temp_file).unwrap();
        assert_eq!(icon.filename, "test");
        assert_eq!(icon.width, Some(100));
        assert_eq!(icon.height, Some(100));
        assert_eq!(icon.viewbox, Some("0 0 100 100".to_string()));
        
        std::fs::remove_file(temp_file).ok();
    }
}
