// Wizard catalog — category and operation lists derived from engine metadata.

use bnto_core::metadata::{NodeCategory, NodeTypeInfo};

use super::{CategoryEntry, OperationEntry};

/// Categories excluded from the wizard. These are infrastructure nodes,
/// not user-facing file processing operations.
const EXCLUDED_CATEGORIES: &[NodeCategory] =
    &[NodeCategory::Io, NodeCategory::Control, NodeCategory::Data];

fn is_wizard_category(cat: &NodeCategory) -> bool {
    !EXCLUDED_CATEGORIES.iter().any(|exc| exc == cat)
}

/// Build the category list from all node types, excluding infrastructure.
pub(super) fn build_categories(types: &[NodeTypeInfo]) -> Vec<CategoryEntry> {
    let mut seen = Vec::new();
    let mut entries = Vec::new();
    for t in types {
        if !is_wizard_category(&t.category) || t.is_container || seen.contains(&t.category) {
            continue;
        }
        entries.push(CategoryEntry {
            category: t.category.clone(),
            label: category_label(&t.category),
            count: operation_count(types, &t.category),
        });
        seen.push(t.category.clone());
    }
    // Sort by label for consistent UI order.
    entries.sort_by(|a, b| a.label.cmp(&b.label));
    entries
}

/// Number of non-container node types in `category`.
fn operation_count(types: &[NodeTypeInfo], category: &NodeCategory) -> usize {
    types
        .iter()
        .filter(|n| &n.category == category && !n.is_container)
        .count()
}

/// Build operations for a specific category.
pub(super) fn build_operations(
    types: &[NodeTypeInfo],
    category: &NodeCategory,
) -> Vec<OperationEntry> {
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
