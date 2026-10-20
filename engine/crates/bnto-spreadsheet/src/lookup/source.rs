// Loads lookup tables from disk, parsed once and reused across a run.
//
// `process()` runs once per file, and one table can serve thousands of files,
// so re-reading and re-indexing the CSV per file would dominate the run. Each
// table is cached by path and key column, and is read again when its size or
// modified time changes (the TUI keeps one registry across runs). An edit that
// keeps both, within the filesystem's time resolution, can be missed.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use bnto_core::errors::BntoError;

use super::table::LookupTable;

/// (path, key column) → (size, modified time) and the parsed table.
type Cache = HashMap<(String, String), ((u64, Option<SystemTime>), Arc<LookupTable>)>;

/// Tables kept at once; a run holds a handful, a templated path could hold many.
const MAX_CACHED: usize = 16;

static TABLES: Mutex<Option<Cache>> = Mutex::new(None);

/// The table at `path`, indexed by `keyColumn`. Errors name the file.
pub(super) fn load_table(path: &str, key_column: &str) -> Result<Arc<LookupTable>, BntoError> {
    let stat =
        std::fs::metadata(path).map_err(|e| table_error(path, &format!("cannot be read: {e}")))?;
    let version = (stat.len(), stat.modified().ok());
    let key = (path.to_string(), key_column.to_string());
    let mut guard = TABLES.lock().unwrap_or_else(|e| e.into_inner());
    let cache = guard.get_or_insert_with(Cache::new);
    if let Some((cached, table)) = cache.get(&key)
        && *cached == version
    {
        return Ok(Arc::clone(table));
    }
    let table = Arc::new(read_table(path, key_column)?);
    if cache.len() >= MAX_CACHED {
        cache.clear();
    }
    cache.insert(key, (version, Arc::clone(&table)));
    Ok(table)
}

fn read_table(path: &str, key_column: &str) -> Result<LookupTable, BntoError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| table_error(path, &format!("cannot be read: {e}")))?;
    LookupTable::parse(&text, key_column).map_err(|msg| table_error(path, &msg))
}

fn table_error(path: &str, msg: &str) -> BntoError {
    BntoError::InvalidInput(format!("Lookup table {path} {msg}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_file_names_the_path() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("nope.csv");
        let path = path.to_str().unwrap();
        let err = load_table(path, "name").err().unwrap().to_string();
        assert!(err.contains(path), "{err}");
    }

    #[test]
    fn test_parse_error_names_the_path() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("t.csv");
        std::fs::write(&path, "from,to\n").unwrap();
        let path = path.to_str().unwrap();
        let err = load_table(path, "name").err().unwrap().to_string();
        assert!(err.contains(path) && err.contains("'name'"), "{err}");
    }

    #[test]
    fn test_two_tables_are_both_kept() {
        let tmp = tempfile::tempdir().unwrap();
        let (a, b) = (tmp.path().join("a.csv"), tmp.path().join("b.csv"));
        std::fs::write(&a, "name,v\nx,from-a\n").unwrap();
        std::fs::write(&b, "name,v\nx,from-b\n").unwrap();
        let (a, b) = (a.to_str().unwrap(), b.to_str().unwrap());
        let first = load_table(a, "name").unwrap();
        load_table(b, "name").unwrap();
        assert!(
            Arc::ptr_eq(&first, &load_table(a, "name").unwrap()),
            "a was re-read"
        );
        assert_eq!(
            load_table(b, "name").unwrap().get("x").unwrap()["v"],
            "from-b"
        );
    }

    #[test]
    fn test_edited_file_is_read_again() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("t.csv");
        std::fs::write(&path, "name,v\na,1\n").unwrap();
        let p = path.to_str().unwrap();
        assert_eq!(load_table(p, "name").unwrap().get("a").unwrap()["v"], "1");

        // A different length changes the key even when the mtime tick is coarse.
        std::fs::write(&path, "name,v\na,22\n").unwrap();
        assert_eq!(load_table(p, "name").unwrap().get("a").unwrap()["v"], "22");
    }
}
