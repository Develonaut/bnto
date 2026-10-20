// shell-command's `each` arg groups through the built binary: files a
// collect step found reach one command, each as its own argument.

mod helpers;

use std::path::Path;
use std::process::{Command, Output};

use helpers::{bnto_bin, temp_output_dir};

/// A project with DLLs (and a stray text file) under two folder trees.
fn project() -> tempfile::TempDir {
    let work = temp_output_dir();
    for file in [
        "bin1/a.dll",
        "bin1/nested/b.dll",
        "bin2/c.dll",
        "bin2/notes.txt",
    ] {
        let path = work.path().join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, b"").unwrap();
    }
    work
}

/// List both trees, collect their DLLs, and hand them all to one
/// `git rev-parse --sq-quote`, which prints its args back quoted.
fn run_each(work: &Path, out: &Path, group: serde_json::Value) -> Output {
    let recipe = work.join("each.bnto.json");
    let body = serde_json::json!({
        "nodes": [
            { "id": "input", "type": "input", "parameters": { "mode": "none" } },
            { "id": "list", "type": "file-list", "parameters": { "paths": ["bin1", "bin2"] } },
            { "id": "collect", "type": "file-collect",
              "parameters": { "pattern": "*.dll", "flatten": false } },
            { "id": "tool", "type": "shell-command", "parameters": {
                "command": "git", "args": ["rev-parse", "--sq-quote", group, "--end"]
            } },
            { "id": "output", "type": "output", "parameters": { "mode": "write" } }
        ],
        "edges": [
            { "id": "e1", "source": "list", "target": "collect" },
            { "id": "e2", "source": "collect", "target": "tool" },
            { "id": "e3", "source": "tool", "target": "output" }
        ]
    });
    std::fs::write(&recipe, body.to_string()).unwrap();
    Command::new(bnto_bin())
        .args(["run", recipe.to_str().unwrap(), "--yes"])
        .args(["-o", out.to_str().unwrap()])
        .current_dir(work)
        .output()
        .unwrap()
}

/// The text of the one file the run wrote to `out`.
fn only_output(out: &Path) -> String {
    let files: Vec<_> = std::fs::read_dir(out)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    assert_eq!(files.len(), 1, "files: {files:?}");
    std::fs::read_to_string(&files[0]).unwrap()
}

#[test]
fn test_every_collected_file_reaches_one_command_as_its_own_arg() {
    let (work, out) = (project(), temp_output_dir());
    let group = serde_json::json!({ "each": ["-object={{path}}"] });
    let output = run_each(work.path(), out.path(), group);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    let quoted = only_output(out.path());
    // bnto lists files under the work dir as the OS reports it: `getcwd`
    // resolves macOS's /var -> /private/var symlink. Windows stays as-is,
    // since canonicalize there would add a \\?\ prefix.
    let root = if cfg!(unix) {
        work.path().canonicalize().unwrap()
    } else {
        work.path().to_path_buf()
    };
    for file in ["bin1/a.dll", "bin1/nested/b.dll", "bin2/c.dll"] {
        let path = file.split('/').fold(root.clone(), |p, s| p.join(s));
        assert!(
            quoted.contains(&format!("'-object={}'", path.display())),
            "{file} missing: {quoted}"
        );
    }
    assert_eq!(quoted.matches("-object=").count(), 3, "{quoted}");
    assert!(!quoted.contains("notes.txt"), "{quoted}");
    assert!(quoted.trim_end().ends_with("'--end'"), "{quoted}");
}

#[test]
fn test_a_group_reads_each_files_pipeline_name() {
    let (work, out) = (project(), temp_output_dir());
    let group = serde_json::json!({ "each": ["--name", "{{file}}"] });
    let output = run_each(work.path(), out.path(), group);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    let quoted = only_output(out.path());
    for name in ["a.dll", "nested/b.dll", "c.dll"] {
        assert!(quoted.contains(&format!("'--name' '{name}'")), "{quoted}");
    }
}
