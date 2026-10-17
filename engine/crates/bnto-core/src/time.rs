// Civil (calendar) time in UTC from Unix epoch time, with no date crate.
//
// Templates format dates from these: `{{ctx.date}}`/`{{ctx.timestamp}}` from
// the wall clock, `{{ctx.started}}` from the run's start. Nodes that report a
// time (e.g. a file's modified time) use `rfc3339_utc` so it compares with
// `{{ctx.started}}`.

/// Convert current wall-clock time to civil (year, month, day, hour, min, sec).
///
/// Uses Howard Hinnant's civil calendar algorithm on the Unix epoch.
/// No `chrono` dependency — pure arithmetic. `SystemTime::now()` works in
/// WASM (backed by `Date.now()`).
pub(crate) fn now_civil() -> (i32, u32, u32, u32, u32, u32) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    epoch_to_civil(secs)
}

/// Convert Unix epoch seconds to (year, month, day, hour, minute, second) in UTC.
///
/// Hinnant's algorithm: http://howardhinnant.github.io/date_algorithms.html
fn epoch_to_civil(epoch_secs: u64) -> (i32, u32, u32, u32, u32, u32) {
    let total_secs = epoch_secs;
    let day_secs = (total_secs % 86400) as u32;
    let hh = day_secs / 3600;
    let mm = (day_secs % 3600) / 60;
    let ss = day_secs % 60;

    // Days since epoch, shifted so day 0 = 0000-03-01
    let z = (total_secs / 86400) as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u32; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    (y as i32, m, d, hh, mm, ss)
}

/// Unix epoch milliseconds as an RFC 3339 UTC time with milliseconds,
/// e.g. `2026-10-17T15:22:56.123Z`.
pub fn rfc3339_utc(epoch_ms: u64) -> String {
    let (y, m, d, hh, mm, ss) = epoch_to_civil(epoch_ms / 1000);
    let ms = epoch_ms % 1000;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}.{ms:03}Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc3339_formats_the_epoch() {
        assert_eq!(rfc3339_utc(0), "1970-01-01T00:00:00.000Z");
    }

    #[test]
    fn rfc3339_keeps_milliseconds() {
        assert_eq!(rfc3339_utc(1_700_000_000_123), "2023-11-14T22:13:20.123Z");
    }

    #[test]
    fn rfc3339_handles_a_leap_day() {
        // 2024-02-29T23:59:59.999Z
        assert_eq!(rfc3339_utc(1_709_251_199_999), "2024-02-29T23:59:59.999Z");
    }
}
