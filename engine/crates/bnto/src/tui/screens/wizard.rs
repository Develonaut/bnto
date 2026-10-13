// Wizard screen — guided recipe creation flow.
//
// Steps: Category → Operation → Configure → Complete.
// Categories and operations come from engine metadata (not hardcoded).
// Pure TEA state machine: WizardModel + WizardMessage + update().

mod catalog;
mod config;
mod steps;

use bnto_core::metadata::NodeCategory;
use bnto_core::registry::NodeRegistry;
use tonkotsu::FormModel;

use super::detail::ParamEntry;
use super::detail_bridge;
use super::editor::EditorScreenModel;
use catalog::build_categories;

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
    Form(tonkotsu::FormMessage),
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

    /// Move the active step's list cursor with `step(cursor, len)`.
    fn move_cursor(&mut self, step: fn(usize, usize) -> usize) {
        let (cursor, len) = match self.step {
            WizardStep::Category => (&mut self.category_cursor, self.categories.len()),
            WizardStep::Operation => (&mut self.operation_cursor, self.operations.len()),
            _ => return,
        };
        if len > 0 {
            *cursor = step(*cursor, len);
        }
    }

    /// Feed a message to the config form and refresh field visibility.
    fn apply_form(&mut self, msg: tonkotsu::FormMessage) {
        if let Some(form) = self.form.take() {
            let updated = tonkotsu::update(form, msg);
            detail_bridge::update_visibility(self.form.insert(updated), &self.params);
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
        WizardMessage::CursorDown => model.move_cursor(|c, len| (c + 1) % len),
        WizardMessage::CursorUp => model.move_cursor(|c, len| if c == 0 { len - 1 } else { c - 1 }),
        WizardMessage::Confirm => return steps::handle_confirm(model, registry),
        WizardMessage::Back => return steps::handle_back(model),
        WizardMessage::Form(form_msg) => model.apply_form(form_msg),
    }
    (model, WizardAction::None)
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
            WizardMessage::Form(tonkotsu::FormMessage::FocusNext),
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
