// Editor mutations — all state changes that push undo snapshots.

use std::collections::HashMap;

use crate::metadata::NodeTypeInfo;

use super::types::{EditorModel, EditorNode, EditorSnapshot};

impl EditorModel {
    /// Add a node with defaults populated from engine metadata.
    pub fn add_node(&mut self, node_type: &str, info: &NodeTypeInfo) {
        self.push_undo();
        let id = format!("{}-{}", node_type, self.nodes.len() + 1);
        self.nodes.push(EditorNode {
            id,
            node_type: node_type.to_string(),
            label: info.label.clone(),
            params: HashMap::new(),
            expanded: false,
        });
        self.selected_index = Some(self.nodes.len() - 1);
        self.dirty = true;
    }

    /// Add a node with pre-populated default parameter values from metadata.
    pub fn add_node_with_defaults(
        &mut self,
        node_type: &str,
        info: &NodeTypeInfo,
        param_defaults: &[(String, serde_json::Value)],
    ) {
        self.push_undo();
        let id = format!("{}-{}", node_type, self.nodes.len() + 1);
        let params: HashMap<String, serde_json::Value> = param_defaults.iter().cloned().collect();
        self.nodes.push(EditorNode {
            id,
            node_type: node_type.to_string(),
            label: info.label.clone(),
            params,
            expanded: false,
        });
        self.selected_index = Some(self.nodes.len() - 1);
        self.dirty = true;
    }

    /// Remove the node at the given index.
    pub fn remove_node(&mut self, index: usize) {
        if index >= self.nodes.len() {
            return;
        }
        self.push_undo();
        self.nodes.remove(index);
        self.selected_index = if self.nodes.is_empty() {
            None
        } else {
            Some(index.min(self.nodes.len() - 1))
        };
        self.dirty = true;
    }

    /// Swap nodes at positions `from` and `to`.
    pub fn reorder(&mut self, from: usize, to: usize) {
        if from >= self.nodes.len() || to >= self.nodes.len() || from == to {
            return;
        }
        self.push_undo();
        self.nodes.swap(from, to);
        self.selected_index = Some(to);
        self.dirty = true;
    }

    /// Update a single parameter value on the node at `index`.
    pub fn update_param(&mut self, index: usize, key: &str, value: serde_json::Value) {
        if index >= self.nodes.len() {
            return;
        }
        self.push_undo();
        self.nodes[index].params.insert(key.to_string(), value);
        self.dirty = true;
    }

    /// Undo the last mutation.
    pub fn undo(&mut self) {
        if let Some(snapshot) = self.undo_stack.pop() {
            let current = EditorSnapshot::capture(self);
            self.redo_stack.push(current);
            self.restore(&snapshot);
        }
    }

    /// Redo the last undone mutation.
    pub fn redo(&mut self) {
        if let Some(snapshot) = self.redo_stack.pop() {
            let current = EditorSnapshot::capture(self);
            self.undo_stack.push(current);
            self.restore(&snapshot);
        }
    }

    /// Mark the model as clean (after a successful save).
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    fn push_undo(&mut self) {
        self.undo_stack.push(EditorSnapshot::capture(self));
        self.redo_stack.clear();
    }

    fn restore(&mut self, snapshot: &EditorSnapshot) {
        self.recipe_name = snapshot.recipe_name.clone();
        self.recipe_description = snapshot.recipe_description.clone();
        self.nodes = snapshot.nodes.clone();
        self.selected_index = if self.nodes.is_empty() {
            None
        } else {
            self.selected_index.map(|i| i.min(self.nodes.len() - 1))
        };
    }
}
