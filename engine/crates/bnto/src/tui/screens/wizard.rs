// Wizard screen — guided recipe creation flow.
//
// Steps: Category → Operation → Configure → Complete.
// Categories and operations come from engine metadata (not hardcoded).
// Pure TEA state machine: WizardModel + WizardMessage + update().

use bnto_core::editor::{EditorModel, EditorSource};
use bnto_core::metadata::{NodeCategory, NodeTypeInfo};
use bnto_core::registry::NodeRegistry;
use bnto_form::FormModel;

use super::detail::ParamEntry;
use super::detail_bridge;
use super::editor::EditorScreenModel;

// --- Category filtering ---

/// Categories excluded from the wizard. These are infrastructure nodes,
/// not user-facing file processing operations.
const EXCLUDED_CATEGORIES: &[NodeCategory] =
    &[NodeCategory::Io, NodeCategory::Control, NodeCategory::Data];

fn is_wizard_category(cat: &NodeCategory) -> bool {
    !EXCLUDED_CATEGORIES.iter().any(|exc| exc == cat)
}

// --- Types ---

/// Which step the wizard is on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WizardStep {
    /// "What kind of files?" — pick a category.
    Category,
    /// "What do you want to do?" — pick an operation within the category.
    Operation,
    /// "Configure" — schema-driven param form for the chosen operation.
    Configure,
    /// Summary + "Open in Editor".
    Complete,
}

/// A category entry derived from engine metadata.
#[derive(Debug, Clone)]
pub struct CategoryEntry {
    pub category: NodeCategory,
    pub label: String,
    pub count: usize,
}

/// An operation entry (node type) within a category.
#[derive(Debug, Clone)]
pub struct OperationEntry {
    pub name: String,
    pub label: String,
    pub description: String,
}

/// The wizard's state. Pure data — no I/O.
#[derive(Debug, Clone)]
pub struct WizardModel {
    pub step: WizardStep,
    pub categories: Vec<CategoryEntry>,
    pub operations: Vec<OperationEntry>,
    pub selected_category: Option<NodeCategory>,
    pub selected_operation: Option<String>,
    pub category_cursor: usize,
    pub operation_cursor: usize,
    /// Parameter metadata for the config step.
    pub params: Vec<ParamEntry>,
    /// Form model for the config step.
    pub form: Option<FormModel>,
    /// Generated recipe name (set at Complete step).
    pub recipe_name: String,
}

/// Messages that drive wizard state transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WizardMessage {
    CursorDown,
    CursorUp,
    Confirm,
    Back,
    Form(bnto_form::FormMessage),
}

/// Side effect from the wizard update.
#[derive(Debug, Clone)]
pub enum WizardAction {
    /// Stay on wizard, no side effect.
    None,
    /// Wizard finished — hand off to editor with pre-populated model.
    Complete(Box<EditorScreenModel>),
    /// Exit wizard without completing.
    Back,
}

// --- Construction ---

impl WizardModel {
    /// Build a new wizard from the engine's node type registry.
    pub fn new(_registry: &NodeRegistry) -> Self {
        let types = bnto_core::metadata::all_node_types();
        let categories = build_categories(&types);
        Self {
            step: WizardStep::Category,
            categories,
            operations: Vec::new(),
            selected_category: None,
            selected_operation: None,
            category_cursor: 0,
            operation_cursor: 0,
            params: Vec::new(),
            form: None,
            recipe_name: String::new(),
        }
    }

    /// The number of items in the current step's list.
    #[allow(dead_code)] // Used in tests; will be used for viewport scrolling.
    pub fn list_len(&self) -> usize {
        match self.step {
            WizardStep::Category => self.categories.len(),
            WizardStep::Operation => self.operations.len(),
            _ => 0,
        }
    }

    /// The current cursor position for the active step.
    #[allow(dead_code)] // Used in tests; will be used for viewport scrolling.
    pub fn cursor(&self) -> usize {
        match self.step {
            WizardStep::Category => self.category_cursor,
            WizardStep::Operation => self.operation_cursor,
            _ => 0,
        }
    }
}

// --- Update ---

