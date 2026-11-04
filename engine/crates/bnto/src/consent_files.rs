// The files a recipe's shell steps run from the folder it runs in: a
// relative `searchPath` folder its command is found in, the `cwd` it starts
// in (a tool reads its config there), and a relative path among the words
// of its args, `env` values and script (an option's `--opt=path` too), or
// a bare name of a file in the run folder when that's known. Approval pins the recipe's bytes, not these files, so the prompt
// names them and says a change to them won't ask again (settled #4 of
// decisions/project-recipes.md).

use std::path::{Path, PathBuf};

use bnto_core::{PipelineDefinition, PipelineNode};
use serde_json::{Map, Value};

use crate::path_flag::is_relative;

/// A file or folder from the working folder that a shell step runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunFile {
    /// The path as the recipe wrote it.
    pub path: String,
    /// Which part of the step names it: `searchPath`, `cwd`, `args`, `env`,
    /// `script`.
    pub source: &'static str,
}

/// Every such file in `def`, in recipe order, each named once. `run_dir`
/// is the folder the recipe runs in, when the prompt knows it (a project's
/// root): a bare name (`setup.py`) then counts when that file is there, or
/// in the step's `cwd`.
pub fn files_it_runs(def: &PipelineDefinition, run_dir: Option<&Path>) -> Vec<RunFile> {
    let mut found = Vec::new();
    collect(&def.nodes, run_dir, &mut found);
    found
}

fn collect(nodes: &[PipelineNode], run_dir: Option<&Path>, found: &mut Vec<RunFile>) {
    for node in nodes {
        if node.node_type == bnto_shell::NODE_TYPE {
            for file in step_files(&node.params, run_dir) {
                if !found.iter().any(|f| f.path == file.path) {
                    found.push(file);
                }
            }
        }
        if let Some(children) = &node.children {
            collect(children, run_dir, found);
        }
    }
}

