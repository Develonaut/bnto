// Definition ↔ EditorModel conversion and utility functions.

use std::collections::HashMap;

use crate::definition::{Definition, Edge, Metadata, Position};
use crate::pipeline::{IterationMode, PipelineSettings};
use crate::{FORMAT_VERSION, definition};

use super::types::{EditorModel, EditorNode, EditorSource};

impl EditorModel {
    /// Build an editor model from a parsed `Definition`.
    pub fn from_definition(def: &Definition, source: EditorSource) -> Self {
        let nodes: Vec<EditorNode> = def
            .nodes
            .as_ref()
            .map(|defs| {
                defs.iter()
                    .map(|d| EditorNode {
                        id: d.id.clone(),
                        node_type: d.node_type.clone(),
                        label: d.name.clone(),
                        params: json_to_params(&d.parameters),
                        expanded: false,
                    })
                    .collect()
            })
            .unwrap_or_default();

        let selected_index = if nodes.is_empty() { None } else { Some(0) };

        Self {
            recipe_name: def.name.clone(),
            recipe_description: def.metadata.description.clone().unwrap_or_default(),
            nodes,
            selected_index,
            dirty: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            source,
        }
    }

    /// Serialize the editor state back to a `Definition`.
    pub fn to_definition(&self) -> Definition {
        let child_defs: Vec<Definition> = self
            .nodes
            .iter()
            .enumerate()
            .map(|(i, node)| Definition {
                id: node.id.clone(),
                node_type: node.node_type.clone(),
                version: FORMAT_VERSION.to_string(),
                parent_id: None,
                name: node.label.clone(),
                position: Position {
                    x: 0.0,
                    y: (i as f64) * 100.0,
                },
                metadata: Metadata::default(),
                parameters: params_to_json(&node.params),
                fields: None,
                input_ports: vec![],
                output_ports: vec![],
                nodes: None,
                edges: None,
                settings: None,
                requires: Vec::new(),
            })
            .collect();

        let edges: Vec<Edge> = child_defs
            .windows(2)
            .enumerate()
            .map(|(i, pair)| Edge {
                id: format!("e{}", i + 1),
                source: pair[0].id.clone(),
                target: pair[1].id.clone(),
                source_handle: None,
                target_handle: None,
            })
            .collect();

        Definition {
            id: slug_from_name(&self.recipe_name),
            node_type: "group".to_string(),
            version: FORMAT_VERSION.to_string(),
            parent_id: None,
            name: self.recipe_name.clone(),
            position: Position { x: 0.0, y: 0.0 },
            metadata: Metadata {
                description: if self.recipe_description.is_empty() {
                    None
                } else {
                    Some(self.recipe_description.clone())
                },
                ..Default::default()
            },
            parameters: definition::default_parameters(),
            fields: None,
            input_ports: vec![],
            output_ports: vec![],
            nodes: Some(child_defs),
            edges: Some(edges),
            settings: Some(PipelineSettings {
                iteration: IterationMode::Auto,
            }),
            requires: Vec::new(),
        }
    }
}

/// Convert a `serde_json::Value` object into a `HashMap<String, Value>`.
pub(crate) fn json_to_params(value: &serde_json::Value) -> HashMap<String, serde_json::Value> {
    match value.as_object() {
        Some(map) => map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        None => HashMap::new(),
    }
}

/// Convert a `HashMap<String, Value>` back to a `serde_json::Value` object.
pub(crate) fn params_to_json(params: &HashMap<String, serde_json::Value>) -> serde_json::Value {
    let map: serde_json::Map<String, serde_json::Value> =
        params.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    serde_json::Value::Object(map)
}

/// Generate a URL-safe slug from a recipe name.
pub(crate) fn slug_from_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_from_name_produces_valid_slug() {
        assert_eq!(slug_from_name("My Great Recipe"), "my-great-recipe");
        assert_eq!(slug_from_name(""), "");
        assert_eq!(slug_from_name("hello"), "hello");
        assert_eq!(slug_from_name("A  B"), "a-b");
        assert_eq!(slug_from_name("Compress & Resize!"), "compress-resize");
    }

    #[test]
    fn json_to_params_handles_object() {
        use serde_json::json;
        let val = json!({"quality": 80, "format": "webp"});
        let params = json_to_params(&val);
        assert_eq!(params.get("quality"), Some(&json!(80)));
        assert_eq!(params.get("format"), Some(&json!("webp")));
    }

    #[test]
    fn json_to_params_handles_non_object() {
        use serde_json::json;
        let val = json!(42);
        let params = json_to_params(&val);
        assert!(params.is_empty());
    }
}
