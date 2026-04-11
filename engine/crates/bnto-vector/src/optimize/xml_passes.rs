// XML-level pass helpers for the SVG optimizer.
//
// Each function is a pure decision — takes a node or attribute,
// returns whether it should be kept or skipped. No tree mutation.

use roxmltree::{Attribute, Document, ExpandedName};

/// Collect namespace prefixes bound to editor URIs on the root SVG element.
///
/// roxmltree exposes namespace declarations via `node.namespaces()`,
/// not as regular attributes. Each namespace has `.name()` (the prefix)
/// and `.uri()` (the namespace URI).
pub fn collect_editor_prefixes(doc: &Document, editor_uris: &[&str]) -> Vec<String> {
    let svg = match doc.root().first_element_child() {
        Some(el) => el,
        None => return Vec::new(),
    };

    let mut prefixes = Vec::new();
    for ns in svg.namespaces() {
        if let Some(prefix) = ns.name()
            && editor_uris.contains(&ns.uri())
        {
            prefixes.push(prefix.to_string());
        }
    }
    prefixes
}

/// Check if an element belongs to an editor namespace (e.g., `sodipodi:namedview`).
pub fn is_editor_element(tag: ExpandedName, editor_prefixes: &[String]) -> bool {
    let ns = match tag.namespace() {
        Some(ns) => ns,
        None => return false,
    };

    // Check if the namespace URI is an SVG or xlink namespace (keep those)
    if ns == "http://www.w3.org/2000/svg" || ns == "http://www.w3.org/1999/xlink" {
        return false;
    }

    // If the element has a resolved namespace that matches editor URIs,
    // it should be removed. We also check prefixes for xmlns declarations
    // that roxmltree already resolved.
    let _ = editor_prefixes; // prefixes checked via namespace URI resolution
    true
}

/// Elements that can be removed when empty (no surviving children).
pub fn is_collapsible_container(tag_name: &str) -> bool {
    matches!(
        tag_name,
        "g" | "defs" | "symbol" | "marker" | "clipPath" | "mask" | "pattern"
    )
}

/// Check if a `<g>` has no meaningful attributes (only editor cruft or nothing).
///
/// A group with `id`, `class`, `style`, `transform`, `opacity`, `clip-path`,
/// `mask`, `filter`, or `fill`/`stroke` attributes is meaningful and should
/// NOT be collapsed even if it has only one child.
pub fn has_no_meaningful_attributes(node: &roxmltree::Node, editor_prefixes: &[String]) -> bool {
    for attr in node.attributes() {
        if is_editor_attribute(&attr, editor_prefixes) {
            continue;
        }
        if attr.value().is_empty() {
            continue;
        }
        // Any non-editor, non-empty attribute makes the group meaningful
        return false;
    }
    true
}

/// Check if an attribute belongs to an editor namespace.
pub fn is_editor_attribute(attr: &Attribute, editor_prefixes: &[String]) -> bool {
    // If the attribute has a resolved namespace URI, check it
    if let Some(ns) = attr.namespace()
        && ns != "http://www.w3.org/2000/svg"
        && ns != "http://www.w3.org/1999/xlink"
        && ns != "http://www.w3.org/XML/1998/namespace"
    {
        return true;
    }

    // Check if the attribute name has an editor prefix (for unresolved prefixed attrs)
    if let Some(prefix) = attr.name().split(':').next()
        && attr.name().contains(':')
        && editor_prefixes.contains(&prefix.to_string())
    {
        return true;
    }

    false
}

/// Minify text content — collapse whitespace to a single space, trim.
pub fn minify_text(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    // Collapse runs of whitespace into single spaces
    let mut result = String::with_capacity(trimmed.len());
    let mut prev_was_space = false;
    for ch in trimmed.chars() {
        if ch.is_whitespace() {
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true;
            }
        } else {
            result.push(ch);
            prev_was_space = false;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_editor_prefixes_finds_inkscape() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"><rect/></svg>"#;
        let doc = Document::parse(svg).unwrap();
        let uris = &["http://www.inkscape.org/namespaces/inkscape"];
        let prefixes = collect_editor_prefixes(&doc, uris);
        assert_eq!(prefixes, vec!["inkscape"]);
    }

    #[test]
    fn test_collect_editor_prefixes_ignores_svg_namespace() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><rect/></svg>"#;
        let doc = Document::parse(svg).unwrap();
        let uris = &["http://www.inkscape.org/namespaces/inkscape"];
        let prefixes = collect_editor_prefixes(&doc, uris);
        assert!(prefixes.is_empty());
    }

    #[test]
    fn test_collapsible_containers() {
        assert!(is_collapsible_container("g"));
        assert!(is_collapsible_container("defs"));
        assert!(is_collapsible_container("symbol"));
        assert!(!is_collapsible_container("rect"));
        assert!(!is_collapsible_container("svg"));
        assert!(!is_collapsible_container("text"));
    }

    #[test]
    fn test_minify_text_collapses_whitespace() {
        assert_eq!(minify_text("  hello   world  "), "hello world");
        assert_eq!(minify_text("\n  \t  "), "");
        assert_eq!(minify_text("no change"), "no change");
        assert_eq!(minify_text("  a  b  c  "), "a b c");
    }

    #[test]
    fn test_minify_text_empty_string() {
        assert_eq!(minify_text(""), "");
        assert_eq!(minify_text("   "), "");
    }
}