fn step_files(params: &Map<String, Value>, run_dir: Option<&Path>) -> Vec<RunFile> {
    let cwd = cwd(params);
    let dirs = run_dirs(run_dir, cwd.as_deref());
    let is_file = |word: &String| names_a_file(word) || in_run_dir(word, &dirs);
    let script = params.get("script").and_then(Value::as_str).unwrap_or("");
    let sources = [
        ("args", arg_words(params)),
        ("env", env_values(params)),
        ("script", script_words(script)),
    ];
    let mut found: Vec<(String, &'static str)> = Vec::new();
    found.extend(folders(params).map(|p| (p, "searchPath")));
    found.extend(cwd.map(|c| (c, "cwd")));
    for (source, texts) in sources {
        found.extend(words(texts).filter(is_file).map(|w| (w, source)));
    }
    let file = |(path, source)| RunFile { path, source };
    found.into_iter().map(file).collect()
}

/// The folders a bare name is looked for in: the run folder, and the
/// step's `cwd` under it.
fn run_dirs(run_dir: Option<&Path>, cwd: Option<&str>) -> Vec<PathBuf> {
    let Some(dir) = run_dir else {
        return Vec::new();
    };
    let mut dirs = vec![dir.to_path_buf()];
    dirs.extend(cwd.map(|c| dir.join(c)));
    dirs
}

/// The folder the step starts in, when it names one.
fn cwd(params: &Map<String, Value>) -> Option<String> {
    let cwd = params.get("cwd").and_then(Value::as_str).map(str::trim)?;
    (!cwd.is_empty()).then(|| cwd.to_string())
}

/// The step's text args, groups included.
fn arg_words(params: &Map<String, Value>) -> Vec<String> {
    let mut args = Vec::new();
    literal_args(params.get("args"), &mut args);
    args
}

/// Every word of `texts` that could name a file: each whitespace-separated
/// word, and the value after `=` in `--opt=value`.
fn words(texts: impl IntoIterator<Item = String>) -> impl Iterator<Item = String> {
    let words: Vec<String> = texts
        .into_iter()
        .flat_map(|text| {
            let words: Vec<String> = text.split_whitespace().map(str::to_string).collect();
            words
        })
        .flat_map(|word| match word.split_once('=') {
            Some((key, value)) if key.starts_with('-') => vec![value.to_string()],
            _ => vec![word],
        })
        .collect();
    words.into_iter()
}

/// The relative `searchPath` folders the command is found in.
fn folders(params: &Map<String, Value>) -> impl Iterator<Item = String> {
    bnto_shell::search_path::read_search_path(params.get("searchPath"))
        .unwrap_or_default()
        .into_iter()
        .filter(|f| is_relative(f))
}

/// The step's `env` values that hold no placeholder.
fn env_values(params: &Map<String, Value>) -> Vec<String> {
    let env = params
        .get("env")
        .and_then(Value::as_object)
        .into_iter()
        .flatten();
    env.filter_map(|(_, v)| v.as_str())
        .filter(|v| !v.contains("{{"))
        .map(str::to_string)
        .collect()
}

/// Every text arg, groups included (a template is filled with data, so a
/// value holding one isn't the recipe's own file).
fn literal_args(value: Option<&Value>, out: &mut Vec<String>) {
    match value {
        Some(Value::String(s)) if !s.contains("{{") => out.push(s.clone()),
        Some(Value::Array(items)) => items.iter().for_each(|v| literal_args(Some(v), out)),
        Some(Value::Object(map)) => literal_args(map.get("each"), out),
        _ => {}
    }
}

/// A word that reads as a relative path: not an option or a URL, and
/// holding a separator (`tools/x.py`, `./x.sh`).
fn names_a_file(word: &str) -> bool {
    plain_word(word) && word.contains(['/', '\\'])
}

/// A bare name (`setup.py`) of a file in one of `dirs`.
fn in_run_dir(word: &str, dirs: &[PathBuf]) -> bool {
    plain_word(word) && !word.contains(['/', '\\']) && dirs.iter().any(|d| d.join(word).is_file())
}

/// Not empty, an option, a URL or an absolute path.
fn plain_word(word: &str) -> bool {
    !word.is_empty() && !word.starts_with('-') && !word.contains("://") && is_relative(word)
}

/// A script's words, quotes trimmed, split at shell punctuation too.
fn script_words(script: &str) -> Vec<String> {
    script
        .split(|c: char| c.is_whitespace() || matches!(c, ';' | '&' | '|' | '(' | ')'))
        .map(|word| word.trim_matches(['"', '\'']).to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn def(params: Value) -> PipelineDefinition {
        serde_json::from_value(
            json!({ "nodes": [{ "id": "s", "type": "shell-run", "parameters": params }] }),
        )
        .unwrap()
    }

    fn paths(def: &PipelineDefinition) -> Vec<(String, &'static str)> {
        paths_in(def, None)
    }

    fn paths_in(
        def: &PipelineDefinition,
        run_dir: Option<&std::path::Path>,
    ) -> Vec<(String, &'static str)> {
        files_it_runs(def, run_dir)
            .into_iter()
            .map(|f| (f.path, f.source))
            .collect()
    }

    #[test]
    fn a_relative_search_path_is_a_file_it_runs() {
        let d = def(json!({ "command": "lint", "searchPath": ["bin", "/usr/local/bin"] }));
        assert_eq!(paths(&d), [("bin".to_string(), "searchPath")]);
    }

    #[test]
    fn a_cwd_is_a_folder_it_runs_in() {
        let d = def(json!({ "command": "cargo", "args": ["build"], "cwd": "engine" }));
        assert_eq!(paths(&d), [("engine".to_string(), "cwd")]);
    }

    #[test]
    fn a_relative_path_in_args_is_a_file_it_runs() {
        let d = def(json!({ "command": "python3",
            "args": ["tools/x.py", "--flag", "-o", "out/y", "https://e.com/a", "{{path}}"] }));
        assert_eq!(
            paths(&d),
            [
                ("tools/x.py".to_string(), "args"),
                ("out/y".to_string(), "args")
            ]
        );
    }

    #[test]
    fn a_script_running_a_relative_path_names_it() {
        let d = def(json!({ "shell": "sh",
            "script": "set -eu\n./tools/build.sh \"$1\" && sh '../x/run'\necho /dev/null" }));
        assert_eq!(
            paths(&d),
            [
                ("./tools/build.sh".to_string(), "script"),
                ("../x/run".to_string(), "script")
            ]
        );
    }

    #[test]
    fn a_relative_path_in_an_env_value_is_a_file_it_runs() {
        let d = def(json!({ "command": "tool",
            "env": { "CONFIG": "conf/tool.toml", "MODE": "fast", "URL": "https://e.com/x" } }));
        assert_eq!(paths(&d), [("conf/tool.toml".to_string(), "env")]);
    }

    #[test]
    fn a_script_running_a_file_by_its_path_names_it() {
        let d = def(json!({ "shell": "sh",
            "script": "python3 tools/x.py \"$1\"\nbash ci/deploy.sh" }));
        assert_eq!(
            paths(&d),
            [
                ("tools/x.py".to_string(), "script"),
                ("ci/deploy.sh".to_string(), "script")
            ]
        );
    }

    #[test]
    fn an_options_value_naming_a_file_counts() {
        let d = def(json!({ "command": "tool", "args": ["--config=conf/x.toml", "--n=3"] }));
        assert_eq!(paths(&d), [("conf/x.toml".to_string(), "args")]);
    }

    #[test]
    fn a_path_inside_an_env_value_counts() {
        let d = def(json!({ "command": "tool", "env": { "OPTS": "-c conf/x.toml -v" } }));
        assert_eq!(paths(&d), [("conf/x.toml".to_string(), "env")]);
    }

    #[test]
    fn a_bare_name_counts_when_that_file_is_in_the_run_folder() {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(root.path().join("setup.py"), "").unwrap();
        std::fs::create_dir(root.path().join("tools")).unwrap();
        std::fs::write(root.path().join("tools/deploy.sh"), "").unwrap();
        let d = def(json!({ "command": "python3", "args": ["setup.py", "build", "--x"] }));
        assert_eq!(
            paths_in(&d, Some(root.path())),
            [("setup.py".to_string(), "args")]
        );
        assert!(
            paths_in(&d, None).is_empty(),
            "no run folder, nothing to look in"
        );
        let s = def(json!({ "shell": "sh", "cwd": "tools", "script": "bash deploy.sh" }));
        assert_eq!(
            paths_in(&s, Some(root.path())),
            [
                ("tools".to_string(), "cwd"),
                ("deploy.sh".to_string(), "script")
            ]
        );
    }

    #[test]
    fn a_step_on_path_names_no_files() {
        let d = def(json!({ "command": "git", "args": ["--version"] }));
        assert!(files_it_runs(&d, None).is_empty());
    }

    #[test]
    fn non_shell_steps_name_no_files() {
        let d: PipelineDefinition = serde_json::from_value(json!({ "nodes": [
            { "id": "l", "type": "file-list", "parameters": { "base": "bin" } }
        ] }))
        .unwrap();
        assert!(files_it_runs(&d, None).is_empty());
    }
}