/// Pure state transition for the wizard screen.
pub fn update(
    mut model: WizardModel,
    msg: WizardMessage,
    registry: &NodeRegistry,
) -> (WizardModel, WizardAction) {
    match msg {
        WizardMessage::CursorDown => {
            match model.step {
                WizardStep::Category => {
                    if !model.categories.is_empty() {
                        model.category_cursor =
                            (model.category_cursor + 1) % model.categories.len();
                    }
                }
                WizardStep::Operation => {
                    if !model.operations.is_empty() {
                        model.operation_cursor =
                            (model.operation_cursor + 1) % model.operations.len();
                    }
                }
                _ => {}
            }
            (model, WizardAction::None)
        }
        WizardMessage::CursorUp => {
            match model.step {
                WizardStep::Category => {
                    if !model.categories.is_empty() {
                        model.category_cursor = if model.category_cursor == 0 {
                            model.categories.len() - 1
                        } else {
                            model.category_cursor - 1
                        };
                    }
                }
                WizardStep::Operation => {
                    if !model.operations.is_empty() {
                        model.operation_cursor = if model.operation_cursor == 0 {
                            model.operations.len() - 1
                        } else {
                            model.operation_cursor - 1
                        };
                    }
                }
                _ => {}
            }
            (model, WizardAction::None)
        }
        WizardMessage::Confirm => handle_confirm(model, registry),
        WizardMessage::Back => handle_back(model),
        WizardMessage::Form(form_msg) => {
            if let Some(form) = model.form.take() {
                let updated = bnto_form::update(form, form_msg);
                detail_bridge::update_visibility(model.form.insert(updated), &model.params);
            }
            (model, WizardAction::None)
        }
    }
}

fn handle_confirm(mut model: WizardModel, registry: &NodeRegistry) -> (WizardModel, WizardAction) {
    match model.step {
        WizardStep::Category => {
            let cat = &model.categories[model.category_cursor];
            model.selected_category = Some(cat.category.clone());
            let types = bnto_core::metadata::all_node_types();
            model.operations = build_operations(&types, &cat.category);
            model.operation_cursor = 0;
            model.step = WizardStep::Operation;
            (model, WizardAction::None)
        }
        WizardStep::Operation => {
            let op = &model.operations[model.operation_cursor];
            model.selected_operation = Some(op.name.clone());

            // Build config form from operation's metadata.
            let (params, form) = build_config_form(&op.name, registry);
            if params.is_empty() {
                // No configurable params — skip to Complete.
                model.recipe_name = generate_name(&model);
                model.step = WizardStep::Complete;
            } else {
                model.params = params;
                model.form = Some(form);
                model.step = WizardStep::Configure;
            }
            (model, WizardAction::None)
        }
        WizardStep::Configure => {
            model.recipe_name = generate_name(&model);
            model.step = WizardStep::Complete;
            (model, WizardAction::None)
        }
        WizardStep::Complete => {
            let editor = build_editor_model(&model, registry);
            let screen = EditorScreenModel::new(editor);
            (model, WizardAction::Complete(Box::new(screen)))
        }
    }
}

fn handle_back(mut model: WizardModel) -> (WizardModel, WizardAction) {
    match model.step {
        WizardStep::Category => (model, WizardAction::Back),
        WizardStep::Operation => {
            model.step = WizardStep::Category;
            model.selected_category = None;
            model.operations.clear();
            (model, WizardAction::None)
        }
        WizardStep::Configure => {
            model.step = WizardStep::Operation;
            model.selected_operation = None;
            model.params.clear();
            model.form = None;
            (model, WizardAction::None)
        }
        WizardStep::Complete => {
            // Back from Complete goes to Configure if there were params,
            // otherwise to Operation.
            if model.params.is_empty() {
                model.step = WizardStep::Operation;
                model.selected_operation = None;
            } else {
                model.step = WizardStep::Configure;
            }
            model.recipe_name.clear();
            (model, WizardAction::None)
        }
    }
}

// --- Helpers ---

/// Build the category list from all node types, excluding infrastructure.
fn build_categories(types: &[NodeTypeInfo]) -> Vec<CategoryEntry> {
    let mut seen = Vec::new();
    let mut entries = Vec::new();
    for t in types {
        if !is_wizard_category(&t.category) || t.is_container {
            continue;
        }
        if !seen.contains(&t.category) {
            let count = types
                .iter()
                .filter(|n| n.category == t.category && !n.is_container)
                .count();
            entries.push(CategoryEntry {
                category: t.category.clone(),
                label: category_label(&t.category),
                count,
            });
            seen.push(t.category.clone());
        }
    }
    // Sort by label for consistent UI order.
    entries.sort_by(|a, b| a.label.cmp(&b.label));
    entries
}

/// Build operations for a specific category.
fn build_operations(types: &[NodeTypeInfo], category: &NodeCategory) -> Vec<OperationEntry> {
    types
        .iter()
        .filter(|t| &t.category == category && !t.is_container)
        .map(|t| OperationEntry {
            name: t.name.clone(),
            label: t.label.clone(),
            description: t.description.clone(),
        })
        .collect()
}

