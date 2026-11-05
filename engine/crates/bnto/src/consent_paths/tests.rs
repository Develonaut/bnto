use super::*;
use serde_json::json;

fn def(nodes: Value) -> PipelineDefinition {
    serde_json::from_value(json!({ "nodes": nodes })).unwrap()
}

fn shown(params: &[PathParam]) -> Vec<(String, &str, String, Option<PathFlag>)> {
    params
        .iter()
        .map(|p| (p.node_type.clone(), p.param, p.value.clone(), p.flag))
        .collect()
}

fn row(
    t: &str,
    param: &'static str,
    v: &str,
    flag: Option<PathFlag>,
) -> (String, &'static str, String, Option<PathFlag>) {
    (t.to_string(), param, v.to_string(), flag)
}

#[test]
fn an_absolute_file_delete_base_is_flagged() {
    let d =
        def(json!([{ "id": "d", "type": "file-delete", "parameters": { "base": "/Users/me" } }]));
    assert_eq!(
        shown(&path_params(&d, None)),
        [row(
            "file-delete",
            "base",
            "/Users/me",
            Some(PathFlag::Absolute)
        )]
    );
}

#[test]
fn every_path_param_is_listed_in_recipe_order() {
    let d = def(json!([
        { "id": "l", "type": "file-list", "parameters": { "base": "assets", "paths": ["a.txt", "../b"] } },
        { "id": "g", "type": "group", "nodes": [
            { "id": "m", "type": "file-move", "parameters": { "destination": "~/Archive" } },
            { "id": "c", "type": "file-copy", "parameters": { "destination": "backup" } }
        ] },
        { "id": "s", "type": "shell-run", "parameters": {
            "command": "tool", "searchPath": ["bin", "/opt/x"], "cwd": "tools" } },
        { "id": "o", "type": "output", "parameters": { "mode": "write", "directory": "{{env.OUT}}" } }
    ]));
    assert_eq!(
        shown(&path_params(&d, None)),
        [
            row("file-list", "base", "assets", None),
            row("file-list", "paths", "a.txt", None),
            row("file-list", "paths", "../b", Some(PathFlag::ParentDir)),
            row(
                "file-move",
                "destination",
                "~/Archive",
                Some(PathFlag::Home)
            ),
            row("file-copy", "destination", "backup", None),
            row("shell-run", "searchPath", "bin", None),
            row(
                "shell-run",
                "searchPath",
                "/opt/x",
                Some(PathFlag::Absolute)
            ),
            row("shell-run", "cwd", "tools", None),
            row(
                "output",
                "directory",
                "{{env.OUT}}",
                Some(PathFlag::SetWhenRun)
            ),
        ]
    );
}

#[test]
fn overwrite_mode_is_flagged_for_what_it_does() {
    let d = def(json!([{ "id": "o", "type": "output", "parameters": { "mode": "overwrite" } }]));
    assert_eq!(
        shown(&path_params(&d, None)),
        [row(
            "output",
            "mode",
            "overwrite",
            Some(PathFlag::Overwrite)
        )]
    );
}

#[test]
fn steps_without_paths_list_nothing() {
    let d = def(json!([
        { "id": "i", "type": "input" },
        { "id": "s", "type": "file-sort", "parameters": { "by": "name" } },
        { "id": "o", "type": "output", "parameters": { "mode": "write", "directory": "" } }
    ]));
    assert!(path_params(&d, None).is_empty());
}

#[cfg(unix)]
#[test]
fn a_project_folder_linked_outside_is_flagged() {
    let root = tempfile::tempdir().unwrap();
    let elsewhere = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(elsewhere.path(), root.path().join("dist")).unwrap();
    let d = def(json!([{ "id": "d", "type": "file-delete", "parameters": { "base": "dist" } }]));
    assert_eq!(
        path_params(&d, Some(root.path()))[0].flag,
        Some(PathFlag::LeavesProject)
    );
    assert_eq!(path_params(&d, None)[0].flag, None);
}

#[test]
fn spreadsheet_export_writes_a_path_and_may_replace_it() {
    let d = def(json!([{ "id": "e", "type": "spreadsheet-export",
        "parameters": { "path": "~/.zshrc", "overwrite": true } }]));
    assert_eq!(
        shown(&path_params(&d, None)),
        [
            row(
                "spreadsheet-export",
                "path",
                "~/.zshrc",
                Some(PathFlag::Home)
            ),
            row(
                "spreadsheet-export",
                "overwrite",
                "true",
                Some(PathFlag::ReplacesExisting)
            ),
        ]
    );
}

#[test]
fn readers_are_listed_too() {
    let d = def(
        json!([{ "id": "l", "type": "spreadsheet-lookup", "parameters": { "table": "/etc/x.csv" } }]),
    );
    assert_eq!(
        shown(&path_params(&d, None)),
        [row(
            "spreadsheet-lookup",
            "table",
            "/etc/x.csv",
            Some(PathFlag::Absolute)
        )]
    );
}

