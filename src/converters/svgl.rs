use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;
use std::path::Path;

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
        reader.trim_text(true);

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
