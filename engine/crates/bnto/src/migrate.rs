// Recipe migration — applies sequential transforms to `.bnto.json` files.
//
// `bnto migrate <path>` reads a recipe file, detects which migrations apply
// based on node types and parameter names, and writes the updated file.
// Operates on raw `serde_json::Value` so it can handle old formats that
// may not deserialize into current structs.

use std::fs;
use std::path::{Path, PathBuf};

use colored::Colorize;
use serde_json::Value;

/// A single migration transform applied to a recipe JSON document.
struct Migration {
    /// Human-readable description shown in CLI output.
    description: &'static str,
    /// Returns true if this migration should be applied to the given JSON.
    applies: fn(&Value) -> bool,
    /// Mutate the JSON value in place.
    apply: fn(&mut Value),
}

/// All registered migrations in chronological order.
/// Each migration is idempotent — `applies()` returns false if already migrated.
fn all_migrations() -> Vec<Migration> {
    vec![
        Migration {
            description: "Rename 'compression' parameter to 'quality'",
            applies: has_compression_param,
            apply: rename_compression_to_quality,
        },
        Migration {
            description: "Convert 'file-sanitize' node type to 'file-rename' with sanitize params",
            applies: has_file_sanitize_node,
            apply: convert_file_sanitize_to_file_rename,
        },
    ]
}

/// Apply all applicable migrations to a recipe JSON value.
/// Returns descriptions of migrations that were applied.
pub fn apply_migrations(value: &mut Value) -> Vec<&'static str> {
    let migrations = all_migrations();
    let mut applied = Vec::new();

    for migration in &migrations {
        if (migration.applies)(value) {
            (migration.apply)(value);
            applied.push(migration.description);
        }
    }

    applied
}

// --- Migration: compression → quality ---

/// Check if any node in the tree has a `compression` parameter.
fn has_compression_param(value: &Value) -> bool {
    visit_nodes(value, &|node| {
        node.get("parameters")
            .and_then(Value::as_object)
            .is_some_and(|p| p.contains_key("compression"))
    })
}

/// Rename `compression` → `quality` on all nodes in the tree.
fn rename_compression_to_quality(value: &mut Value) {
    mutate_nodes(value, &|node| {
        if let Some(params) = node.get_mut("parameters").and_then(Value::as_object_mut)
            && let Some(val) = params.remove("compression")
        {
            params.insert("quality".to_string(), val);
        }
    });
}

// --- Migration: file-sanitize → file-rename ---

/// Check if any node has `type: "file-sanitize"`.
fn has_file_sanitize_node(value: &Value) -> bool {
    visit_nodes(value, &|node| {
        node.get("type")
            .and_then(Value::as_str)
            .is_some_and(|t| t == "file-sanitize")
    })
}

/// Convert `file-sanitize` nodes to `file-rename` with sanitize params preserved.
fn convert_file_sanitize_to_file_rename(value: &mut Value) {
    mutate_nodes(value, &|node| {
        let is_file_sanitize = node
            .get("type")
            .and_then(Value::as_str)
            .is_some_and(|t| t == "file-sanitize");

        if is_file_sanitize {
            node["type"] = Value::String("file-rename".to_string());
        }
    });
}

// --- Tree traversal helpers ---

/// Visit all nodes in the tree, returning true if the predicate matches any.
fn visit_nodes(value: &Value, predicate: &dyn Fn(&Value) -> bool) -> bool {
    if predicate(value) {
        return true;
    }

    if let Some(nodes) = value.get("nodes").and_then(Value::as_array) {
        for node in nodes {
            if visit_nodes(node, predicate) {
                return true;
            }
        }
    }

    // Also check `children` (PipelineDefinition format).
    if let Some(children) = value.get("children").and_then(Value::as_array) {
        for child in children {
            if visit_nodes(child, predicate) {
                return true;
            }
        }
    }

    false
}