#[test]
fn output_names_that_can_leave_their_folder_are_flagged() {
    let d = def(json!([
        { "id": "r", "type": "file-render", "parameters": { "filename": "../../home/zshrc" } },
        { "id": "j", "type": "file-join", "parameters": { "filename": "/etc/motd" } },
        { "id": "d", "type": "file-decode", "parameters": { "filename": "{{name}}.json" } },
        { "id": "n", "type": "file-rename", "parameters": { "prefix": "../../home/ren_", "pattern": "{{name}}" } },
        { "id": "o", "type": "output", "parameters": { "filename": "small-{{name}}" } }
    ]));
    assert_eq!(
        shown(&path_params(&d, None)),
        [
            row(
                "file-render",
                "filename",
                "../../home/zshrc",
                Some(PathFlag::ParentDir)
            ),
            row(
                "file-join",
                "filename",
                "/etc/motd",
                Some(PathFlag::Absolute)
            ),
            row("file-decode", "filename", "{{name}}.json", None),
            row(
                "file-rename",
                "prefix",
                "../../home/ren_",
                Some(PathFlag::ParentDir)
            ),
            row("file-rename", "pattern", "{{name}}", None),
            row("output", "filename", "small-{{name}}", None),
        ]
    );
}

#[test]
fn a_copy_that_overwrites_says_so() {
    let d = def(json!([{ "id": "c", "type": "file-copy",
        "parameters": { "destination": "backup", "conflict": "overwrite" } }]));
    assert_eq!(
        shown(&path_params(&d, None)),
        [
            row("file-copy", "destination", "backup", None),
            row(
                "file-copy",
                "conflict",
                "overwrite",
                Some(PathFlag::ReplacesExisting)
            ),
        ]
    );
}

/// Params that read like paths but hold none, each with why.
const NOT_PATHS: &[(&str, &str)] = &[
    // Base64 image data in a recipe; only `-p` reads a path, the user's own.
    ("image-overlay", "overlay"),
    // Choose among files already flowing, by name or folder; they read and
    // write nothing themselves.
    ("file-filter", "namePattern"),
    ("file-filter", "match"),
    ("file-collect", "pattern"),
    ("file-collect", "flatten"),
    ("file-collect", "recursive"),
    // Shape a file name but can't add a separator or `..`: an enum or a
    // number.
    ("file-rename", "sanitize"),
    ("file-rename", "maxLength"),
    ("file-rename", "case"),
    // A bare program name or a shell by name; the commands section shows both.
    ("shell-run", "command"),
    ("shell-run", "shell"),
    // Columns and match rules inside the CSV that `table` and `path` name
    // (both listed): a header or an enum, never a path.
    ("spreadsheet-lookup", "matchOn"),
    ("spreadsheet-lookup", "keyColumn"),
    ("spreadsheet-export", "nameColumn"),
    ("spreadsheet-read", "format"),
    // Options of a listed param, or text a step prints or keeps as metadata.
    ("file-copy", "createDirs"),
    ("file-move", "createDirs"),
    ("shell-run", "outputMode"),
    ("shell-run", "args"),
    ("output", "zip"),
    ("output", "autoDownload"),
    ("assert", "value"),
    ("edit-fields", "values"),
    ("report", "message"),
    ("file-render", "row"),
    ("file-render", "header"),
    ("file-render", "escape"),
];

/// A param whose name or description says it holds a path or folder.
fn looks_like_a_path(param: &bnto_core::ParameterDef) -> bool {
    const NAMES: &[&str] = &[
        "path",
        "paths",
        "base",
        "destination",
        "directory",
        "filename",
        "table",
        "overlay",
        "file",
        "folder",
        "searchPath",
        "cwd",
        "prefix",
        "suffix",
        "pattern",
    ];
    let text = param.description.to_lowercase();
    NAMES.contains(&param.name.as_str())
        || ["path", "folder", "directory", "file name", "filename"]
            .iter()
            .any(|word| text.contains(word))
}

