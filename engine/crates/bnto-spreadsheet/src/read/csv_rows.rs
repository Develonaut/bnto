// CSV parsing for spreadsheet-read: each record becomes one row of string cells,
// keyed by its header (or `col0`, `col1`, … without headers).

use bnto_core::errors::BntoError;

use super::params::ReadConfig;
use super::table::{Row, Table};

/// Read CSV bytes into a table, per the delimiter and header settings.
pub(super) fn read(data: &[u8], config: &ReadConfig) -> Result<Table, BntoError> {
    let csv_text = parse_and_strip_bom(data)?;
    let delimiter = resolve_delimiter(&config.delimiter);
    let (headers, records) = read_csv(csv_text, delimiter, config.has_headers)?;
    Ok(Table {
        column_count: headers.len(),
        rows: records.iter().map(|r| to_row(&headers, r)).collect(),
    })
}

/// Validate UTF-8 and strip BOM if present.
fn parse_and_strip_bom(data: &[u8]) -> Result<&str, BntoError> {
    let data = data.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(data);

    let text = std::str::from_utf8(data).map_err(|e| {
        BntoError::InvalidInput(format!(
            "File is not valid UTF-8 text (is this really a CSV?): {e}"
        ))
    })?;

    if text.trim().is_empty() {
        return Err(BntoError::InvalidInput(
            "CSV file is empty — no data to read".to_string(),
        ));
    }

    Ok(text)
}

/// Map delimiter name to byte.
fn resolve_delimiter(delimiter: &str) -> u8 {
    match delimiter {
        "comma" | "," => b',',
        "semicolon" | ";" => b';',
        "tab" | "\t" => b'\t',
        "pipe" | "|" => b'|',
        other if other.len() == 1 => other.as_bytes()[0],
        _ => b',',
    }
}

/// Read CSV text into headers + records. When `has_headers` is false,
/// generates synthetic column names (col0, col1, ...) from the first record.
fn read_csv(
    csv_text: &str,
    delimiter: u8,
    has_headers: bool,
) -> Result<(Vec<String>, Vec<csv::StringRecord>), BntoError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .delimiter(delimiter)
        .flexible(true)
        .from_reader(csv_text.as_bytes());
    let headers = if has_headers {
        read_headers(&mut reader)?
    } else {
        Vec::new()
    };
    let records = reader
        .records()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV row: {e}")))?;
    let headers = match records.first() {
        Some(first) if !has_headers => (0..first.len()).map(|i| format!("col{i}")).collect(),
        _ => headers,
    };
    Ok((headers, records))
}

/// The header record, each name trimmed.
fn read_headers(reader: &mut csv::Reader<&[u8]>) -> Result<Vec<String>, BntoError> {
    let raw = reader
        .headers()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read CSV headers: {e}")))?;
    Ok(raw.iter().map(|h| h.trim().to_string()).collect())
}

/// One record's cells keyed by header; a short record's missing cells are "".
fn to_row(headers: &[String], record: &csv::StringRecord) -> Row {
    headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            let value = record.get(i).unwrap_or("").to_string();
            (header.clone(), serde_json::Value::String(value))
        })
        .collect()
}
