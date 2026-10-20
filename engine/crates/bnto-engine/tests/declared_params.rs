// A run refuses params a node doesn't declare, but only for a processor
// whose metadata describes it (`NodeProcessor::declared_params`). One left
// on the placeholder metadata would take any param, so every registered
// processor must publish its own.

use bnto_engine::create_registry;
use serde_json::Map;

#[test]
fn every_registered_processor_declares_its_params() {
    let registry = create_registry();
    let placeholder: Vec<String> = registry
        .catalog()
        .iter()
        .filter_map(|meta| {
            let processor = registry.resolve(&meta.node_type, &Map::new());
            let declares = processor.is_some_and(|p| p.declared_params().is_some());
            (!declares).then(|| format!("{} ({})", meta.node_type, meta.name))
        })
        .collect();

    assert!(
        placeholder.is_empty(),
        "no declared params: {placeholder:?}"
    );
}
