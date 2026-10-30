use super::*;
use bnto_core::NoopContext;
use bnto_core::metadata::InputCardinality;
use serde_json::{Value, json};

fn item(name: &str, metadata: Value) -> BatchFile {
    BatchFile {
        data: FileData::Bytes(Vec::new()),
        filename: name.to_string(),
        mime_type: Some("text/csv".to_string()),
        metadata: metadata.as_object().cloned().unwrap_or_default(),
    }
}

fn render(params: Value, files: Vec<BatchFile>) -> Result<NodeOutput, BntoError> {
    let input = BatchInput {
        files,
        params: params.as_object().cloned().unwrap(),
    };
    FileRender::new().process_batch(input, &ProgressReporter::new_noop(), &NoopContext)
}

fn text(out: &NodeOutput) -> String {
    assert_eq!(out.files.len(), 1, "one rendered file");
    let bytes = out.files[0].data.to_bytes().unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

fn table(escape: &str) -> Value {
    json!({
        "header": "| Name | Note |\n|---|---|",
        "row": "| {{item.name}} | {{item.note}} |",
        "footer": "{{count}} items",
        "filename": "report.md",
        "escape": escape,
    })
}

fn three_items() -> Vec<BatchFile> {
    vec![
        item("a_row0.csv", json!({ "name": "alpha", "note": "first" })),
        item("a_row1.csv", json!({ "name": "beta", "note": "second" })),
        item("a_row2.csv", json!({ "name": "gamma", "note": "third" })),
    ]
}

// --- Layout ---

#[test]
fn test_three_items_give_header_three_rows_and_footer_in_order() {
    let out = render(table("none"), three_items()).unwrap();
    assert_eq!(
        text(&out),
        "| Name | Note |\n|---|---|\n\
         | alpha | first |\n\
         | beta | second |\n\
         | gamma | third |\n\
         3 items\n"
    );
}

#[test]
fn test_zero_items_give_header_and_footer() {
    let out = render(table("none"), vec![]).unwrap();
    assert_eq!(text(&out), "| Name | Note |\n|---|---|\n0 items\n");
}

#[test]
fn test_zero_items_and_no_header_or_footer_give_an_empty_file() {
    let params = json!({ "row": "{{item.name}}", "filename": "list.txt" });
    let out = render(params, vec![]).unwrap();
    assert_eq!(text(&out), "");
}

#[test]
fn test_separator_goes_between_rows_only() {
    let params = json!({
        "header": "Items:",
        "row": "{{item.name}}",
        "separator": ", ",
        "filename": "list.txt",
    });
    let out = render(params, three_items()).unwrap();
    assert_eq!(text(&out), "Items:\nalpha, beta, gamma\n");
}

#[test]
fn test_separator_defaults_to_a_newline() {
    let params = json!({ "row": "- {{item.name}}", "filename": "list.md" });
    let out = render(params, three_items()).unwrap();
    assert_eq!(text(&out), "- alpha\n- beta\n- gamma\n");
}

#[test]
fn test_an_empty_separator_joins_rows_directly() {
    let params = json!({ "row": "{{item.name}};", "separator": "", "filename": "x.txt" });
    let out = render(params, three_items()).unwrap();
    assert_eq!(text(&out), "alpha;beta;gamma;\n");
}

#[test]
fn test_a_value_that_reads_like_a_token_is_written_as_it_is() {
    let files = vec![item("a", json!({ "v": "{{file}} {{item.v}} {{count}}" }))];
    let params = json!({ "row": "{{item.v}} {{other}}", "filename": "x.txt" });
    let out = render(params, files).unwrap();
    assert_eq!(text(&out), "{{file}} {{item.v}} {{count}} {{other}}\n");
}

#[test]
fn test_file_names_each_items_file() {
    let params = json!({ "row": "{{file}}: {{item.note}}", "filename": "list.txt" });
    let out = render(params, three_items()).unwrap();
    assert_eq!(
        text(&out),
        "a_row0.csv: first\na_row1.csv: second\na_row2.csv: third\n"
    );
}

#[test]
fn test_values_that_are_not_text_are_written_as_json() {
    let files = vec![item("a", json!({ "n": 3, "b": true, "z": null, "l": [1] }))];
    let params =
        json!({ "row": "{{item.n}} {{item.b}} [{{item.z}}] {{item.l}}", "filename": "x.txt" });
    let out = render(params, files).unwrap();
    assert_eq!(text(&out), "3 true [] [1]\n");
}

// --- Escaping ---

#[test]
fn test_markdown_escapes_a_pipe_inside_a_field() {
    let files = vec![item("a", json!({ "name": "left|right", "note": "ok" }))];
    let out = render(table("markdown"), files).unwrap();
    assert!(
        text(&out).contains("| left\\|right | ok |\n"),
        "{}",
        text(&out)
    );
}

#[test]
fn test_none_leaves_a_pipe_inside_a_field_as_is() {
    let files = vec![item("a", json!({ "name": "left|right", "note": "ok" }))];
    let out = render(table("none"), files).unwrap();
    assert!(
        text(&out).contains("| left|right | ok |\n"),
        "{}",
        text(&out)
    );
}

#[test]
fn test_markdown_turns_newlines_in_a_value_into_br() {
    let files = vec![item(
        "a",
        json!({ "name": "one\ntwo\r\nthree", "note": "" }),
    )];
    let out = render(table("markdown"), files).unwrap();
    assert!(
        text(&out).contains("| one<br>two<br>three |  |\n"),
        "{}",
        text(&out)
    );
}

#[test]
fn test_markdown_escapes_inline_markup_in_values_and_file_names() {
    let files = vec![item("my_file*.txt", json!({ "v": "`a` [b](c) <d> \\ #1" }))];
    let params = json!({ "row": "{{file}} {{item.v}}", "filename": "x.md", "escape": "markdown" });
    let out = render(params, files).unwrap();
    assert_eq!(
        text(&out),
        "my\\_file\\*.txt \\`a\\` \\[b\\](c) \\<d\\> \\\\ #1\n"
    );
}

#[test]
fn test_escaping_never_touches_the_template_text() {
    let files = vec![item("a", json!({ "name": "x|y", "note": "n" }))];
    let params = json!({
        "header": "**Report** | _all_",
        "row": "* {{item.name}} |",
        "footer": "`done`",
        "filename": "x.md",
        "escape": "markdown",
    });
    let out = render(params, files).unwrap();
    assert_eq!(text(&out), "**Report** | _all_\n* x\\|y |\n`done`\n");
}

// --- Missing keys and refused params ---

#[test]
fn test_a_missing_item_key_fails_naming_the_key_and_the_item() {
    let files = vec![
        item("a.txt", json!({ "name": "alpha", "note": "n" })),
        item("b.txt", json!({ "name": "beta" })),
    ];
    let err = render(table("none"), files).err().unwrap().to_string();
    assert!(
        err.contains("{{item.note}}") && err.contains("b.txt"),
        "{err}"
    );
    assert!(err.contains("it has: name"), "{err}");
}

#[test]
fn test_header_and_footer_may_not_read_an_item() {
    // `{{item.*}}` is the loop's item inside a loop, which validate can't
    // see, so only the run refuses it (outside one); `{{file}}` never fills.
    for (param, value, validate_refuses) in [
        ("header", "{{item.name}}", false),
        ("footer", "{{file}}", true),
        ("filename", "{{item.name}}.md", false),
    ] {
        let mut params = table("none");
        params[param] = json!(value);
        let errors = FileRender::new().validate(params.as_object().unwrap());
        assert_eq!(
            errors.len(),
            usize::from(validate_refuses),
            "{param}: {errors:?}"
        );
        let err = render(params, three_items()).err().unwrap().to_string();
        assert!(err.contains(param), "{err}");
    }
}

#[test]
fn test_validate_refuses_a_missing_row_or_filename_or_an_unknown_escape() {
    let render_node = FileRender::new();
    let cases = [
        (json!({ "filename": "a.md" }), "row"),
        (json!({ "row": "x" }), "filename"),
        (json!({ "row": "x", "filename": "" }), "filename"),
        (
            json!({ "row": "x", "filename": "a.md", "escape": "html" }),
            "escape",
        ),
    ];
    for (params, word) in cases {
        let errors = render_node.validate(params.as_object().unwrap());
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(errors[0].contains(word), "{errors:?}");
    }
    let ok = json!({ "row": "{{item.x}}", "filename": "{{vars.Name}}.md" });
    assert!(render_node.validate(ok.as_object().unwrap()).is_empty());
}

// --- Output file and metadata ---

#[test]
fn test_output_is_one_in_memory_file_named_by_filename() {
    let out = render(table("none"), three_items()).unwrap();
    assert_eq!(out.files[0].filename, "report.md");
    assert!(matches!(out.files[0].data, FileData::Bytes(_)));
}

#[test]
fn test_mime_follows_the_extension() {
    for (filename, mime) in [
        ("report.md", "text/markdown"),
        ("REPORT.MD", "text/markdown"),
        ("report.markdown", "text/markdown"),
        ("report.txt", "text/plain"),
        ("report", "text/plain"),
    ] {
        let params = json!({ "row": "x", "filename": filename });
        let out = render(params, vec![]).unwrap();
        assert_eq!(out.files[0].mime_type, mime, "{filename}");
    }
}

#[test]
fn test_node_metadata_records_the_row_count() {
    let out = render(table("none"), three_items()).unwrap();
    assert_eq!(out.metadata["rows"], 3);
    let out = render(table("none"), vec![]).unwrap();
    assert_eq!(out.metadata["rows"], 0);
    assert_eq!(FileRender::new().count_keys(), ["rows"]);
}

#[test]
fn test_metadata_is_a_browser_capable_batch_file_node() {
    let node = FileRender::new();
    assert_eq!(node.name(), "file-render");
    let meta = node.metadata();
    assert_eq!(meta.node_type, "file-render");
    assert_eq!(meta.category, bnto_core::metadata::NodeCategory::File);
    assert_eq!(meta.input_cardinality, InputCardinality::Batch);
    assert!(meta.platforms.contains(&"browser".to_string()));
    // It fills {{item.*}} in `row` itself, so the executor hands them over.
    assert!(node.resolves_items());
    let params: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(
        params,
        ["header", "row", "footer", "separator", "filename", "escape"]
    );
}

#[test]
fn test_schema_defaults_match_the_reader() {
    let defs = params::build_render_parameters();
    let default = |name: &str| {
        defs.iter()
            .find(|d| d.name == name)
            .unwrap()
            .default
            .clone()
    };
    assert_eq!(default("separator"), Some(json!("\n")));
    assert_eq!(default("escape"), Some(json!("none")));
}

#[test]
fn test_mime_for_a_dotfile_is_plain_text() {
    // ISS-33: `notes/.md` has no extension.
    assert_eq!(mime_for("notes/.md"), "text/plain");
    assert_eq!(mime_for("v1.2/notes.md"), "text/markdown");
}

// --- Inside a loop ---

fn render_in_loop(
    params: Value,
    files: Vec<BatchFile>,
    item: Value,
) -> Result<NodeOutput, BntoError> {
    let input = BatchInput {
        files,
        params: params.as_object().cloned().unwrap(),
    };
    let ctx = crate::in_loop::in_loop(item);
    FileRender::new().process_batch(input, &ProgressReporter::new_noop(), &ctx)
}

#[test]
fn test_in_a_loop_header_footer_and_filename_read_the_loop_item() {
    let params = json!({
        "header": "# {{item.slug}} ({{count}})", "row": "- {{item.name}}",
        "footer": "{{item.note}}", "filename": "{{item.slug}}.md",
    });
    let item = json!({ "slug": "team", "note": "{{count}}" });
    let out = render_in_loop(params, three_items(), item).unwrap();
    assert_eq!(out.files[0].filename, "team.md");
    // A value that reads like a token is written as it is (ISS-35).
    assert_eq!(
        text(&out),
        "# team (3)\n- alpha\n- beta\n- gamma\n{{count}}\n"
    );
}

#[test]
fn test_in_a_loop_a_row_reads_the_loop_item_on_top_and_escapes_it() {
    // RED (ISS-58): a row reads the item's own keys and the loop's, the
    // loop's winning a shared key; filled values are escaped.
    let params =
        json!({ "row": "{{item.name}} {{item.tag}}", "filename": "x.md", "escape": "markdown" });
    let files = vec![item("a.csv", json!({ "name": "alpha", "tag": "own" }))];
    let out = render_in_loop(params, files, json!({ "tag": "a|b" })).unwrap();
    assert_eq!(text(&out), "alpha a\\|b\n");
}

#[test]
fn test_in_a_loop_a_written_once_template_still_may_not_read_the_file() {
    let params = json!({ "row": "x", "filename": "{{file}}.md" });
    let err = render_in_loop(params, three_items(), json!({}))
        .err()
        .unwrap();
    assert!(err.to_string().contains("'filename'"), "{err}");
}

#[test]
fn test_in_a_loop_a_key_the_loop_item_lacks_is_refused_naming_it() {
    let params = json!({ "row": "x", "filename": "{{item.slug}}.md" });
    let err = render_in_loop(params, three_items(), json!({ "other": 1 }))
        .err()
        .unwrap();
    let err = err.to_string();
    assert!(
        err.contains("'filename'") && err.contains("slug") && err.contains("other"),
        "{err}"
    );
}
