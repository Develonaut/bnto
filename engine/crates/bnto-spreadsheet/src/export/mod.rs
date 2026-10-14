// Export Items Node — write a run's files, with their metadata, as a CSV.
//
// One row per file: its pipeline name, then every metadata key seen across
// the files. Placed after a lookup and a rename, it records each file's new
// and old names and the row's columns. Native-only: it writes to disk.

mod params;
mod processor;
mod table;

pub use processor::SpreadsheetExport;
