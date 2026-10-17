// Read Rows Node — explode a table file (CSV, or a JSON array) into one item per row.
//
// Each row becomes an OutputFile with empty `data` and its values in
// `metadata`, so loops and per-file templates read them as `{{item.<key>}}`.

mod csv_rows;
mod json_rows;
mod params;
mod processor;
mod table;

pub use processor::ReadSpreadsheet;

#[cfg(test)]
mod json_tests;
