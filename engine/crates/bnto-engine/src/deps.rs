// Dependency checker — verifies that external tools required by processors
// are available on the host system. WASM has no shell, so this module is
// only useful in CLI/desktop contexts where ProcessContext is a NativeContext.

use bnto_core::{BntoError, Dependency, NodeRegistry, PipelineDefinition};
use std::collections::HashSet;

/// Result of checking a single dependency.
#[derive(Debug, Clone)]
pub struct DependencyStatus {
    pub dependency: Dependency,
    pub found: bool,
}

/// Collect unique dependencies required by a pipeline definition.
///
/// Merges two sources: recipe-level deps (`definition.requires`) first,
/// then per-node processor deps (from `metadata().requires`). Deduplicates
/// by binary name — recipe-level deps take precedence when both declare
/// the same binary.
pub fn collect_pipeline_dependencies(
    definition: &PipelineDefinition,
    registry: &NodeRegistry,
) -> Vec<Dependency> {
    let mut seen = HashSet::new();
    let mut deps = Vec::new();

    // Recipe-level deps first — these take precedence.
    for dep in &definition.requires {
        if seen.insert(dep.binary.clone()) {
            deps.push(dep.clone());
        }
    }

    // Then per-node processor deps (existing logic).
    collect_from_nodes(&definition.nodes, registry, &mut seen, &mut deps);
    deps
}

fn collect_from_nodes(
    nodes: &[bnto_core::PipelineNode],
    registry: &NodeRegistry,
    seen: &mut HashSet<String>,
    deps: &mut Vec<Dependency>,
) {
    let empty_params = serde_json::Map::new();
    for node in nodes {
        if let Some(processor) = registry.resolve(&node.node_type, &empty_params) {
            for dep in &processor.metadata().requires {
                if seen.insert(dep.binary.clone()) {
                    deps.push(dep.clone());
                }
            }
        }
        // Recurse into container children.
        if let Some(children) = &node.children {
            collect_from_nodes(children, registry, seen, deps);
        }
    }
}

/// Collect unique dependencies from ALL registered processors in the registry.
pub fn collect_all_dependencies(registry: &NodeRegistry) -> Vec<Dependency> {
    let mut seen = HashSet::new();
    let mut deps = Vec::new();
    for metadata in registry.catalog() {
        for dep in metadata.requires {
            if seen.insert(dep.binary.clone()) {
                deps.push(dep);
            }
        }
    }
    deps
}

/// Check whether each dependency's binary is available on the system.
///
/// Uses `which <binary>` to probe the PATH. Returns a status for each
/// dependency indicating whether it was found.
pub fn check_dependencies(
    deps: &[Dependency],
    ctx: &dyn bnto_core::ProcessContext,
) -> Vec<DependencyStatus> {
    deps.iter()
        .map(|dep| {
            let found = ctx.run_command("which", &[&dep.binary]).is_ok();
            DependencyStatus {
                dependency: dep.clone(),
                found,
            }
        })
        .collect()
}

