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
    /// Installed version string (if detected via `--version`).
    pub installed_version: Option<String>,
    /// Whether the installed version satisfies the constraint.
    /// `None` if no constraint was specified or version couldn't be determined.
    pub version_satisfied: Option<bool>,
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

/// Check whether each dependency's binary is available on the system
/// and whether its version satisfies any declared constraint.
///
/// Probes the PATH with the platform's lookup command
/// (`which` on Unix, `where` on Windows). If the dependency has a
/// non-empty `version` constraint, runs `<binary> --version` and
/// validates the output against the constraint.
pub fn check_dependencies(
    deps: &[Dependency],
    ctx: &dyn bnto_core::ProcessContext,
) -> Vec<DependencyStatus> {
    deps.iter()
        .map(|dep| {
            let found = ctx
                .run_command(bnto_core::probe_command(), &[&dep.binary])
                .is_ok();

            // Only check version if the binary exists and has a constraint.
            let (installed_version, version_satisfied) = if found && !dep.version.is_empty() {
                match bnto_core::check_version(&dep.binary, &dep.version, ctx) {
                    bnto_core::VersionCheckResult::Checked {
                        installed,
                        satisfied,
                    } => (Some(installed), Some(satisfied)),
                    bnto_core::VersionCheckResult::Skipped => (None, None),
                }
            } else {
                (None, None)
            };

            DependencyStatus {
                dependency: dep.clone(),
                found,
                installed_version,
                version_satisfied,
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

    let mut messages: Vec<String> = Vec::new();

    for s in &statuses {
        if !s.found {
            let hint = &s.dependency.install_hint;
            messages.push(format!("  - {} (install: {})", s.dependency.binary, hint));
        } else if s.version_satisfied == Some(false) {
            let installed = s.installed_version.as_deref().unwrap_or("unknown");
            messages.push(format!(
                "  - {} (installed: {}, requires: {})",
                s.dependency.binary, installed, s.dependency.version
            ));
        }
    }

    if messages.is_empty() {
        return Ok(());
    }

    Err(BntoError::InvalidInput(format!(
        "Dependency requirements not met:\n{}",
        messages.join("\n")
    )))
}

/// Check that all required secrets declared by a pipeline are available.
///
/// Parallel to `check_pipeline_dependencies()` — this is the pre-flight
/// check for env vars. Returns `Ok(())` when all required secrets are
/// present, or `Err(BntoError)` listing the missing ones.
pub fn check_pipeline_secrets(
    definition: &PipelineDefinition,
    ctx: &dyn bnto_core::ProcessContext,
) -> Result<(), BntoError> {
    if definition.secrets.is_empty() {
        return Ok(());
    }

    let statuses = bnto_core::secrets::check_secrets(&definition.secrets, ctx);
    let missing = bnto_core::secrets::missing_required(&statuses);

    if missing.is_empty() {
        return Ok(());
    }

    Err(BntoError::InvalidInput(
        bnto_core::secrets::format_missing_error(&missing),
    ))
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
                    metadata: serde_json::Map::new(),
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
                    metadata: serde_json::Map::new(),
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
                    metadata: serde_json::Map::new(),
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

    // --- Version constraint integration ---

    /// Mock context that returns a path for `which` and version output
    /// for `<binary> --version`. Simulates a system with ffmpeg 6.1.1.
    struct VersionedContext {
        version_output: String,
    }

    impl VersionedContext {
        fn new(version_output: &str) -> Self {
            Self {
                version_output: version_output.to_string(),
            }
        }
    }

    impl ProcessContext for VersionedContext {
        fn run_command(&self, cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            if cmd == bnto_core::probe_command() {
                Ok(b"/usr/local/bin/found".to_vec())
            } else {
                Ok(self.version_output.as_bytes().to_vec())
            }
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, BntoError> {
            Err(BntoError::ProcessingFailed("mock".to_string()))
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Err(BntoError::ProcessingFailed("mock".to_string()))
        }
    }

    #[test]
    fn test_check_deps_version_satisfied() {
        let ctx = VersionedContext::new("ffmpeg version 6.1.1 Copyright");
        let deps = vec![ffmpeg_dep()]; // requires >=6.0
        let statuses = check_dependencies(&deps, &ctx);
        assert_eq!(statuses.len(), 1);
        assert!(statuses[0].found);
        assert_eq!(statuses[0].installed_version.as_deref(), Some("6.1.1"));
        assert_eq!(statuses[0].version_satisfied, Some(true));
    }

    #[test]
    fn test_check_deps_version_unsatisfied() {
        let ctx = VersionedContext::new("ffmpeg version 5.0.2 Copyright");
        let deps = vec![ffmpeg_dep()]; // requires >=6.0
        let statuses = check_dependencies(&deps, &ctx);
        assert_eq!(statuses.len(), 1);
        assert!(statuses[0].found);
        assert_eq!(statuses[0].installed_version.as_deref(), Some("5.0.2"));
        assert_eq!(statuses[0].version_satisfied, Some(false));
    }

    #[test]
    fn test_check_deps_no_version_constraint() {
        let ctx = VersionedContext::new("yt-dlp 2024.12.23");
        let deps = vec![ytdlp_dep()]; // no version constraint
        let statuses = check_dependencies(&deps, &ctx);
        assert_eq!(statuses.len(), 1);
        assert!(statuses[0].found);
        assert!(statuses[0].installed_version.is_none());
        assert!(statuses[0].version_satisfied.is_none());
    }

    #[test]
    fn test_preflight_version_mismatch_returns_error() {
        let ctx = VersionedContext::new("ffmpeg version 5.0.2 Copyright");
        let mut registry = NodeRegistry::new();
        registry.register("video-transcode", Box::new(FfmpegProcessor));
        let def = make_definition(&["input", "video-transcode", "output"]);
        let result = check_pipeline_dependencies(&def, &registry, &ctx);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("ffmpeg"));
        assert!(err_msg.contains("5.0.2"));
        assert!(err_msg.contains(">=6.0"));
    }

    #[test]
    fn test_preflight_version_satisfied_passes() {
        let ctx = VersionedContext::new("ffmpeg version 6.1.1 Copyright");
        let mut registry = NodeRegistry::new();
        registry.register("video-transcode", Box::new(FfmpegProcessor));
        let def = make_definition(&["input", "video-transcode", "output"]);
        let result = check_pipeline_dependencies(&def, &registry, &ctx);
        assert!(result.is_ok());
    }

    // --- check_pipeline_secrets ---

    fn make_definition_with_secrets(secrets: Vec<bnto_core::SecretDef>) -> PipelineDefinition {
        let mut def = make_definition(&["input", "output"]);
        def.secrets = secrets;
        def
    }

    fn required_secret(key: &str) -> bnto_core::SecretDef {
        bnto_core::SecretDef {
            key: key.to_string(),
            description: String::new(),
            required: true,
        }
    }

    fn optional_secret(key: &str) -> bnto_core::SecretDef {
        bnto_core::SecretDef {
            key: key.to_string(),
            description: String::new(),
            required: false,
        }
    }

    /// Mock context that provides specific env vars.
    struct EnvContext {
        vars: std::collections::HashMap<String, String>,
    }

    impl EnvContext {
        fn new(pairs: &[(&str, &str)]) -> Self {
            Self {
                vars: pairs
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            }
        }
    }

    impl ProcessContext for EnvContext {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            Err(BntoError::ProcessingFailed("mock".to_string()))
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, BntoError> {
            Err(BntoError::ProcessingFailed("mock".to_string()))
        }
        fn env_var(&self, key: &str) -> Option<String> {
            self.vars.get(key).cloned()
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Err(BntoError::ProcessingFailed("mock".to_string()))
        }
    }

    #[test]
    fn test_secrets_preflight_no_secrets_ok() {
        let def = make_definition(&["input", "output"]);
        let result = check_pipeline_secrets(&def, &NoopContext);
        assert!(result.is_ok());
    }

    #[test]
    fn test_secrets_preflight_all_present_ok() {
        let def = make_definition_with_secrets(vec![required_secret("API_KEY")]);
        let ctx = EnvContext::new(&[("API_KEY", "sk-123")]);
        let result = check_pipeline_secrets(&def, &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_secrets_preflight_required_missing_fails() {
        let def = make_definition_with_secrets(vec![required_secret("API_KEY")]);
        let result = check_pipeline_secrets(&def, &NoopContext);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("API_KEY"));
        assert!(msg.contains("~/.config/bnto/.env"));
    }

    #[test]
    fn test_secrets_preflight_optional_missing_ok() {
        let def = make_definition_with_secrets(vec![optional_secret("OPTIONAL_KEY")]);
        let result = check_pipeline_secrets(&def, &NoopContext);
        assert!(result.is_ok());
    }

    #[test]
    fn test_secrets_preflight_mixed_missing_only_required() {
        let def = make_definition_with_secrets(vec![
            required_secret("REQUIRED_KEY"),
            optional_secret("OPTIONAL_KEY"),
        ]);
        let result = check_pipeline_secrets(&def, &NoopContext);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("REQUIRED_KEY"));
        assert!(!msg.contains("OPTIONAL_KEY"));
    }

    // --- probe command wiring ---

    /// Mock context that records the command used to probe for binaries.
    struct RecordingContext {
        probed_with: std::sync::Mutex<Vec<String>>,
    }

    impl ProcessContext for RecordingContext {
        fn run_command(&self, cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            self.probed_with.lock().unwrap().push(cmd.to_string());
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

    /// Guards against regressing to a hardcoded `which`, which does not
    /// exist on Windows and made every dependency look missing there.
    #[test]
    fn test_check_deps_probes_with_platform_command() {
        let ctx = RecordingContext {
            probed_with: std::sync::Mutex::new(Vec::new()),
        };
        check_dependencies(&[ytdlp_dep()], &ctx);

        let probed = ctx.probed_with.lock().unwrap();
        assert_eq!(probed.len(), 1);
        assert_eq!(probed[0], bnto_core::probe_command());
        assert_ne!(probed[0], "", "probe command must not be empty");
    }
}
