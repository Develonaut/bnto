// OptimizeSvg — NodeProcessor that strips editor cruft from SVG files.
//
// Runs 9 XML-level cleanup passes using roxmltree/xmlwriter.
// Zero new Cargo dependencies — both are already in the WASM binary
// via the resvg/usvg transitive chain.

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::metadata::*;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use super::{OptimizeConfig, optimize_svg};
use crate::common::svg_accepts;

/// SVG optimization processor — removes editor metadata, comments,
/// unused namespaces, empty containers, and collapses redundant groups.
pub struct OptimizeSvg;

impl NodeProcessor for OptimizeSvg {
    fn name(&self) -> &str {
        "vector-optimize"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "vector-optimize".to_string(),
            name: "Optimize SVG".to_string(),
            description:
                "Optimize SVG files by removing editor metadata, comments, and unnecessary elements"
                    .to_string(),
            category: NodeCategory::Vector,
            accepts: svg_accepts(),
            platforms: vec!["browser".to_string(), "native".to_string()],
            input_cardinality: InputCardinality::PerFile,
            parameters: vec![
                ParameterDef {
                    name: "removeComments".to_string(),
                    label: "Remove Comments".to_string(),
                    description: "Strip XML comments from the SVG".to_string(),
                    param_type: ParameterType::Boolean,
                    default: Some(serde_json::Value::Bool(true)),
                    constraints: None,
                    placeholder: None,
                    visible_when: None,
                    required_when: None,
                    surfaceable: true,
                },
                ParameterDef {
                    name: "removeMetadata".to_string(),
                    label: "Remove Metadata".to_string(),
                    description: "Strip <metadata> elements (RDF, Dublin Core, etc.)".to_string(),
                    param_type: ParameterType::Boolean,
                    default: Some(serde_json::Value::Bool(true)),
                    constraints: None,
                    placeholder: None,
                    visible_when: None,
                    required_when: None,
                    surfaceable: true,
                },
                ParameterDef {
                    name: "collapseGroups".to_string(),
                    label: "Collapse Groups".to_string(),
                    description: "Remove empty groups and collapse single-child wrapper groups"
                        .to_string(),
                    param_type: ParameterType::Boolean,
                    default: Some(serde_json::Value::Bool(true)),
                    constraints: None,
                    placeholder: None,
                    visible_when: None,
                    required_when: None,
                    surfaceable: true,
                },
                ParameterDef {
                    name: "minify".to_string(),
                    label: "Minify".to_string(),
                    description: "Remove unnecessary whitespace and newlines".to_string(),
                    param_type: ParameterType::Boolean,
                    default: Some(serde_json::Value::Bool(true)),
                    constraints: None,
                    placeholder: None,
                    visible_when: None,
                    required_when: None,
                    surfaceable: true,
                },
                ParameterDef {
                    name: "precision".to_string(),
                    label: "Numeric Precision".to_string(),
                    description: "Decimal places for numeric values (reserved for Tier 2)"
                        .to_string(),
                    param_type: ParameterType::Number,
                    default: Some(serde_json::json!(3)),
                    constraints: Some(Constraints {
                        min: Some(0.0),
                        max: Some(8.0),
                        required: false,
                    }),
                    placeholder: None,
                    visible_when: None,
                    required_when: None,
                    surfaceable: true,
                },
            ],
            requires: vec![],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let config = extract_config(&input.params);

        progress.report(10, "Parsing SVG...");
        let input_str = std::str::from_utf8(&input.data)
            .map_err(|e| BntoError::InvalidInput(format!("SVG is not valid UTF-8: {e}")))?;

        progress.report(30, "Optimizing SVG...");
        let optimized = optimize_svg(input_str, &config)
            .map_err(|e| BntoError::ProcessingFailed(format!("SVG optimization failed: {e}")))?;

        let input_size = input.data.len();
        let output_size = optimized.len();

        let metadata = build_metadata(input_size, output_size);

        progress.report(100, "SVG optimization complete");
        Ok(NodeOutput {
            files: vec![OutputFile {
                data: optimized.into_bytes(),
                filename: input.filename,
                mime_type: "image/svg+xml".to_string(),
            }],
            metadata,
        })
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        let mut errors = Vec::new();
        if let Some(precision) = params.get("precision").and_then(|v| v.as_u64())
            && precision > 8
        {
            errors.push(format!(
                "precision must be between 0 and 8, got {precision}"
            ));
        }
        errors
    }
}

fn extract_config(params: &serde_json::Map<String, serde_json::Value>) -> OptimizeConfig {
    OptimizeConfig {
        remove_comments: params
            .get("removeComments")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
        remove_metadata: params
            .get("removeMetadata")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
        collapse_groups: params
            .get("collapseGroups")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
        minify: params
            .get("minify")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
        precision: params
            .get("precision")
            .and_then(|v| v.as_u64())
            .unwrap_or(3) as u8,
    }
}

