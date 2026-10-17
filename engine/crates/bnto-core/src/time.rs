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

/// Parse an RFC 3339 UTC time (`Z` or `+00:00`, optional fraction) into
/// Unix epoch milliseconds; `None` for anything else, or a time before 1970.
pub fn parse_rfc3339_utc(text: &str) -> Option<u64> {
    let rest = text
        .strip_suffix(['Z', 'z'])
        .or_else(|| text.strip_suffix("+00:00"))?;
    let (date_time, fraction) = match rest.split_once('.') {
        Some((dt, frac)) => (dt, frac),
        None => (rest, ""),
    };
    let secs = parse_date_time(date_time)?;
    Some(secs.checked_mul(1000)? + fraction_ms(fraction)?)
}

/// `YYYY-MM-DDTHH:MM:SS` as Unix epoch seconds.
fn parse_date_time(text: &str) -> Option<u64> {
    let b = text.as_bytes();
    let shape_ok = b.len() == 19
        && b[4] == b'-'
        && b[7] == b'-'
        && matches!(b[10], b'T' | b't')
        && b[13] == b':'
        && b[16] == b':';
    if !shape_ok {
        return None;
    }
    let num = |range: std::ops::Range<usize>| text.get(range)?.parse::<u32>().ok();
    let (y, m, d) = (num(0..4)?, num(5..7)?, num(8..10)?);
    let (hh, mm, ss) = (num(11..13)?, num(14..16)?, num(17..19)?);
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) || hh > 23 || mm > 59 || ss > 60 {
        return None;
    }
    let days = u64::try_from(days_from_civil(y as i64, m, d)).ok()?;
    Some(days * 86400 + u64::from(hh * 3600 + mm * 60 + ss))
}

/// A fraction's first three digits as milliseconds (`5` → 500); any digits
/// past milliseconds are dropped. Empty is zero.
fn fraction_ms(fraction: &str) -> Option<u64> {
    if !fraction.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let ms: String = fraction.chars().chain("000".chars()).take(3).collect();
    ms.parse().ok()
}

/// Days since the Unix epoch for a civil date (Hinnant's `days_from_civil`).
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = i64::from((m + 9) % 12);
    let doy = (153 * mp + 2) / 5 + i64::from(d) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
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

    #[test]
    fn parse_reads_what_rfc3339_utc_writes() {
        for ms in [0, 1_700_000_000_123, 1_709_251_199_999] {
            assert_eq!(parse_rfc3339_utc(&rfc3339_utc(ms)), Some(ms));
        }
    }

    #[test]
    fn parse_takes_any_fraction_or_none() {
        assert_eq!(
            parse_rfc3339_utc("2023-11-14T22:13:20Z"),
            Some(1_700_000_000_000)
        );
        assert_eq!(
            parse_rfc3339_utc("2023-11-14T22:13:20.5Z"),
            Some(1_700_000_000_500)
        );
        assert_eq!(
            parse_rfc3339_utc("2023-11-14T22:13:20.123456Z"),
            Some(1_700_000_000_123)
        );
        assert_eq!(
            parse_rfc3339_utc("2023-11-14T22:13:20+00:00"),
            Some(1_700_000_000_000)
        );
    }

    #[test]
    fn parse_refuses_other_text() {
        for bad in [
            "",
            "now",
            "2023-11-14",
            "2023-11-14T22:13:20",
            "2023-13-01T00:00:00Z",
            "2023-11-14T22:13:20+02:00",
            "1969-12-31T23:59:59Z",
            "2023-11-14T25:00:00Z",
        ] {
            assert_eq!(parse_rfc3339_utc(bad), None, "{bad}");
        }
    }
}
