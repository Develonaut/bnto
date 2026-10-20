use super::*;

fn problems(command: &str, args: &[&str]) -> Vec<String> {
    program_text_problems(command, command, args)
}

#[test]
fn data_in_a_positional_program_is_refused() {
    for (command, args) in [
        ("awk", vec!["{ print \"{{fields.v}}\" }", "in.txt"]),
        (
            "gawk",
            vec!["-F", ",", "BEGIN { system(\"{{item.cmd}}\") }"],
        ),
        ("sed", vec!["s/x/{{fields.y}}/", "in.txt"]),
        ("sed", vec!["-n", "-E", "/{{path}}/p"]),
        (
            "jq",
            vec![".[] | select(.name == \"{{fields.n}}\")", "data.json"],
        ),
        ("jq", vec!["--arg", "k", "v", ".{{fields.key}}"]),
    ] {
        let found = problems(command, &args);
        assert_eq!(found.len(), 1, "{command} {args:?}");
        assert!(found[0].contains("program text"), "{}", found[0]);
    }
}

#[test]
fn data_in_code_flags_is_refused() {
    assert_eq!(problems("sed", &["-e", "s/a/{{fields.b}}/", "f"]).len(), 1);
    assert_eq!(problems("sed", &["--expression=s/{{url}}//"]).len(), 1);
    assert_eq!(
        problems("gawk", &["--source", "{{fields.p}}", "f"]).len(),
        1
    );
}

#[test]
fn the_safe_channels_are_allowed() {
    assert!(
        problems(
            "awk",
            &["-v", "name={{fields.v}}", "{ print name }", "{{path}}"]
        )
        .is_empty()
    );
    assert!(
        problems(
            "jq",
            &["--arg", "n", "{{fields.n}}", ".[] | select(.name == $n)"]
        )
        .is_empty()
    );
    assert!(problems("jq", &["--rawfile", "t", "{{path}}", "$t"]).is_empty());
}

#[test]
fn a_program_from_a_file_leaves_positionals_as_input() {
    assert!(problems("awk", &["-f", "prog.awk", "{{path}}"]).is_empty());
    assert!(problems("sed", &["-f", "edits.sed", "{{path}}"]).is_empty());
    assert!(problems("jq", &["-f", "filter.jq", "{{path}}"]).is_empty());
    assert!(problems("awk", &["-fprog.awk", "{{path}}"]).is_empty());
}

#[test]
fn input_files_after_the_program_are_data() {
    assert!(problems("awk", &["{ print $1 }", "{{path}}"]).is_empty());
    assert!(problems("sed", &["-e", "s/a/b/", "{{path}}"]).is_empty());
    assert!(problems("sed", &["s/a/b/", "--", "{{path}}"]).is_empty());
}

#[test]
fn other_tools_and_literal_look_alikes_are_ignored() {
    assert!(problems("grep", &["{{fields.pattern}}", "f"]).is_empty());
    assert!(problems("awk", &["{ print \"{{fields.*}}\" }"]).is_empty());
    assert!(problems("sed", &["s/{{vars.A}}/b/"]).is_empty());
}
