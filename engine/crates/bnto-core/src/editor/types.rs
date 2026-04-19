// Editor types — pure data structures for recipe editing state.

use std::collections::HashMap;
use std::path::PathBuf;

/// Errors that can occur during editor operations (load, save, validation).
#[derive(Debug, thiserror::Error)]
pub enum EditorError {
    #[error("file not found: {0}")]
    NotFound(PathBuf),

    #[error("invalid JSON: {0}")]
    InvalidJson(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Where the recipe being edited came from.
#[derive(Debug, Clone, PartialEq)]
pub enum EditorSource {
    /// Brand-new recipe, never saved.
    New,
    /// Loaded from a user file on disk.
    File(PathBuf),
    /// Cloned from a built-in predefined recipe.
    Predefined(String),
}

/// A single node inside the editor's node list.
#[derive(Debug, Clone, PartialEq)]
pub struct EditorNode {
    /// Unique identifier (matches Definition.id).
    pub id: String,
    /// Engine node type key (e.g., "image-compress").
    pub node_type: String,
    /// Human-readable label (from NodeTypeInfo).
    pub label: String,
    /// Current parameter values.
    pub params: HashMap<String, serde_json::Value>,
    /// Whether the node is expanded in the list view.
    pub expanded: bool,
}

/// A snapshot of editor state for undo/redo.
#[derive(Debug, Clone, PartialEq)]
pub struct EditorSnapshot {
    pub recipe_name: String,
    pub recipe_description: String,
    pub nodes: Vec<EditorNode>,
}

/// The main editor state. Pure data — all mutations are method calls
/// that modify fields in place. No I/O, no side effects.
#[derive(Debug, Clone)]
pub struct EditorModel {
    pub recipe_name: String,
    pub recipe_description: String,
    pub nodes: Vec<EditorNode>,
    pub selected_index: Option<usize>,
    pub dirty: bool,
    pub undo_stack: Vec<EditorSnapshot>,
    pub redo_stack: Vec<EditorSnapshot>,
    pub source: EditorSource,
}

impl EditorSnapshot {
    /// Capture the current state as a snapshot.
    pub(crate) fn capture(model: &EditorModel) -> Self {
        Self {
            recipe_name: model.recipe_name.clone(),
            recipe_description: model.recipe_description.clone(),
            nodes: model.nodes.clone(),
        }
    }
}

impl Default for EditorModel {
    fn default() -> Self {
        Self {
            recipe_name: String::new(),
            recipe_description: String::new(),
            nodes: Vec::new(),
            selected_index: None,
            dirty: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            source: EditorSource::New,
        }
    }
}

impl EditorModel {
    /// Create a blank editor with no nodes.
    pub fn new() -> Self {
        Self::default()
    }
}
