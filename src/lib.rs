pub mod converters;
pub mod readers;

// Include generated FlatBuffers code
#[allow(dead_code, unused_imports)]
#[path = "icon_generated.rs"]
pub mod icon_generated;

// Re-export commonly used types
pub use readers::{IconSetsReader, SvglReader};

/// Initialize and get access to icon-sets library
pub fn icon_sets() -> IconSetsReader {
    IconSetsReader::new()
}

/// Initialize and get access to svgl library
pub fn svgl() -> SvglReader {
    SvglReader::new()
}
