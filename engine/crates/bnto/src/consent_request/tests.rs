use super::*;
use crate::path_flag::PathFlag;
use std::path::PathBuf;

fn def(json: serde_json::Value) -> PipelineDefinition {
    serde_json::from_value(json).unwrap()
}

fn project(file: Option<PathBuf>) -> SourceKind {
    SourceKind::Project {
        root: PathBuf::from("/repo"),
        file,
    }
}

#[test]
fn a_project_recipe_names_its_slug_and_file() {
    let file = PathBuf::from("/repo/.bnto/recipes/tidy.bnto.json");
    assert_eq!(
        origin(&project(Some(file)), "tidy"),
        "Project recipe `tidy` from `/repo/.bnto/recipes/tidy.bnto.json`"
    );
    assert_eq!(
        origin(&project(None), "tidy"),
        "Project recipe `tidy`, edited here"
    );
}

#[test]
fn library_and_path_recipes_name_their_file() {
    let file = PathBuf::from("/home/me/.bnto/recipes/esc.bnto.json");
    assert_eq!(
        origin(&SourceKind::Library(Some(file)), "esc"),
        "Library recipe `esc` from `/home/me/.bnto/recipes/esc.bnto.json`"
    );
    assert_eq!(
        origin(&SourceKind::Library(None), "esc"),
        "Local recipe `esc`"
    );
    assert_eq!(
        origin(
            &SourceKind::Path(PathBuf::from("x/a.bnto.json")),
            "x/a.bnto.json"
        ),
        "Recipe file `x/a.bnto.json`"
    );
}

#[test]
fn steps_show_by_node_type_not_the_authors_name() {
    let d = def(serde_json::json!({ "nodes": [
        { "id": "in", "type": "input", "name": "Pick photos" },
        { "id": "g", "type": "group", "name": "Harmless tidy", "nodes": [
            { "id": "del", "type": "file-delete", "name": "Just a rename",
              "parameters": { "base": "/Users/me" } }
        ] }
    ] }));
    let request = ConsentRequest::new(&d, "Tidy".into(), "tidy", &project(None), "h".into());
    assert_eq!(request.steps, ["input", "group", "  file-delete"]);
    assert!(request.project);
    assert_eq!(request.paths[0].flag, Some(PathFlag::Absolute));
}

#[test]
fn a_library_recipe_is_not_a_projects() {
    let d = def(serde_json::json!({ "nodes": [] }));
    let request = ConsentRequest::new(
        &d,
        "Mine".into(),
        "mine",
        &SourceKind::Library(None),
        "h".into(),
    );
    assert!(!request.project);
    assert!(request.steps.is_empty());
}