/// Mutate all nodes in the tree with the given function.
fn mutate_nodes(value: &mut Value, mutator: &dyn Fn(&mut Value)) {
    mutator(value);

    if let Some(nodes) = value.get_mut("nodes").and_then(Value::as_array_mut) {
        for node in nodes {
            mutate_nodes(node, mutator);
        }
    }

    if let Some(children) = value.get_mut("children").and_then(Value::as_array_mut) {
        for child in children {
            mutate_nodes(child, mutator);
        }
    }
}

// --- File operations ---

/// Result of migrating a single file.
pub struct MigrateFileResult {
    pub applied: Vec<&'static str>,
}

/// Migrate a single `.bnto.json` file. Returns the list of applied migrations.
/// If `dry_run` is true, reports changes without writing.
pub fn migrate_file(path: &Path, dry_run: bool) -> Result<MigrateFileResult, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Cannot read '{}': {e}", path.display()))?;

    let mut value: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Invalid JSON in '{}': {e}", path.display()))?;

    let applied = apply_migrations(&mut value);

    if !applied.is_empty() && !dry_run {
        // Back up the original before overwriting.
        let mut backup_name = path.as_os_str().to_os_string();
        backup_name.push(".bak");
        let backup = PathBuf::from(backup_name);
        fs::copy(path, &backup)
            .map_err(|e| format!("Cannot create backup '{}': {e}", backup.display()))?;

        let output = serde_json::to_string_pretty(&value)
            .map_err(|e| format!("Cannot serialize '{}': {e}", path.display()))?;

        fs::write(path, format!("{output}\n"))
            .map_err(|e| format!("Cannot write '{}': {e}", path.display()))?;
    }

    Ok(MigrateFileResult { applied })
}

/// Collect all `.bnto.json` files in a directory (non-recursive for safety).
pub fn find_recipe_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let entries =
        fs::read_dir(dir).map_err(|e| format!("Cannot read directory '{}': {e}", dir.display()))?;

    let mut files: Vec<PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e == "json")
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.ends_with(".bnto.json"))
        })
        .collect();

    files.sort();
    Ok(files)
}

// --- CLI entry point ---

