//! Field grouping for named sections.
//!
//! `FieldGroup` wraps a range of fields into a named section.
//! Groups are visual organizers — they render section headers in
//! DisplayEdit and FullScreenEdit modes. Navigation crosses group
//! boundaries seamlessly.

/// A named section of fields within a form.
#[derive(Debug, Clone)]
pub struct FieldGroup {
    /// Section label rendered as a header (empty string = implicit/hidden group).
    pub label: String,
    /// Indices into `FormModel.fields` that belong to this group.
    pub field_indices: Vec<usize>,
}

impl FieldGroup {
    /// Create a new group with a label and field indices.
    pub fn new(label: impl Into<String>, field_indices: Vec<usize>) -> Self {
        Self {
            label: label.into(),
            field_indices,
        }
    }

    /// Whether this group should render a visible header.
    pub fn has_header(&self) -> bool {
        !self.label.is_empty()
    }
}

/// Find which group a field index belongs to.
pub fn group_for_field(groups: &[FieldGroup], field_index: usize) -> Option<usize> {
    groups
        .iter()
        .position(|g| g.field_indices.contains(&field_index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_new_stores_label_and_indices() {
        let g = FieldGroup::new("General", vec![0, 1, 2]);
        assert_eq!(g.label, "General");
        assert_eq!(g.field_indices, vec![0, 1, 2]);
    }

    #[test]
    fn empty_label_hides_header() {
        let g = FieldGroup::new("", vec![0]);
        assert!(!g.has_header());
    }

    #[test]
    fn non_empty_label_shows_header() {
        let g = FieldGroup::new("Settings", vec![0]);
        assert!(g.has_header());
    }

    #[test]
    fn group_for_field_finds_correct_group() {
        let groups = vec![
            FieldGroup::new("A", vec![0, 1]),
            FieldGroup::new("B", vec![2, 3]),
        ];
        assert_eq!(group_for_field(&groups, 0), Some(0));
        assert_eq!(group_for_field(&groups, 1), Some(0));
        assert_eq!(group_for_field(&groups, 2), Some(1));
        assert_eq!(group_for_field(&groups, 3), Some(1));
        assert_eq!(group_for_field(&groups, 4), None);
    }
}
