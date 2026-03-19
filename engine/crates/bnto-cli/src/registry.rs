// Processor registry — mirrors bnto-wasm/src/execute.rs:create_default_registry().
// Same 6 processors, same compound keys. CLI and WASM are sibling adapters.

use bnto_core::NodeRegistry;

/// Create a registry pre-loaded with all node processors.
/// Identical to the WASM registry — same processors, same keys.
pub fn create_default_registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();

    registry.register(
        "image:compress",
        Box::new(bnto_image::CompressImages::new()),
    );
    registry.register("image:resize", Box::new(bnto_image::ResizeImages::new()));
    registry.register(
        "image:convert",
        Box::new(bnto_image::ConvertImageFormat::new()),
    );

    registry.register("spreadsheet:clean", Box::new(bnto_csv::CleanCsv::new()));
    registry.register(
        "spreadsheet:rename",
        Box::new(bnto_csv::RenameCsvColumns::new()),
    );

    registry.register(
        "file-system:rename",
        Box::new(bnto_file::RenameFiles::new()),
    );

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_all_processors() {
        let registry = create_default_registry();
        assert_eq!(registry.len(), 6);
    }
}