/// Build the config form for a specific operation using registry metadata.
fn build_config_form(node_type: &str, registry: &NodeRegistry) -> (Vec<ParamEntry>, FormModel) {
    let empty_map = serde_json::Map::new();
    let Some(processor) = registry.resolve(node_type, &empty_map) else {
        return (Vec::new(), FormModel::new(Vec::new()));
    };

    let meta = processor.metadata();
    let params: Vec<ParamEntry> = meta
        .parameters
        .iter()
        .filter(|p| p.surfaceable)
        .map(|p| {
            let default_str = p.default.as_ref().map(value_to_display).unwrap_or_default();
            ParamEntry {
                node_id: String::new(),
                name: p.name.clone(),
                label: p.label.clone(),
                value: default_str.clone(),
                param_type: p.param_type.clone(),
                default: default_str,
                description: if p.description.is_empty() {
                    None
                } else {
                    Some(p.description.clone())
                },
                constraints: p.constraints.clone(),
                suffix: p.suffix.clone(),
                control: p.control.clone(),
                visible_when: p.visible_when.clone(),
            }
        })
        .collect();

    let fields = detail_bridge::params_to_fields(&params);
    let form = FormModel::new(fields);
    (params, form)
}

/// Generate a recipe name from the selected operation.
fn generate_name(model: &WizardModel) -> String {
    model
        .operations
        .iter()
        .find(|op| Some(&op.name) == model.selected_operation.as_ref())
        .map(|op| op.label.clone())
        .unwrap_or_else(|| "New Recipe".to_string())
}

/// Build an EditorModel from wizard selections.
fn build_editor_model(model: &WizardModel, _registry: &NodeRegistry) -> EditorModel {
    let mut editor = EditorModel::new();
    editor.recipe_name = model.recipe_name.clone();
    editor.source = EditorSource::New;

    if let Some(ref op_name) = model.selected_operation {
        let types = bnto_core::metadata::all_node_types();
        let info = types.iter().find(|t| &t.name == op_name);
        if let Some(info) = info {
            // Collect param values from the form (if config step was visited).
            let defaults: Vec<(String, serde_json::Value)> = if let Some(ref form) = model.form {
                model
                    .params
                    .iter()
                    .enumerate()
                    .filter_map(|(i, param)| {
                        let field = form.fields.get(i)?;
                        let val = display_to_value(&field.value, &param.param_type);
                        Some((param.name.clone(), val))
                    })
                    .collect()
            } else {
                Vec::new()
            };

            editor.add_node_with_defaults(op_name, info, &defaults);
            // The add_node_with_defaults call sets dirty + pushes undo.
            // For a freshly created recipe, clear both.
            editor.dirty = false;
            editor.undo_stack.clear();
        }
    }

    editor
}

/// Human-readable label for a node category.
fn category_label(cat: &NodeCategory) -> String {
    match cat {
        NodeCategory::Image => "Images".to_string(),
        NodeCategory::Spreadsheet => "Spreadsheets".to_string(),
        NodeCategory::File => "Files".to_string(),
        NodeCategory::Vector => "Vector Graphics".to_string(),
        NodeCategory::Video => "Video".to_string(),
        NodeCategory::Network => "Network".to_string(),
        NodeCategory::System => "System".to_string(),
        _ => format!("{cat:?}"),
    }
}

