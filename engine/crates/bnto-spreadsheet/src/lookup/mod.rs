// Lookup Rows Node — join each file to a CSV row by an exact key.
//
// The row's columns become the file's metadata, so later nodes can read them
// with `{{item.<column>}}` (e.g. `file-rename` `pattern: "{{item.new_name}}"`).
// Native-only: the table is read from a path on disk.

mod params;
mod processor;
mod source;
mod table;

pub use processor::SpreadsheetLookup;