#[test]
fn every_path_like_param_of_every_node_is_listed() {
    // A new node that writes, reads or names a path must be listed here
    // (`path_param_names`) or in NOT_PATHS with why, so the consent prompt
    // can't miss it.
    let mut missing = Vec::new();
    for node in bnto_engine::create_registry().catalog() {
        let listed = path_param_names(&node.node_type);
        for param in node.parameters.iter().filter(|p| looks_like_a_path(p)) {
            let exempt = NOT_PATHS.contains(&(node.node_type.as_str(), param.name.as_str()));
            if !listed.contains(&param.name.as_str()) && !exempt {
                missing.push(format!("{} {}", node.node_type, param.name));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "path-like params not listed: {missing:?}"
    );
}

fn effects_of(node: Value) -> Vec<(String, &'static str, String, Option<PathFlag>)> {
    let params = path_params(&def(json!([node])), None);
    params
        .iter()
        .map(|p| (p.node_type.clone(), p.param, p.value.clone(), p.flag))
        .filter(|(_, param, _, _)| matches!(*param, "mode" | "conflict" | "overwrite"))
        .collect()
}

#[test]
fn a_copy_that_keeps_the_newer_file_still_replaces_older_ones() {
    assert_eq!(
        effects_of(json!({ "id": "c", "type": "file-copy",
            "parameters": { "destination": "b", "conflict": "newer" } })),
        [row(
            "file-copy",
            "conflict",
            "newer",
            Some(PathFlag::ReplacesExisting)
        )]
    );
}

#[test]
fn export_overwrite_is_read_as_the_engine_reads_it() {
    for value in [json!(true), json!("true"), json!("TRUE"), json!("True")] {
        let node = json!({ "id": "e", "type": "spreadsheet-export",
            "parameters": { "path": "x.csv", "overwrite": value } });
        let effects = effects_of(node);
        assert_eq!(effects.len(), 1, "{value}");
        assert_eq!(effects[0].3, Some(PathFlag::ReplacesExisting), "{value}");
    }
    for value in [json!(false), json!("false"), json!("yes")] {
        let node = json!({ "id": "e", "type": "spreadsheet-export",
            "parameters": { "path": "x.csv", "overwrite": value } });
        assert!(effects_of(node).is_empty(), "{value}");
    }
}

#[test]
fn a_placeholder_in_a_mode_may_mean_overwrite() {
    let nodes = [
        json!({ "id": "o", "type": "output", "parameters": { "mode": "{{fields.mode}}" } }),
        json!({ "id": "c", "type": "file-copy",
            "parameters": { "destination": "b", "conflict": "{{vars.c}}" } }),
        json!({ "id": "e", "type": "spreadsheet-export",
            "parameters": { "path": "x.csv", "overwrite": "{{env.O}}" } }),
    ];
    for node in nodes {
        let effects = effects_of(node.clone());
        assert_eq!(effects.len(), 1, "{node}");
        assert_eq!(effects[0].3, Some(PathFlag::MayReplace), "{node}");
    }
}

#[test]
fn a_move_never_replaces_a_file_so_it_names_no_effect() {
    // file-move refuses a move onto an existing file (`relocate::conflicts`).
    let node = json!({ "id": "m", "type": "file-move", "parameters": { "destination": "d" } });
    assert!(effects_of(node).is_empty());
}

#[test]
fn every_part_a_rename_builds_its_name_from_is_listed_and_checked() {
    let d = def(json!([{ "id": "n", "type": "file-rename", "parameters": {
        "find": "(.*)", "replace": "$1", "separator": "/", "suffix": "_v2", "extension": "a/b"
    } }]));
    assert_eq!(
        shown(&path_params(&d, None)),
        [
            row("file-rename", "suffix", "_v2", None),
            row("file-rename", "replace", "$1", Some(PathFlag::FromMatch)),
            row(
                "file-rename",
                "extension",
                "a/b",
                Some(PathFlag::AddsFolder)
            ),
            row("file-rename", "separator", "/", Some(PathFlag::AddsFolder)),
            row("file-rename", "find", "(.*)", Some(PathFlag::EditsPath)),
        ]
    );
}

#[test]
fn a_find_and_replace_edits_the_whole_path() {
    // The `hook` repro: find "q", replace "." turns qgit/hooks/x into
    // .git/hooks/x, with nothing in `replace` itself to flag.
    let d = def(json!([
        { "id": "l", "type": "file-list", "parameters": { "base": "in", "paths": ["qgit/hooks/post-checkout"] } },
        { "id": "r", "type": "file-rename", "parameters": { "find": "q", "replace": "." } },
        { "id": "c", "type": "file-copy", "parameters": { "destination": "." } }
    ]));
    let params = path_params(&d, None);
    let find = params
        .iter()
        .find(|p| p.param == "find")
        .expect("find is listed");
    assert_eq!(find.flag, Some(PathFlag::EditsPath));
    assert_eq!(
        PathFlag::EditsPath.describe(),
        "edits the whole path, may move files to other folders"
    );
}

#[cfg(unix)]
#[test]
fn a_listed_path_is_judged_under_its_base() {
    // `in/link` leaves the project though neither `in` nor `link` alone does.
    let root = tempfile::tempdir().unwrap();
    let elsewhere = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join("in")).unwrap();
    std::os::unix::fs::symlink(elsewhere.path(), root.path().join("in/link")).unwrap();
    let d = def(json!([{ "id": "l", "type": "file-list",
        "parameters": { "base": "in", "paths": ["link/secret.txt", "ok.txt"] } }]));
    let params = path_params(&d, Some(root.path()));
    let flag = |value: &str| params.iter().find(|p| p.value == value).unwrap().flag;
    assert_eq!(flag("link/secret.txt"), Some(PathFlag::LeavesProject));
    assert_eq!(flag("ok.txt"), None);
    assert_eq!(flag("in"), None);
}