fn build_metadata(
    original_size: usize,
    compressed_size: usize,
) -> serde_json::Map<String, serde_json::Value> {
    let ratio = if original_size > 0 {
        (1.0 - (compressed_size as f64 / original_size as f64)) * 100.0
    } else {
        0.0
    };

    let mut meta = serde_json::Map::new();
    meta.insert("originalSize".to_string(), serde_json::json!(original_size));
    meta.insert(
        "compressedSize".to_string(),
        serde_json::json!(compressed_size),
    );
    if let Some(ratio_num) = serde_json::Number::from_f64(ratio) {
        meta.insert(
            "compressionRatio".to_string(),
            serde_json::Value::Number(ratio_num),
        );
    }
    meta
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NoopContext;

    fn make_input(params: serde_json::Map<String, serde_json::Value>) -> NodeInput {
        NodeInput {
            data: verbose_svg_bytes(),
            filename: "verbose.svg".to_string(),
            mime_type: Some("image/svg+xml".to_string()),
            params,
        }
    }

    fn verbose_svg_bytes() -> Vec<u8> {
        std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../test-fixtures/vector/verbose.svg"
        ))
        .expect("verbose.svg fixture must exist")
    }

    fn empty_params() -> serde_json::Map<String, serde_json::Value> {
        serde_json::Map::new()
    }

    #[test]
    fn test_name_matches_registry_key() {
        assert_eq!(OptimizeSvg.name(), "vector-optimize");
    }

    #[test]
    fn test_metadata_accepts_svg() {
        let meta = OptimizeSvg.metadata();
        assert!(meta.accepts.contains(&"image/svg+xml".to_string()));
    }

    #[test]
    fn test_metadata_has_five_params() {
        let meta = OptimizeSvg.metadata();
        assert_eq!(meta.parameters.len(), 5);
    }

    #[test]
    fn test_metadata_supports_browser_platform() {
        let meta = OptimizeSvg.metadata();
        assert!(meta.platforms.contains(&"browser".to_string()));
    }

    #[test]
    fn test_validate_passes_with_no_params() {
        let errors = OptimizeSvg.validate(&empty_params());
        assert!(errors.is_empty(), "Default params should pass validation");
    }

    #[test]
    fn test_validate_rejects_precision_too_high() {
        let mut params = empty_params();
        params.insert("precision".to_string(), serde_json::json!(9));
        let errors = OptimizeSvg.validate(&params);
        assert!(!errors.is_empty(), "precision=9 should fail validation");
    }

    #[test]
    fn test_process_produces_smaller_output() {
        let input = make_input(empty_params());
        let input_size = input.data.len();
        let reporter = ProgressReporter::new_noop();
        let output = OptimizeSvg.process(input, &reporter, &NoopContext).unwrap();

        assert_eq!(output.files.len(), 1);
        assert!(
            output.files[0].data.len() < input_size,
            "Output ({}) should be smaller than input ({input_size})",
            output.files[0].data.len(),
        );
    }

    #[test]
    fn test_process_preserves_filename() {
        let mut input = make_input(empty_params());
        input.filename = "icon.svg".to_string();
        let reporter = ProgressReporter::new_noop();
        let output = OptimizeSvg.process(input, &reporter, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "icon.svg");
    }

    #[test]
    fn test_process_returns_svg_mime_type() {
        let input = make_input(empty_params());
        let reporter = ProgressReporter::new_noop();
        let output = OptimizeSvg.process(input, &reporter, &NoopContext).unwrap();

        assert_eq!(output.files[0].mime_type, "image/svg+xml");
    }

    #[test]
    fn test_process_metadata_has_savings() {
        let input = make_input(empty_params());
        let reporter = ProgressReporter::new_noop();
        let output = OptimizeSvg.process(input, &reporter, &NoopContext).unwrap();

        assert!(output.metadata.contains_key("originalSize"));
        assert!(output.metadata.contains_key("compressedSize"));
        assert!(output.metadata.contains_key("compressionRatio"));
    }

    #[test]
    fn test_process_with_different_params_produces_different_output() {
        let reporter = ProgressReporter::new_noop();

        let out_default = OptimizeSvg
            .process(make_input(empty_params()), &reporter, &NoopContext)
            .unwrap();

        let mut params = empty_params();
        params.insert("removeComments".to_string(), serde_json::json!(false));
        let out_comments = OptimizeSvg
            .process(make_input(params), &reporter, &NoopContext)
            .unwrap();

        assert_ne!(
            out_default.files[0].data, out_comments.files[0].data,
            "Different params should produce different output"
        );
    }

    #[test]
    fn test_process_reports_minimum_50pct_savings() {
        let input = make_input(empty_params());
        let reporter = ProgressReporter::new_noop();
        let output = OptimizeSvg.process(input, &reporter, &NoopContext).unwrap();

        let ratio = output
            .metadata
            .get("compressionRatio")
            .and_then(|v| v.as_f64())
            .expect("compressionRatio should be in metadata");
        assert!(
            ratio >= 50.0,
            "Expected >=50% savings on verbose SVG fixture, got {ratio}%"
        );
    }

    #[test]
    fn test_process_invalid_utf8_returns_error() {
        let input = NodeInput {
            data: vec![0xFF, 0xFE, 0x00],
            filename: "bad.svg".to_string(),
            mime_type: Some("image/svg+xml".to_string()),
            params: empty_params(),
        };
        let reporter = ProgressReporter::new_noop();
        let result = OptimizeSvg.process(input, &reporter, &NoopContext);
        assert!(result.is_err());
    }
}
