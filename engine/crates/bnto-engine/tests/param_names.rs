// Every processor param is camelCase, like the rest of the recipe format
// (`.claude/scopes/rust/node-vocabulary.md`, Shared Params). A new
// snake_case param fails here instead of reaching recipes.

use bnto_engine::create_registry;

fn is_camel_case(name: &str) -> bool {
    name.starts_with(|c: char| c.is_ascii_lowercase())
        && name.chars().all(|c| c.is_ascii_alphanumeric())
}

#[test]
fn every_processor_param_is_camel_case() {
    let offenders: Vec<String> = create_registry()
        .catalog()
        .iter()
        .flat_map(|meta| {
            meta.parameters
                .iter()
                .filter(|p| !is_camel_case(&p.name))
                .map(move |p| format!("{}.{}", meta.node_type, p.name))
        })
        .collect();

    assert!(offenders.is_empty(), "snake_case params: {offenders:?}");
}

#[test]
fn camel_case_check_accepts_and_refuses_the_right_names() {
    assert!(is_camel_case("createDirs"));
    assert!(is_camel_case("quality"));
    assert!(!is_camel_case("create_dirs"));
    assert!(!is_camel_case("CreateDirs"));
    assert!(!is_camel_case("max-size"));
}
