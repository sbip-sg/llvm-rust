//! Module provide additional utilities to handle LLVM `MetadataValue`.

use inkwell::values::MetadataValue;

/// Trait providing additional functions to handle `MetadataValue`.
pub trait MetadataExt {
    /// Get name of the `MetadataValue` or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `MetadataExt` for `MetadataValue`.
impl<'ctx> MetadataExt for MetadataValue<'ctx> {
    fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<empty-metadata-name>".to_string(),
        }
    }
}
