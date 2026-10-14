// A CSV table indexed by one column, for exact-key row lookups.

use std::collections::HashMap;

/// One table row: header → cell, every value a string.
pub(super) type Row = serde_json::Map<String, serde_json::Value>;

/// Rows of a CSV, findable by the value in their key column.
pub(super) struct LookupTable {
    rows: Vec<Row>,
    index: HashMap<String, usize>,
}

impl LookupTable {
    /// Parse CSV text with a header row. A key may appear on one row only;
    /// rows with an empty key (spreadsheet trailer rows) are skipped, since no
    /// file has an empty name. Errors name the line at fault.
    pub(super) fn parse(text: &str, key_column: &str) -> Result<Self, String> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(text.as_bytes());
        let headers = read_headers(&mut reader)?;
        let key_at = column_index(&headers, key_column)?;
        let mut table = Self {
            rows: Vec::new(),
            index: HashMap::new(),
        };
        for record in reader.records() {
            let record = record.map_err(|e| e.to_string())?;
            table.insert(&record, &headers, key_at)?;
        }
        Ok(table)
    }

    fn insert(
        &mut self,
        record: &csv::StringRecord,
        headers: &csv::StringRecord,
        key_at: usize,
    ) -> Result<(), String> {
        let key = record.get(key_at).unwrap_or_default().to_string();
        if key.is_empty() {
            return Ok(());
        }
        if self.index.contains_key(&key) {
            return Err(format!(
                "line {}: key '{key}' appears on more than one row",
                line_of(record)
            ));
        }
        self.index.insert(key, self.rows.len());
        self.rows.push(to_row(record, headers));
        Ok(())
    }

    /// The row whose key column equals `key` exactly.
    pub(super) fn get(&self, key: &str) -> Option<&Row> {
        self.index.get(key).map(|&i| &self.rows[i])
    }
}

fn read_headers(reader: &mut csv::Reader<&[u8]>) -> Result<csv::StringRecord, String> {
    let headers = reader.headers().map_err(|e| e.to_string())?.clone();
    if headers.iter().all(str::is_empty) {
        return Err("has no header row".to_string());
    }
    if let Some(name) = first_repeat(&headers) {
        return Err(format!("names the column '{name}' more than once"));
    }
    Ok(headers)
}

fn first_repeat(headers: &csv::StringRecord) -> Option<&str> {
    let mut seen = std::collections::HashSet::new();
    headers.iter().find(|h| !seen.insert(*h))
}

fn column_index(headers: &csv::StringRecord, name: &str) -> Result<usize, String> {
    headers.iter().position(|h| h == name).ok_or_else(|| {
        let found = headers.iter().collect::<Vec<_>>().join(", ");
        format!("has no '{name}' column (columns: {found})")
    })
}

fn to_row(record: &csv::StringRecord, headers: &csv::StringRecord) -> Row {
    headers
        .iter()
        .zip(record.iter())
        .map(|(k, v)| (k.to_string(), serde_json::Value::String(v.to_string())))
        .collect()
}

fn line_of(record: &csv::StringRecord) -> u64 {
    record.position().map_or(0, csv::Position::line)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE: &str = "name,new_name,pack\n\
                         Pack/a.wav,A.wav,Guns\n\
                         Pack/b.wav,B.wav,Steps\n";

    #[test]
    fn test_get_returns_the_row_with_every_column() {
        let t = LookupTable::parse(TABLE, "name").unwrap();
        let row = t.get("Pack/b.wav").unwrap();
        assert_eq!(row["name"], "Pack/b.wav");
        assert_eq!(row["new_name"], "B.wav");
        assert_eq!(row["pack"], "Steps");
    }

    #[test]
    fn test_get_is_exact() {
        let t = LookupTable::parse(TABLE, "name").unwrap();
        assert!(t.get("pack/a.wav").is_none(), "case-sensitive");
        assert!(t.get("a.wav").is_none(), "whole value only");
        assert!(t.get("Pack/a.wav ").is_none(), "no trimming");
    }

    #[test]
    fn test_any_column_can_be_the_key() {
        let t = LookupTable::parse(TABLE, "new_name").unwrap();
        assert_eq!(t.get("A.wav").unwrap()["name"], "Pack/a.wav");
    }

    #[test]
    fn test_quoted_cells_with_commas_and_quotes() {
        let t = LookupTable::parse("name,v\n\"a, \"\"b\"\".wav\",1\n", "name").unwrap();
        assert_eq!(t.get("a, \"b\".wav").unwrap()["v"], "1");
    }

    #[test]
    fn test_utf8_bom_is_ignored_in_the_header() {
        let t = LookupTable::parse("\u{feff}name,v\nx,1\n", "name").unwrap();
        assert!(t.get("x").is_some());
    }

    #[test]
    fn test_duplicate_key_fails_naming_its_line() {
        let err = LookupTable::parse("name,v\nx,1\ny,2\nx,3\n", "name")
            .err()
            .unwrap();
        assert!(err.contains("line 4"), "{err}");
        assert!(err.contains("'x'"), "{err}");
    }

    #[test]
    fn test_missing_key_column_is_named() {
        let err = LookupTable::parse("from,to\na,b\n", "name").err().unwrap();
        assert!(err.contains("'name'"), "{err}");
        assert!(err.contains("from, to"), "{err}");
    }

    #[test]
    fn test_empty_text_has_no_header() {
        let err = LookupTable::parse("", "name").err().unwrap();
        assert!(err.contains("no header"), "{err}");
    }

    #[test]
    fn test_rows_with_an_empty_key_are_skipped() {
        // Two keyless rows would otherwise fail as a duplicate key ''.
        let t = LookupTable::parse("name,v\nx,1\n,2\n,\n", "name").unwrap();
        assert!(t.get("x").is_some());
        assert!(t.get("").is_none());
    }

    #[test]
    fn test_a_repeated_header_fails_naming_it() {
        let err = LookupTable::parse("name,v,v\nx,1,2\n", "name")
            .err()
            .unwrap();
        assert!(err.contains("'v'"), "{err}");
    }

    #[test]
    fn test_ragged_row_fails() {
        assert!(LookupTable::parse("name,v\nx\n", "name").is_err());
    }
}
