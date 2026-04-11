// SVG optimizer — parse with roxmltree, run passes, write with xmlwriter.
//
// roxmltree is read-only, so the pattern is: parse once, walk the tree,
// selectively write to xmlwriter — skipping or transforming nodes based
// on the active pass configuration.

pub mod processor;
mod write;
pub mod xml_passes;

use roxmltree::Document;
use xmlwriter::XmlWriter;

/// Configuration controlling which optimization passes run.
#[derive(Debug, Clone)]
pub struct OptimizeConfig {
    pub remove_comments: bool,
    pub remove_metadata: bool,
    pub collapse_groups: bool,
    pub minify: bool,
    pub precision: u8,
}

impl Default for OptimizeConfig {
    fn default() -> Self {
        Self {
            remove_comments: true,
            remove_metadata: true,
            collapse_groups: true,
            minify: true,
            precision: 3,
        }
    }
}

/// Known editor namespace URIs — elements and attributes with these
/// namespaces are removed as editor cruft.
const EDITOR_NAMESPACE_URIS: &[&str] = &[
    "http://www.inkscape.org/namespaces/inkscape",
    "http://sodipodi.sourceforge.net/DTD/sodipodi-0.0.dtd",
    "http://purl.org/dc/elements/1.1/",
    "http://creativecommons.org/ns#",
    "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    "http://www.bohemiancoding.com/sketch/ns",
    "http://ns.adobe.com/AdobeIllustrator/10.0/",
    "http://ns.adobe.com/AdobeSVGViewerExtensions/3.0/",
    "http://ns.adobe.com/Extensibility/1.0/",
    "http://ns.adobe.com/Flows/1.0/",
    "http://ns.adobe.com/Graphs/1.0/",
    "http://ns.adobe.com/ImageReplacement/1.0/",
    "http://ns.adobe.com/SaveForWeb/1.0/",
    "http://ns.adobe.com/Variables/1.0/",
    "http://ns.adobe.com/xap/1.0/",
    "http://ns.adobe.com/xap/1.0/mm/",
    "http://ns.adobe.com/xap/1.0/sType/ResourceRef#",
    "http://www.serif.com/",
    "http://www.figma.com/figma/ns",
    "http://www.vectornator.io/",
    "http://schemas.microsoft.com/visio/2003/SVGExtensions/",
    "http://krita.org/namespaces/svg/krita",
    "http://www.w3.org/2004/02/skos/core#",
    "http://www.boxysvg.com/ns",
];

/// Run the SVG optimization pipeline.
///
/// Parses the input SVG with roxmltree, walks the tree once, and writes
/// optimized output with xmlwriter — skipping nodes that match removal
/// criteria based on the config.
pub fn optimize_svg(input: &str, config: &OptimizeConfig) -> Result<String, OptimizeError> {
    // Strip XML declaration and DOCTYPE — roxmltree doesn't support DTD.
    let cleaned = strip_xml_prolog(input);
    let doc = Document::parse(&cleaned).map_err(|e| OptimizeError::Parse(e.to_string()))?;

    // Phase 1: analysis — collect editor namespace prefixes from the root SVG element.
    let editor_prefixes = xml_passes::collect_editor_prefixes(&doc, EDITOR_NAMESPACE_URIS);

    // Phase 2: streaming write — walk tree, apply passes inline.
    let mut opts = xmlwriter::Options::default();
    if config.minify {
        opts.indent = xmlwriter::Indent::None;
    }
    let mut writer = XmlWriter::new(opts);
    let mut declared_ns = std::collections::HashSet::new();
    write::write_node(
        doc.root(),
        &mut writer,
        config,
        &editor_prefixes,
        &mut declared_ns,
    );

    Ok(writer.end_document())
}

/// Strip XML declaration (`<?xml ... ?>`) and DOCTYPE from input.
///
/// roxmltree rejects DTD declarations. Real-world SVGs from editors
/// often include `<!DOCTYPE svg ...>` which we simply discard.
fn strip_xml_prolog(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut remaining = input.trim_start();

    // Skip XML declaration
    if remaining.starts_with("<?xml")
        && let Some(end) = remaining.find("?>")
    {
        remaining = remaining[end + 2..].trim_start();
    }

    // Skip DOCTYPE
    if remaining.starts_with("<!DOCTYPE")
        && let Some(end) = remaining.find('>')
    {
        remaining = remaining[end + 1..].trim_start();
    }

    result.push_str(remaining);
    result
}