/// Convert a JSON value to display string (for form default values).
fn value_to_display(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Convert a display string back to JSON based on param type.
fn display_to_value(s: &str, param_type: &bnto_core::metadata::ParameterType) -> serde_json::Value {
    match param_type {
        bnto_core::metadata::ParameterType::Boolean => {
            serde_json::Value::Bool(s == "true" || s == "Yes")
        }
        bnto_core::metadata::ParameterType::Number => s
            .parse::<f64>()
            .map(|n| serde_json::json!(n))
            .unwrap_or_else(|_| serde_json::Value::String(s.to_string())),
        _ => serde_json::Value::String(s.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_engine::create_registry;

    fn registry() -> NodeRegistry {
        create_registry()
    }

    fn new_wizard() -> WizardModel {
        WizardModel::new(&registry())
    }

    // --- Construction ---

    #[test]
    fn wizard_starts_at_category_step() {
        let w = new_wizard();
        assert_eq!(w.step, WizardStep::Category);
        assert!(!w.categories.is_empty());
        assert_eq!(w.category_cursor, 0);
    }

    #[test]
    fn wizard_excludes_io_control_data_categories() {
        let w = new_wizard();
        let cats: Vec<&NodeCategory> = w.categories.iter().map(|c| &c.category).collect();
        assert!(!cats.contains(&&NodeCategory::Io), "IO should be excluded");
        assert!(
            !cats.contains(&&NodeCategory::Control),
            "Control should be excluded"
        );
        assert!(
            !cats.contains(&&NodeCategory::Data),
            "Data should be excluded"
        );
    }

    #[test]
    fn wizard_includes_image_category() {
        let w = new_wizard();
        let cats: Vec<&NodeCategory> = w.categories.iter().map(|c| &c.category).collect();
        assert!(cats.contains(&&NodeCategory::Image));
    }

    #[test]
    fn category_count_matches_node_types() {
        let w = new_wizard();
        let image = w
            .categories
            .iter()
            .find(|c| c.category == NodeCategory::Image);
        assert!(image.is_some());
        assert_eq!(image.unwrap().count, 5, "Image should have 5 node types");
    }

    // --- Category navigation ---

    #[test]
    fn category_cursor_wraps_down() {
        let mut w = new_wizard();
        let len = w.categories.len();
        w.category_cursor = len - 1;
        let (w, _) = update(w, WizardMessage::CursorDown, &registry());
        assert_eq!(w.category_cursor, 0);
    }

    #[test]
    fn category_cursor_wraps_up() {
        let w = new_wizard();
        assert_eq!(w.category_cursor, 0);
        let (w, _) = update(w, WizardMessage::CursorUp, &registry());
        assert_eq!(w.category_cursor, w.categories.len() - 1);
    }

    // --- Category → Operation ---

    #[test]
    fn category_confirm_advances_to_operation() {
        let mut w = new_wizard();
        // Find the Image category index.
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (w, action) = update(w, WizardMessage::Confirm, &registry());
        assert_eq!(w.step, WizardStep::Operation);
        assert!(matches!(action, WizardAction::None));
        assert_eq!(w.selected_category, Some(NodeCategory::Image));
        assert!(!w.operations.is_empty());
    }

    #[test]
    fn operations_filtered_by_category() {
        let mut w = new_wizard();
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (w, _) = update(w, WizardMessage::Confirm, &registry());
        // All operations should be image operations.
        assert!(!w.operations.is_empty());
        let types = bnto_core::metadata::all_node_types();
        for op in &w.operations {
            let t = types.iter().find(|t| t.name == op.name).unwrap();
            assert_eq!(t.category, NodeCategory::Image);
        }
    }

    // --- Operation navigation ---

    #[test]
    fn operation_cursor_wraps_down() {
        let mut w = new_wizard();
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (mut w, _) = update(w, WizardMessage::Confirm, &registry());
        let len = w.operations.len();
        w.operation_cursor = len - 1;
        let (w, _) = update(w, WizardMessage::CursorDown, &registry());
        assert_eq!(w.operation_cursor, 0);
    }

    // --- Operation → Configure ---

    #[test]
    fn operation_confirm_advances_to_configure() {
        let reg = registry();
        let mut w = new_wizard();
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (mut w, _) = update(w, WizardMessage::Confirm, &reg);
        // Find "image-compress" which has params.
        let op_idx = w
            .operations
            .iter()
            .position(|o| o.name == "image-compress")
            .unwrap();
        w.operation_cursor = op_idx;
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        assert_eq!(w.step, WizardStep::Configure);
        assert!(w.form.is_some());
        assert!(!w.params.is_empty());
    }

    #[test]
    fn operation_skips_config_when_no_params() {
        let reg = registry();
        // Simulate an operation with no surfaceable params by constructing
        // a wizard already on the Operation step with a fake operation.
        let mut w = new_wizard();
        w.step = WizardStep::Operation;
        w.selected_category = Some(NodeCategory::Image);
        w.operations = vec![OperationEntry {
            name: "fake-no-param".to_string(),
            label: "Fake No Param".to_string(),
            description: "Test op".to_string(),
        }];
        w.operation_cursor = 0;
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        // "fake-no-param" won't resolve in the registry → no params → skip to Complete.
        assert_eq!(
            w.step,
            WizardStep::Complete,
            "Should skip to Complete for no-param ops"
        );
    }

    // --- Configure → Complete ---

    #[test]
    fn configure_confirm_advances_to_complete() {
        let reg = registry();
        let mut w = new_wizard();
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (mut w, _) = update(w, WizardMessage::Confirm, &reg);
        let op_idx = w
            .operations
            .iter()
            .position(|o| o.name == "image-compress")
            .unwrap();
        w.operation_cursor = op_idx;
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        assert_eq!(w.step, WizardStep::Configure);
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        assert_eq!(w.step, WizardStep::Complete);
        assert!(!w.recipe_name.is_empty());
    }

    // --- Complete → EditorModel ---

    #[test]
    fn complete_produces_valid_editor_model() {
        let reg = registry();
        let mut w = new_wizard();
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (mut w, _) = update(w, WizardMessage::Confirm, &reg);
        let op_idx = w
            .operations
            .iter()
            .position(|o| o.name == "image-compress")
            .unwrap();
        w.operation_cursor = op_idx;
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        assert_eq!(w.step, WizardStep::Complete);
        let (_, action) = update(w, WizardMessage::Confirm, &reg);
        match action {
            WizardAction::Complete(screen) => {
                assert_eq!(screen.editor.nodes.len(), 1);
                assert_eq!(screen.editor.nodes[0].node_type, "image-compress");
                assert!(
                    !screen.editor.dirty,
                    "Fresh wizard recipe should not be dirty"
                );
                assert!(screen.editor.undo_stack.is_empty());
            }
            _ => panic!("Expected Complete action"),
        }
    }

    // --- Auto-naming ---

    #[test]
    fn auto_naming_uses_operation_label() {
        let reg = registry();
        let mut w = new_wizard();
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (mut w, _) = update(w, WizardMessage::Confirm, &reg);
        let op_idx = w
            .operations
            .iter()
            .position(|o| o.name == "image-compress")
            .unwrap();
        let expected_label = w.operations[op_idx].label.clone();
        w.operation_cursor = op_idx;
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        assert_eq!(w.recipe_name, expected_label);
    }

    // --- Back navigation ---

    #[test]
    fn back_from_category_exits_wizard() {
        let w = new_wizard();
        let (_, action) = update(w, WizardMessage::Back, &registry());
        assert!(matches!(action, WizardAction::Back));
    }

    #[test]
    fn back_from_operation_returns_to_category() {
        let reg = registry();
        let mut w = new_wizard();
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        assert_eq!(w.step, WizardStep::Operation);
        let (w, action) = update(w, WizardMessage::Back, &reg);
        assert_eq!(w.step, WizardStep::Category);
        assert!(matches!(action, WizardAction::None));
        assert_eq!(w.selected_category, None);
    }

    #[test]
    fn back_from_configure_returns_to_operation() {
        let reg = registry();
        let mut w = new_wizard();
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (mut w, _) = update(w, WizardMessage::Confirm, &reg);
        let op_idx = w
            .operations
            .iter()
            .position(|o| o.name == "image-compress")
            .unwrap();
        w.operation_cursor = op_idx;
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        assert_eq!(w.step, WizardStep::Configure);
        let (w, _) = update(w, WizardMessage::Back, &reg);
        assert_eq!(w.step, WizardStep::Operation);
        assert_eq!(w.selected_operation, None);
    }

    #[test]
    fn back_from_complete_returns_to_configure_if_params_exist() {
        let reg = registry();
        let mut w = new_wizard();
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (mut w, _) = update(w, WizardMessage::Confirm, &reg);
        let op_idx = w
            .operations
            .iter()
            .position(|o| o.name == "image-compress")
            .unwrap();
        w.operation_cursor = op_idx;
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        assert_eq!(w.step, WizardStep::Complete);
        let (w, _) = update(w, WizardMessage::Back, &reg);
        assert_eq!(w.step, WizardStep::Configure);
    }

    // --- Form messages ---

    #[test]
    fn form_message_updates_form_state() {
        let reg = registry();
        let mut w = new_wizard();
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (mut w, _) = update(w, WizardMessage::Confirm, &reg);
        let op_idx = w
            .operations
            .iter()
            .position(|o| o.name == "image-compress")
            .unwrap();
        w.operation_cursor = op_idx;
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        assert!(w.form.is_some());
        // Send a focus message — should not crash.
        let (w, _) = update(
            w,
            WizardMessage::Form(bnto_form::FormMessage::FocusNext),
            &reg,
        );
        assert!(w.form.is_some());
    }

    // --- list_len helper ---

    #[test]
    fn list_len_returns_category_count_on_category_step() {
        let w = new_wizard();
        assert_eq!(w.list_len(), w.categories.len());
    }

    #[test]
    fn list_len_returns_operation_count_on_operation_step() {
        let reg = registry();
        let mut w = new_wizard();
        let idx = w
            .categories
            .iter()
            .position(|c| c.category == NodeCategory::Image)
            .unwrap();
        w.category_cursor = idx;
        let (w, _) = update(w, WizardMessage::Confirm, &reg);
        assert_eq!(w.list_len(), w.operations.len());
    }
}