/// Run the migrate command.
pub fn run_migrate(path: &str, dry_run: bool) {
    let target = Path::new(path);

    let files = if target.is_dir() {
        match find_recipe_files(target) {
            Ok(files) => files,
            Err(e) => {
                eprintln!("{} {e}", "Error:".red());
                std::process::exit(1);
            }
        }
    } else {
        vec![target.to_path_buf()]
    };

    if files.is_empty() {
        eprintln!("{}", "No .bnto.json files found.".dimmed());
        return;
    }

    if dry_run {
        eprintln!("{}", "Dry run — no files will be modified.\n".dimmed());
    }

    let mut total_migrated = 0;
    let mut total_skipped = 0;

    for file in &files {
        match migrate_file(file, dry_run) {
            Ok(result) => {
                if result.applied.is_empty() {
                    total_skipped += 1;
                    eprintln!("  {} {}", "✓".dimmed(), file.display().to_string().dimmed());
                } else {
                    total_migrated += 1;
                    eprintln!("  {} {}", "↑".green(), file.display());
                    for desc in &result.applied {
                        eprintln!("    {} {desc}", "→".dimmed());
                    }
                }
            }
            Err(e) => {
                eprintln!("  {} {}", "✗".red(), e);
            }
        }
    }

    eprintln!();
    if total_migrated > 0 {
        let verb = if dry_run { "would migrate" } else { "migrated" };
        eprintln!(
            "{} {total_migrated} file{} {verb}, {total_skipped} already up to date.",
            "Done.".green().bold(),
            if total_migrated == 1 { "" } else { "s" },
        );
        if !dry_run {
            eprintln!(
                "{}",
                "Backups saved as .bnto.json.bak alongside originals.".dimmed()
            );
        }
    } else {
        eprintln!(
            "{} All {} file{} already up to date.",
            "Done.".green().bold(),
            total_skipped,
            if total_skipped == 1 { "" } else { "s" },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- apply_migrations ---

    #[test]
    fn detects_compression_param_needs_migration() {
        let value = json!({
            "id": "test",
            "type": "group",
            "nodes": [{
                "id": "compress",
                "type": "image-compress",
                "parameters": { "compression": 80 }
            }]
        });
        assert!(has_compression_param(&value));
    }

    #[test]
    fn renames_compression_to_quality() {
        let mut value = json!({
            "id": "test",
            "type": "group",
            "parameters": {},
            "nodes": [{
                "id": "compress",
                "type": "image-compress",
                "parameters": { "compression": 80 }
            }]
        });

        let applied = apply_migrations(&mut value);
        assert_eq!(applied.len(), 1);
        assert!(applied[0].contains("compression"));

        let params = &value["nodes"][0]["parameters"];
        assert!(params.get("compression").is_none());
        assert_eq!(params["quality"], json!(80));
    }

    #[test]
    fn migrates_file_sanitize_to_file_rename() {
        let mut value = json!({
            "id": "test",
            "type": "group",
            "parameters": {},
            "nodes": [{
                "id": "sanitize",
                "type": "file-sanitize",
                "parameters": { "mode": "slugify", "separator": "-" }
            }]
        });

        let applied = apply_migrations(&mut value);
        assert_eq!(applied.len(), 1);
        assert!(applied[0].contains("file-sanitize"));

        let node = &value["nodes"][0];
        assert_eq!(node["type"], json!("file-rename"));
        // Sanitize params preserved on the node.
        assert_eq!(node["parameters"]["mode"], json!("slugify"));
        assert_eq!(node["parameters"]["separator"], json!("-"));
    }

    #[test]
    fn already_migrated_is_noop() {
        let mut value = json!({
            "id": "test",
            "type": "group",
            "parameters": {},
            "nodes": [{
                "id": "compress",
                "type": "image-compress",
                "parameters": { "quality": 80 }
            }]
        });

        let applied = apply_migrations(&mut value);
        assert!(applied.is_empty(), "No migrations should apply");
    }

    #[test]
    fn idempotent_double_run() {
        let mut value = json!({
            "id": "test",
            "type": "group",
            "parameters": {},
            "nodes": [{
                "id": "compress",
                "type": "image-compress",
                "parameters": { "compression": 80 }
            }]
        });

        apply_migrations(&mut value);
        let snapshot = value.clone();

        let applied = apply_migrations(&mut value);
        assert!(applied.is_empty(), "Second run should find nothing to do");
        assert_eq!(value, snapshot, "JSON unchanged after idempotent re-run");
    }

    #[test]
    fn unknown_recipe_format_is_left_alone() {
        let mut value = json!({
            "something": "unexpected",
            "nodes": [{
                "id": "x",
                "type": "custom-node",
                "parameters": { "foo": "bar" }
            }]
        });

        let original = value.clone();
        let applied = apply_migrations(&mut value);
        assert!(applied.is_empty());
        assert_eq!(value, original);
    }

    #[test]
    fn multiple_migrations_applied_sequentially() {
        let mut value = json!({
            "id": "test",
            "type": "group",
            "parameters": {},
            "nodes": [
                {
                    "id": "compress",
                    "type": "image-compress",
                    "parameters": { "compression": 80 }
                },
                {
                    "id": "sanitize",
                    "type": "file-sanitize",
                    "parameters": { "mode": "slugify" }
                }
            ]
        });

        let applied = apply_migrations(&mut value);
        assert_eq!(applied.len(), 2, "Both migrations should apply");

        // compression → quality
        assert_eq!(value["nodes"][0]["parameters"]["quality"], json!(80));
        assert!(value["nodes"][0]["parameters"].get("compression").is_none());

        // file-sanitize → file-rename
        assert_eq!(value["nodes"][1]["type"], json!("file-rename"));
    }

    #[test]
    fn handles_deeply_nested_nodes() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "parameters": {},
            "nodes": [{
                "id": "loop",
                "type": "loop",
                "parameters": {},
                "nodes": [{
                    "id": "compress",
                    "type": "image-compress",
                    "parameters": { "compression": 60 }
                }]
            }]
        });

        let applied = apply_migrations(&mut value);
        assert_eq!(applied.len(), 1);
        assert_eq!(
            value["nodes"][0]["nodes"][0]["parameters"]["quality"],
            json!(60)
        );
    }

    #[test]
    fn handles_pipeline_definition_with_children() {
        let mut value = json!({
            "id": "root",
            "type": "group",
            "params": {},
            "children": [{
                "id": "compress",
                "type": "image-compress",
                "params": {},
                "parameters": { "compression": 50 }
            }]
        });

        let applied = apply_migrations(&mut value);
        assert_eq!(applied.len(), 1);
        assert_eq!(value["children"][0]["parameters"]["quality"], json!(50));
    }

    // --- File operations ---

    #[test]
    fn migrate_file_with_dry_run_does_not_write() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("test.bnto.json");
        let content = serde_json::to_string_pretty(&json!({
            "id": "test",
            "type": "group",
            "parameters": {},
            "nodes": [{
                "id": "c",
                "type": "image-compress",
                "parameters": { "compression": 80 }
            }]
        }))
        .unwrap();
        fs::write(&file, &content).unwrap();

        let result = migrate_file(&file, true).unwrap();
        assert_eq!(result.applied.len(), 1);

        // File should be unchanged after dry run.
        let after = fs::read_to_string(&file).unwrap();
        assert_eq!(after, content, "Dry run must not modify the file");

        // No backup should exist.
        let backup = file.parent().unwrap().join("test.bnto.json.bak");
        assert!(!backup.exists());
    }

    #[test]
    fn migrate_file_writes_and_creates_backup() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("recipe.bnto.json");
        let original = json!({
            "id": "test",
            "type": "group",
            "parameters": {},
            "nodes": [{
                "id": "c",
                "type": "image-compress",
                "parameters": { "compression": 75 }
            }]
        });
        fs::write(&file, serde_json::to_string_pretty(&original).unwrap()).unwrap();

        let result = migrate_file(&file, false).unwrap();
        assert_eq!(result.applied.len(), 1);

        // Backup should exist with original content.
        let backup = dir.path().join("recipe.bnto.json.bak");
        assert!(backup.exists(), "Backup file should be created");

        // Migrated file should have quality, not compression.
        let migrated: Value = serde_json::from_str(&fs::read_to_string(&file).unwrap()).unwrap();
        assert_eq!(migrated["nodes"][0]["parameters"]["quality"], json!(75));
        assert!(
            migrated["nodes"][0]["parameters"]
                .get("compression")
                .is_none()
        );
    }

    #[test]
    fn find_recipe_files_filters_correctly() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("recipe.bnto.json"), "{}").unwrap();
        fs::write(dir.path().join("other.json"), "{}").unwrap();
        fs::write(dir.path().join("readme.md"), "hello").unwrap();

        let files = find_recipe_files(dir.path()).unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("recipe.bnto.json"));
    }

    #[test]
    fn migrate_file_noop_does_not_create_backup() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("current.bnto.json");
        let content = json!({
            "id": "test",
            "type": "group",
            "parameters": {},
            "nodes": [{
                "id": "c",
                "type": "image-compress",
                "parameters": { "quality": 80 }
            }]
        });
        fs::write(&file, serde_json::to_string_pretty(&content).unwrap()).unwrap();

        let result = migrate_file(&file, false).unwrap();
        assert!(result.applied.is_empty());

        // No backup when nothing changed.
        let backup = dir.path().join("current.bnto.json.bak");
        assert!(!backup.exists());
    }
}