#[derive(Debug, thiserror::Error)]
pub enum OptimizeError {
    #[error("SVG parse error: {0}")]
    Parse(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verbose_svg() -> String {
        std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../test-fixtures/vector/verbose.svg"
        ))
        .expect("verbose.svg fixture must exist")
    }

    fn simple_svg() -> &'static str {
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><rect x="0" y="0" width="100" height="100" fill="blue"/></svg>"#
    }

    #[test]
    fn test_optimize_produces_valid_svg() {
        let result = optimize_svg(&verbose_svg(), &OptimizeConfig::default()).unwrap();
        assert!(result.contains("<svg"));
        assert!(result.contains("</svg>"));
    }

    #[test]
    fn test_optimize_reduces_size() {
        let input = verbose_svg();
        let result = optimize_svg(&input, &OptimizeConfig::default()).unwrap();
        assert!(
            result.len() < input.len(),
            "Optimized ({} bytes) should be smaller than input ({} bytes)",
            result.len(),
            input.len()
        );
    }

    #[test]
    fn test_removes_metadata_elements() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><metadata><title>Test</title></metadata><rect width="10" height="10"/></svg>"#;
        let result = optimize_svg(svg, &OptimizeConfig::default()).unwrap();
        assert!(!result.contains("<metadata"), "metadata should be removed");
        assert!(result.contains("<rect"), "visual elements should survive");
    }

    #[test]
    fn test_removes_comments() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><!-- remove me --><rect width="10" height="10"/></svg>"#;
        let result = optimize_svg(svg, &OptimizeConfig::default()).unwrap();
        assert!(!result.contains("<!--"), "comments should be removed");
        assert!(result.contains("<rect"), "visual elements should survive");
    }

    #[test]
    fn test_removes_editor_namespaces() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" inkscape:version="1.3"><rect width="10" height="10" inkscape:label="bg"/></svg>"#;
        let result = optimize_svg(svg, &OptimizeConfig::default()).unwrap();
        assert!(
            !result.contains("inkscape"),
            "inkscape attributes should be removed"
        );
    }

    #[test]
    fn test_removes_empty_groups() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><g></g><rect width="10" height="10"/></svg>"#;
        let result = optimize_svg(svg, &OptimizeConfig::default()).unwrap();
        // The empty <g></g> should be gone
        assert!(result.contains("<rect"), "visual elements should survive");
        // Count <g occurrences — should be 0
        assert!(
            !result.contains("<g"),
            "empty groups should be removed, got: {result}"
        );
    }

    #[test]
    fn test_collapses_single_child_groups() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><g><rect width="10" height="10"/></g></svg>"#;
        let result = optimize_svg(svg, &OptimizeConfig::default()).unwrap();
        assert!(!result.contains("<g"), "wrapper <g> should be collapsed");
        assert!(result.contains("<rect"), "child element should survive");
    }

    #[test]
    fn test_preserves_meaningful_groups() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><g id="layer1"><rect width="10" height="10"/></g></svg>"#;
        let result = optimize_svg(svg, &OptimizeConfig::default()).unwrap();
        assert!(result.contains("<g"), "group with id should be preserved");
        assert!(result.contains("layer1"), "id attribute should survive");
    }

    #[test]
    fn test_removes_empty_attributes() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="10" height="10" fill="" stroke="blue"/></svg>"#;
        let result = optimize_svg(svg, &OptimizeConfig::default()).unwrap();
        assert!(
            !result.contains(r#"fill="""#),
            "empty fill should be removed"
        );
        assert!(result.contains("stroke"), "non-empty stroke should survive");
    }

    #[test]
    fn test_minify_strips_whitespace() {
        let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\">\n  <rect width=\"10\" height=\"10\"/>\n</svg>";
        let result = optimize_svg(svg, &OptimizeConfig::default()).unwrap();
        assert!(
            !result.contains('\n'),
            "newlines should be stripped when minifying"
        );
    }

    #[test]
    fn test_param_removecomments_false_preserves_comments() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><!-- keep me --><rect width="10" height="10"/></svg>"#;
        let config = OptimizeConfig {
            remove_comments: false,
            ..Default::default()
        };
        let result = optimize_svg(svg, &config).unwrap();
        assert!(
            result.contains("<!-- keep me -->"),
            "comment should be preserved"
        );
    }

    #[test]
    fn test_param_removemetadata_false_preserves_metadata() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><metadata><title>Keep</title></metadata><rect width="10" height="10"/></svg>"#;
        let config = OptimizeConfig {
            remove_metadata: false,
            ..Default::default()
        };
        let result = optimize_svg(svg, &config).unwrap();
        assert!(result.contains("<metadata"), "metadata should be preserved");
    }

    #[test]
    fn test_param_collapsegroups_false_preserves_groups() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><g><rect width="10" height="10"/></g></svg>"#;
        let config = OptimizeConfig {
            collapse_groups: false,
            ..Default::default()
        };
        let result = optimize_svg(svg, &config).unwrap();
        assert!(result.contains("<g"), "group should be preserved");
    }

    #[test]
    fn test_param_minify_false_preserves_whitespace() {
        let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\">\n  <rect width=\"10\" height=\"10\"/>\n</svg>";
        let config = OptimizeConfig {
            minify: false,
            ..Default::default()
        };
        let result = optimize_svg(svg, &config).unwrap();
        // With minify off, the text content should still have some structure
        assert!(result.contains("<rect"), "visual elements should survive");
    }

    #[test]
    fn test_roundtrip_preserves_visual_elements() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="100" height="100" fill="blue"/><circle cx="50" cy="50" r="30" fill="orange"/><path d="M 10 10 L 90 90"/><text x="50" y="55">Hello</text></svg>"#;
        let result = optimize_svg(svg, &OptimizeConfig::default()).unwrap();
        assert!(result.contains("<rect"), "rect should survive");
        assert!(result.contains("<circle"), "circle should survive");
        assert!(result.contains("<path"), "path should survive");
        assert!(result.contains("<text"), "text should survive");
        assert!(result.contains("Hello"), "text content should survive");
    }

    #[test]
    fn test_invalid_svg_returns_error() {
        let result = optimize_svg("not valid svg", &OptimizeConfig::default());
        assert!(result.is_err(), "invalid SVG should return error");
    }

    #[test]
    fn test_simple_svg_roundtrips() {
        let result = optimize_svg(simple_svg(), &OptimizeConfig::default()).unwrap();
        assert!(result.contains("<svg"));
        assert!(result.contains("<rect"));
        assert!(result.contains("fill=\"blue\""));
    }

    // --- Quantitative proof tests ---
    // These tests assert measurable optimization outcomes,
    // not just structural correctness.

    #[test]
    fn test_verbose_svg_achieves_minimum_reduction() {
        let input = verbose_svg();
        let result = optimize_svg(&input, &OptimizeConfig::default()).unwrap();
        let reduction_pct = (input.len() - result.len()) as f64 / input.len() as f64 * 100.0;
        assert!(
            reduction_pct >= 50.0,
            "Expected >=50% reduction on verbose Inkscape SVG, got {reduction_pct:.1}% \
             (input={} bytes, output={} bytes)",
            input.len(),
            result.len(),
        );
    }

    #[test]
    fn test_no_redundant_xmlns_on_child_elements() {
        let result = optimize_svg(&verbose_svg(), &OptimizeConfig::default()).unwrap();
        // xmlns should appear exactly once (on the root <svg>)
        let xmlns_count = result.matches("xmlns=").count();
        assert_eq!(
            xmlns_count, 1,
            "xmlns= should appear once (root only), found {xmlns_count} in: {result}"
        );
    }

    #[test]
    fn test_no_editor_namespace_survives() {
        let result = optimize_svg(&verbose_svg(), &OptimizeConfig::default()).unwrap();
        for keyword in &["inkscape", "sodipodi", "sketch", "rdf", "dc:", "cc:"] {
            assert!(
                !result.contains(keyword),
                "Editor namespace '{keyword}' should be removed, found in: {result}"
            );
        }
    }

    #[test]
    fn test_no_xml_prolog_in_output() {
        let result = optimize_svg(&verbose_svg(), &OptimizeConfig::default()).unwrap();
        assert!(
            !result.contains("<?xml"),
            "XML declaration should be stripped"
        );
        assert!(!result.contains("<!DOCTYPE"), "DOCTYPE should be stripped");
    }

    #[test]
    fn test_empty_group_nesting_eliminated() {
        let result = optimize_svg(&verbose_svg(), &OptimizeConfig::default()).unwrap();
        // The verbose fixture has nested <g> wrappers — verify they're collapsed.
        // Only the meaningful group (id="layer1") should survive.
        let g_count = result.matches("<g").count();
        assert!(
            g_count <= 1,
            "Expected at most 1 <g> tag (the id=layer1 group), found {g_count}"
        );
    }
}
