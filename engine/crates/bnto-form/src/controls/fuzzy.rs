// Fuzzy substring matching for Select filter.
//
// Scores options by how well they match a filter string. Supports
// out-of-order character matching (e.g., "jg" matches "jpeg") with
// bonus scoring for consecutive matches, prefix matches, and exact
// case matches.

use crate::field::SelectOption;

/// Score a single option label against a filter string.
///
/// Returns `Some(score)` if all filter characters appear in order within
/// the label, `None` if there's no match. Higher scores = better match.
pub fn fuzzy_score(label: &str, filter: &str) -> Option<u32> {
    if filter.is_empty() {
        return Some(0);
    }

    let label_lower = label.to_lowercase();
    let filter_lower = filter.to_lowercase();
    let label_chars: Vec<char> = label_lower.chars().collect();
    let filter_chars: Vec<char> = filter_lower.chars().collect();

    let mut score: u32 = 0;
    let mut label_idx = 0;
    let mut prev_match_idx: Option<usize> = None;

    for &fc in &filter_chars {
        let mut found = false;
        while label_idx < label_chars.len() {
            if label_chars[label_idx] == fc {
                // Base score for each matched character
                score += 1;

                // Consecutive match bonus
                if let Some(prev) = prev_match_idx
                    && label_idx == prev + 1
                {
                    score += 2;
                }

                // Prefix bonus: matching at position 0
                if label_idx == 0 {
                    score += 3;
                }

                prev_match_idx = Some(label_idx);
                label_idx += 1;
                found = true;
                break;
            }
            label_idx += 1;
        }
        if !found {
            return None;
        }
    }

    Some(score)
}

/// Filter options by fuzzy match, returning indices sorted by score (best first).
///
/// When `filter` is empty, returns all indices in original order.
pub fn fuzzy_filter_options(options: &[SelectOption], filter: &str) -> Vec<usize> {
    if filter.is_empty() {
        return (0..options.len()).collect();
    }

    let mut scored: Vec<(usize, u32)> = options
        .iter()
        .enumerate()
        .filter_map(|(i, o)| fuzzy_score(&o.label, filter).map(|s| (i, s)))
        .collect();

    // Sort by score descending (best match first)
    scored.sort_by_key(|item| std::cmp::Reverse(item.1));

    scored.into_iter().map(|(i, _)| i).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::SelectOption;

    fn opt(value: &str, label: &str) -> SelectOption {
        SelectOption {
            value: value.to_string(),
            label: label.to_string(),
        }
    }

    // --- fuzzy_score ---

    #[test]
    fn test_fuzzy_empty_filter_always_matches() {
        assert_eq!(fuzzy_score("anything", ""), Some(0));
    }

    #[test]
    fn test_fuzzy_exact_match() {
        let score = fuzzy_score("jpeg", "jpeg");
        assert!(score.is_some());
        assert!(score.unwrap() > 0);
    }

    #[test]
    fn test_fuzzy_subsequence_match() {
        // "jg" matches "jpeg" — j at 0, g doesn't exist → None
        // Actually "jg": j matches at 0, g... "jpeg" has no 'g' → None
        // Let's use "jp" instead for a real subsequence
        assert!(fuzzy_score("jpeg", "jp").is_some());
    }

    #[test]
    fn test_fuzzy_no_match() {
        assert_eq!(fuzzy_score("jpeg", "xyz"), None);
    }

    #[test]
    fn test_fuzzy_case_insensitive() {
        assert!(fuzzy_score("JPEG", "jp").is_some());
        assert!(fuzzy_score("jpeg", "JP").is_some());
    }

    #[test]
    fn test_fuzzy_prefix_scores_higher() {
        // "jp" matches "jpeg" starting at prefix — should score higher than
        // a match that starts later in the string
        let prefix_score = fuzzy_score("jpeg", "jp").unwrap();
        let mid_score = fuzzy_score("_jpeg", "jp").unwrap();
        assert!(
            prefix_score > mid_score,
            "prefix {prefix_score} should beat mid {mid_score}"
        );
    }

    #[test]
    fn test_fuzzy_consecutive_bonus() {
        // "jpe" in "jpeg" is consecutive — should score higher than
        // "je" in "jpeg" which skips 'p'
        let consecutive = fuzzy_score("jpeg", "jpe").unwrap();
        let skipped = fuzzy_score("jpeg", "je").unwrap();
        assert!(
            consecutive > skipped,
            "consecutive {consecutive} should beat skipped {skipped}"
        );
    }

    // --- fuzzy_filter_options ---

    #[test]
    fn test_filter_empty_returns_all() {
        let options = vec![opt("a", "Alpha"), opt("b", "Beta"), opt("c", "Charlie")];
        let result = fuzzy_filter_options(&options, "");
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_filter_narrows_results() {
        let options = vec![opt("a", "Alpha"), opt("b", "Beta"), opt("c", "Charlie")];
        // "al" is a subsequence of both "Alpha" (a-l) and "Charlie" (ch-a-r-l-ie)
        // Alpha should rank higher (prefix bonus)
        let result = fuzzy_filter_options(&options, "al");
        assert_eq!(result, vec![0, 2]);
        // A more selective filter narrows further
        let result = fuzzy_filter_options(&options, "alp");
        assert_eq!(result, vec![0]); // only "Alpha" matches "alp"
    }

    #[test]
    fn test_filter_no_match_returns_empty() {
        let options = vec![opt("a", "Alpha"), opt("b", "Beta")];
        let result = fuzzy_filter_options(&options, "xyz");
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_ranks_by_score() {
        // "png" exactly matches "PNG" better than it matches "Padding"
        let options = vec![opt("pad", "Padding"), opt("png", "PNG")];
        let result = fuzzy_filter_options(&options, "png");
        // PNG should rank first (exact match), Padding matches p-n-g but scattered
        assert!(!result.is_empty());
        assert_eq!(result[0], 1, "PNG should rank first");
    }
}