/// Check dependencies for a pipeline definition and return an error if any are missing.
///
/// Intended as a pre-flight check before `run_pipeline()`. Returns `Ok(())`
/// when all dependencies are satisfied, or `Err(BntoError)` listing the
/// missing binaries with install hints.
pub fn check_pipeline_dependencies(
    definition: &PipelineDefinition,
    registry: &NodeRegistry,
    ctx: &dyn bnto_core::ProcessContext,
) -> Result<(), BntoError> {
    let deps = collect_pipeline_dependencies(definition, registry);
    if deps.is_empty() {
        return Ok(());
    }

    let statuses = check_dependencies(&deps, ctx);
    let missing: Vec<&DependencyStatus> = statuses.iter().filter(|s| !s.found).collect();

    if missing.is_empty() {
        return Ok(());
    }

    let messages: Vec<String> = missing
        .iter()
        .map(|s| {
            let hint = &s.dependency.install_hint;
            format!("  - {} (install: {})", s.dependency.binary, hint)
        })
        .collect();

    Err(BntoError::InvalidInput(format!(
        "Missing required dependencies:\n{}",
        messages.join("\n")
    )))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NodeProcessor;
    use bnto_core::NoopContext;
    use bnto_core::context::ProcessContext;
    use bnto_core::errors::BntoError;
    use bnto_core::metadata::{Dependency, InputCardinality, NodeCategory, NodeMetadata};
    use bnto_core::processor::{NodeInput, NodeOutput, OutputFile};
    use bnto_core::progress::ProgressReporter;
    use std::path::{Path, PathBuf};

    // --- Mock processor that declares dependencies ---

    struct FfmpegProcessor;

    impl NodeProcessor for FfmpegProcessor {
        fn name(&self) -> &str {
            "video-transcode"
        }

        fn process(
            &self,
            input: NodeInput,
            _progress: &ProgressReporter,
            _ctx: &dyn ProcessContext,
        ) -> Result<NodeOutput, BntoError> {
            Ok(NodeOutput {
                files: vec![OutputFile {
                    data: input.data,
                    filename: input.filename,
                    mime_type: "video/mp4".to_string(),
                }],
                metadata: serde_json::Map::new(),
            })
        }

        fn metadata(&self) -> NodeMetadata {
            NodeMetadata {
                node_type: "video-transcode".to_string(),
                name: "Transcode Video".to_string(),
                description: "Transcode video using ffmpeg.".to_string(),
                category: NodeCategory::Data,
                accepts: vec!["video/*".to_string()],
                platforms: vec!["cli".to_string()],
                parameters: vec![],
                input_cardinality: InputCardinality::PerFile,
                requires: vec![Dependency {
                    binary: "ffmpeg".to_string(),
                    version: ">=6.0".to_string(),
                    install_hint: "brew install ffmpeg".to_string(),
                    homepage: "https://ffmpeg.org".to_string(),
                }],
            }
        }
    }

    struct YtDlpProcessor;

    impl NodeProcessor for YtDlpProcessor {
        fn name(&self) -> &str {
            "video-download"
        }

        fn process(
            &self,
            input: NodeInput,
            _progress: &ProgressReporter,
            _ctx: &dyn ProcessContext,
        ) -> Result<NodeOutput, BntoError> {
            Ok(NodeOutput {
                files: vec![OutputFile {
                    data: input.data,
                    filename: input.filename,
                    mime_type: "video/mp4".to_string(),
                }],
                metadata: serde_json::Map::new(),
            })
        }

        fn metadata(&self) -> NodeMetadata {
            NodeMetadata {
                node_type: "video-download".to_string(),
                name: "Download Video".to_string(),
                description: "Download video using yt-dlp.".to_string(),
                category: NodeCategory::Data,
                accepts: vec![],
                platforms: vec!["cli".to_string()],
                parameters: vec![],
                input_cardinality: InputCardinality::PerFile,
                requires: vec![
                    Dependency {
                        binary: "yt-dlp".to_string(),
                        version: String::new(),
                        install_hint: "brew install yt-dlp".to_string(),
                        homepage: "https://github.com/yt-dlp/yt-dlp".to_string(),
                    },
                    Dependency {
                        binary: "ffmpeg".to_string(),
                        version: ">=6.0".to_string(),
                        install_hint: "brew install ffmpeg".to_string(),
                        homepage: "https://ffmpeg.org".to_string(),
                    },
                ],
            }
        }
    }

    /// A no-dep processor (like existing browser-only ones).
    struct NoDepsProcessor;

    impl NodeProcessor for NoDepsProcessor {
        fn name(&self) -> &str {
            "no-deps"
        }

        fn process(
            &self,
            input: NodeInput,
            _progress: &ProgressReporter,
            _ctx: &dyn ProcessContext,
        ) -> Result<NodeOutput, BntoError> {
            Ok(NodeOutput {
                files: vec![OutputFile {
                    data: input.data,
                    filename: input.filename,
                    mime_type: "application/octet-stream".to_string(),
                }],
                metadata: serde_json::Map::new(),
            })
        }
    }

    /// Mock context where `which` always fails (simulates missing deps).
    struct AllMissingContext;

    impl ProcessContext for AllMissingContext {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            Err(BntoError::ProcessingFailed("not found".to_string()))
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, BntoError> {
            Err(BntoError::ProcessingFailed("not available".to_string()))
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Err(BntoError::ProcessingFailed("not available".to_string()))
        }
    }

    /// Mock context where `which` always succeeds.
    struct AllFoundContext;

    impl ProcessContext for AllFoundContext {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            Ok(b"/usr/local/bin/found".to_vec())
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, BntoError> {
            Err(BntoError::ProcessingFailed("not available".to_string()))
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Err(BntoError::ProcessingFailed("not available".to_string()))
        }
    }

    fn make_definition(node_types: &[&str]) -> PipelineDefinition {
        let json = serde_json::json!({
            "nodes": node_types.iter().enumerate().map(|(i, t)| {
                serde_json::json!({ "id": format!("n{i}"), "type": t })
            }).collect::<Vec<_>>()
        });
        serde_json::from_value(json).unwrap()
    }

    /// Build a definition with recipe-level `requires` and the given nodes.
    fn make_definition_with_requires(
        node_types: &[&str],
        requires: Vec<Dependency>,
    ) -> PipelineDefinition {
        let mut def = make_definition(node_types);
        def.requires = requires;
        def
    }

    fn ytdlp_dep() -> Dependency {
        Dependency {
            binary: "yt-dlp".to_string(),
            version: String::new(),
            install_hint: "brew install yt-dlp".to_string(),
            homepage: String::new(),
        }
    }

    fn ffmpeg_dep() -> Dependency {
        Dependency {
            binary: "ffmpeg".to_string(),
            version: ">=6.0".to_string(),
            install_hint: "brew install ffmpeg".to_string(),
            homepage: "https://ffmpeg.org".to_string(),
        }
    }

    // --- collect_pipeline_dependencies ---

    #[test]
    fn test_collect_empty_pipeline_returns_no_deps() {
        let def = make_definition(&["input", "output"]);
        let registry = NodeRegistry::new();
        let deps = collect_pipeline_dependencies(&def, &registry);
        assert!(deps.is_empty());
    }

    #[test]
    fn test_collect_pipeline_with_no_dep_processor() {
        let mut registry = NodeRegistry::new();
        registry.register("no-deps", Box::new(NoDepsProcessor));
        let def = make_definition(&["input", "no-deps", "output"]);
        let deps = collect_pipeline_dependencies(&def, &registry);
        assert!(deps.is_empty());
    }

    #[test]
    fn test_collect_pipeline_with_ffmpeg_dep() {
        let mut registry = NodeRegistry::new();
        registry.register("video-transcode", Box::new(FfmpegProcessor));
        let def = make_definition(&["input", "video-transcode", "output"]);
        let deps = collect_pipeline_dependencies(&def, &registry);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].binary, "ffmpeg");
    }

    #[test]
    fn test_collect_deduplicates_shared_deps() {
        let mut registry = NodeRegistry::new();
        registry.register("video-transcode", Box::new(FfmpegProcessor));
        registry.register("video-download", Box::new(YtDlpProcessor));
        let def = make_definition(&["input", "video-transcode", "video-download", "output"]);
        let deps = collect_pipeline_dependencies(&def, &registry);
        // ffmpeg appears in both, but should be deduplicated
        assert_eq!(deps.len(), 2); // ffmpeg + yt-dlp
        let binaries: Vec<&str> = deps.iter().map(|d| d.binary.as_str()).collect();
        assert!(binaries.contains(&"ffmpeg"));
        assert!(binaries.contains(&"yt-dlp"));
    }

    // --- collect_pipeline_dependencies: recipe-level requires ---

    #[test]
    fn test_collect_recipe_only_deps() {
        // Recipe declares deps but no nodes have processor deps.
        let def = make_definition_with_requires(&["input", "output"], vec![ytdlp_dep()]);
        let registry = NodeRegistry::new();
        let deps = collect_pipeline_dependencies(&def, &registry);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].binary, "yt-dlp");
    }

    #[test]
    fn test_collect_node_only_deps_unchanged() {
        // Recipe has no requires but nodes have processor deps.
        // This is the existing behavior — must still work.
        let mut registry = NodeRegistry::new();
        registry.register("video-transcode", Box::new(FfmpegProcessor));
        let def = make_definition(&["input", "video-transcode", "output"]);
        let deps = collect_pipeline_dependencies(&def, &registry);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].binary, "ffmpeg");
    }

    #[test]
    fn test_collect_merged_recipe_and_node_deps() {
        // Recipe declares yt-dlp, node declares ffmpeg — both should appear.
        let mut registry = NodeRegistry::new();
        registry.register("video-transcode", Box::new(FfmpegProcessor));
        let def = make_definition_with_requires(
            &["input", "video-transcode", "output"],
            vec![ytdlp_dep()],
        );
        let deps = collect_pipeline_dependencies(&def, &registry);
        assert_eq!(deps.len(), 2);
        let binaries: Vec<&str> = deps.iter().map(|d| d.binary.as_str()).collect();
        assert!(binaries.contains(&"yt-dlp"));
        assert!(binaries.contains(&"ffmpeg"));
    }

    #[test]
    fn test_collect_recipe_deps_come_first() {
        // Recipe-level deps should appear before node-level deps in the list.
        let mut registry = NodeRegistry::new();
        registry.register("video-transcode", Box::new(FfmpegProcessor));
        let def = make_definition_with_requires(
            &["input", "video-transcode", "output"],
            vec![ytdlp_dep()],
        );
        let deps = collect_pipeline_dependencies(&def, &registry);
        assert_eq!(
            deps[0].binary, "yt-dlp",
            "Recipe-level dep should come first"
        );
        assert_eq!(
            deps[1].binary, "ffmpeg",
            "Node-level dep should come second"
        );
    }

    #[test]
    fn test_collect_deduplicated_recipe_and_node_deps() {
        // Both recipe and node declare ffmpeg — should appear only once.
        let mut registry = NodeRegistry::new();
        registry.register("video-transcode", Box::new(FfmpegProcessor));
        let def = make_definition_with_requires(
            &["input", "video-transcode", "output"],
            vec![ffmpeg_dep()],
        );
        let deps = collect_pipeline_dependencies(&def, &registry);
        assert_eq!(deps.len(), 1, "Duplicate ffmpeg should be deduplicated");
        assert_eq!(deps[0].binary, "ffmpeg");
    }

    #[test]
    fn test_collect_empty_recipe_requires() {
        // Recipe has explicit empty requires — should behave like no requires.
        let def = make_definition_with_requires(&["input", "output"], vec![]);
        let registry = NodeRegistry::new();
        let deps = collect_pipeline_dependencies(&def, &registry);
        assert!(deps.is_empty());
    }

    #[test]
    fn test_check_pipeline_dependencies_catches_recipe_deps() {
        // Pre-flight check should catch missing recipe-level deps too.
        let def = make_definition_with_requires(&["input", "output"], vec![ytdlp_dep()]);
        let registry = NodeRegistry::new();
        let result = check_pipeline_dependencies(&def, &registry, &AllMissingContext);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("yt-dlp"));
        assert!(err_msg.contains("brew install yt-dlp"));
    }

    // --- collect_all_dependencies ---

    #[test]
    fn test_collect_all_from_empty_registry() {
        let registry = NodeRegistry::new();
        let deps = collect_all_dependencies(&registry);
        assert!(deps.is_empty());
    }

    #[test]
    fn test_collect_all_deduplicates() {
        let mut registry = NodeRegistry::new();
        registry.register("video-transcode", Box::new(FfmpegProcessor));
        registry.register("video-download", Box::new(YtDlpProcessor));
        registry.register("no-deps", Box::new(NoDepsProcessor));
        let deps = collect_all_dependencies(&registry);
        assert_eq!(deps.len(), 2); // ffmpeg + yt-dlp (deduplicated)
    }

    // --- check_dependencies ---

    #[test]
    fn test_check_all_missing() {
        let deps = vec![Dependency {
            binary: "ffmpeg".to_string(),
            version: String::new(),
            install_hint: "brew install ffmpeg".to_string(),
            homepage: String::new(),
        }];
        let statuses = check_dependencies(&deps, &AllMissingContext);
        assert_eq!(statuses.len(), 1);
        assert!(!statuses[0].found);
    }

    #[test]
    fn test_check_all_found() {
        let deps = vec![Dependency {
            binary: "ffmpeg".to_string(),
            version: String::new(),
            install_hint: "brew install ffmpeg".to_string(),
            homepage: String::new(),
        }];
        let statuses = check_dependencies(&deps, &AllFoundContext);
        assert_eq!(statuses.len(), 1);
        assert!(statuses[0].found);
    }

    #[test]
    fn test_check_empty_deps_returns_empty() {
        let statuses = check_dependencies(&[], &NoopContext);
        assert!(statuses.is_empty());
    }

    // --- check_pipeline_dependencies ---

    #[test]
    fn test_preflight_no_deps_ok() {
        let mut registry = NodeRegistry::new();
        registry.register("no-deps", Box::new(NoDepsProcessor));
        let def = make_definition(&["input", "no-deps", "output"]);
        let result = check_pipeline_dependencies(&def, &registry, &NoopContext);
        assert!(result.is_ok());
    }

    #[test]
    fn test_preflight_missing_dep_returns_error() {
        let mut registry = NodeRegistry::new();
        registry.register("video-transcode", Box::new(FfmpegProcessor));
        let def = make_definition(&["input", "video-transcode", "output"]);
        let result = check_pipeline_dependencies(&def, &registry, &AllMissingContext);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("ffmpeg"));
        assert!(err_msg.contains("brew install ffmpeg"));
    }

    #[test]
    fn test_preflight_all_found_ok() {
        let mut registry = NodeRegistry::new();
        registry.register("video-transcode", Box::new(FfmpegProcessor));
        let def = make_definition(&["input", "video-transcode", "output"]);
        let result = check_pipeline_dependencies(&def, &registry, &AllFoundContext);
        assert!(result.is_ok());
    }

    #[test]
    fn test_preflight_error_lists_all_missing() {
        let mut registry = NodeRegistry::new();
        registry.register("video-download", Box::new(YtDlpProcessor));
        let def = make_definition(&["input", "video-download", "output"]);
        let result = check_pipeline_dependencies(&def, &registry, &AllMissingContext);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("yt-dlp"));
        assert!(err_msg.contains("ffmpeg"));
    }
}
