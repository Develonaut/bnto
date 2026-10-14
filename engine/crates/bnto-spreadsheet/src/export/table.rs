// Builds the CSV for a run's items: one row per file, its name first, then
// every metadata key seen across the files, in the order first seen. A file's
// metadata map keeps its keys sorted, so keys new in one file are alphabetical.

/// One item to write: its pipeline name and the metadata it carries.
pub(super) struct Item<'a> {
    pub name: &'a str,
    pub metadata: &'a serde_json::Map<String, serde_json::Value>,
}

/// The CSV text for `items`, with `name_column` as the first header.
/// Fails when a metadata key would repeat that header.
pub(super) fn items_csv(items: &[Item<'_>], name_column: &str) -> Result<String, String> {
    let keys = metadata_keys(items);
    refuse_repeated_name_column(&keys, name_column)?;
    let mut writer = csv::WriterBuilder::new()
        .terminator(csv::Terminator::Any(b'\n'))
        .from_writer(Vec::new());
    write_record(
        &mut writer,
        std::iter::once(name_column).chain(keys.iter().map(String::as_str)),
    )?;
    for item in items {
        let cells: Vec<String> = keys.iter().map(|k| cell(item.metadata.get(k))).collect();
        write_record(
            &mut writer,
            std::iter::once(item.name).chain(cells.iter().map(String::as_str)),
        )?;
    }
    let bytes = writer.into_inner().map_err(|e| e.to_string())?;
    String::from_utf8(bytes).map_err(|e| e.to_string())
}

fn refuse_repeated_name_column(keys: &[String], name_column: &str) -> Result<(), String> {
    if !keys.iter().any(|k| k == name_column) {
        return Ok(());
    }
    Err(format!(
        "a file's metadata has a '{name_column}' key, which would repeat the name column; \
         set name_column to another header"
    ))
}

/// Every metadata key across `items`, in the order first seen (alphabetical
/// within one file).
fn metadata_keys(items: &[Item<'_>]) -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();
    for key in items.iter().flat_map(|item| item.metadata.keys()) {
        if !keys.contains(key) {
            keys.push(key.clone());
        }
    }
    keys
}

fn write_record<'a>(
    writer: &mut csv::Writer<Vec<u8>>,
    fields: impl Iterator<Item = &'a str>,
) -> Result<(), String> {
    writer.write_record(fields).map_err(|e| e.to_string())
}

/// A metadata value as cell text: strings as-is, absent or null empty,
/// numbers and booleans as written, arrays and objects as JSON.
fn cell(value: Option<&serde_json::Value>) -> String {
    match value {
        None | Some(serde_json::Value::Null) => String::new(),
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(other) => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn map(v: serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
        v.as_object().unwrap().clone()
    }

    fn rows(csv_text: &str) -> Vec<Vec<String>> {
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv_text.as_bytes())
            .records()
            .map(|r| r.unwrap().iter().map(str::to_string).collect())
            .collect()
    }

    #[test]
    fn test_header_is_the_union_of_keys_with_empty_cells_for_missing_ones() {
        let (a, b, c) = (
            map(json!({ "pack": "Guns", "catid": "GUNMech" })),
            map(json!({ "catid": "DRUMKick", "unmatched": 0 })),
            map(json!({ "original_name": "x.wav" })),
        );
        let items = [
            Item {
                name: "a.wav",
                metadata: &a,
            },
            Item {
                name: "b.wav",
                metadata: &b,
            },
            Item {
                name: "c.wav",
                metadata: &c,
            },
        ];
        let rows = rows(&items_csv(&items, "file").unwrap());
        // a.wav's keys are new and alphabetical; b's and c's new keys follow.
        assert_eq!(
            rows[0],
            ["file", "catid", "pack", "unmatched", "original_name"]
        );
        assert_eq!(rows[1], ["a.wav", "GUNMech", "Guns", "", ""]);
        assert_eq!(rows[2], ["b.wav", "DRUMKick", "", "0", ""]);
        assert_eq!(rows[3], ["c.wav", "", "", "", "x.wav"]);
    }

    #[test]
    fn test_a_name_with_a_comma_a_quote_and_non_ascii_round_trips() {
        let meta = map(json!({ "note": "say \"hi\", ok" }));
        let name = "Pack/Café, \"live\" 01.wav";
        let text = items_csv(
            &[Item {
                name,
                metadata: &meta,
            }],
            "file",
        )
        .unwrap();
        let rows = rows(&text);
        assert_eq!(rows[1], [name, "say \"hi\", ok"]);
        assert!(!text.contains('\r'), "LF line endings");
    }

    #[test]
    fn test_values_become_cell_text() {
        let meta = map(json!({ "n": 3, "b": true, "z": null, "list": ["pattern"] }));
        let rows = rows(
            &items_csv(
                &[Item {
                    name: "a",
                    metadata: &meta,
                }],
                "file",
            )
            .unwrap(),
        );
        assert_eq!(rows[0], ["file", "b", "list", "n", "z"]);
        assert_eq!(rows[1], ["a", "true", "[\"pattern\"]", "3", ""]);
    }

    #[test]
    fn test_no_items_writes_only_the_name_header() {
        assert_eq!(items_csv(&[], "file").unwrap(), "file\n");
    }

    #[test]
    fn test_a_metadata_key_named_like_the_name_column_fails() {
        let meta = map(json!({ "file": "x" }));
        let err = items_csv(
            &[Item {
                name: "a",
                metadata: &meta,
            }],
            "file",
        )
        .unwrap_err();
        assert!(
            err.contains("'file'") && err.contains("name_column"),
            "{err}"
        );
    }
}
