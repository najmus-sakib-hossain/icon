pub mod converters;
pub mod readers;

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
