// Shared formatting helpers — human-readable sizes and durations.

/// Format milliseconds as human-readable duration.
pub fn format_duration(ms: u64) -> String {
    if ms < 1000 {
        return format!("{}ms", ms);
    }
    let secs = ms / 1000;
    let tenths = (ms % 1000) / 100;
    if secs < 60 {
        return format!("{}.{}s", secs, tenths);
    }
    let mins = secs / 60;
    let remaining_secs = secs % 60;
    format!("{}m {}s", mins, remaining_secs)
}

/// Format bytes as human-readable size.
pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{} B", bytes);
    }
    let kb = bytes as f64 / 1024.0;
    if kb < 1024.0 {
        return format!("{:.0} KB", kb);
    }
    let mb = kb / 1024.0;
    format!("{:.1} MB", mb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_duration_human_readable() {
        assert_eq!(format_duration(500), "500ms");
        assert_eq!(format_duration(4123), "4.1s");
        assert_eq!(format_duration(65000), "1m 5s");
    }

    #[test]
    fn format_duration_boundary_at_one_second() {
        assert_eq!(format_duration(999), "999ms");
        assert_eq!(format_duration(1000), "1.0s");
    }

    #[test]
    fn format_size_human_readable() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1 KB");
        assert_eq!(format_size(290_000), "283 KB");
        assert_eq!(format_size(2_340_000), "2.2 MB");
    }

    #[test]
    fn format_size_boundary_at_one_kb() {
        assert_eq!(format_size(1023), "1023 B");
        assert_eq!(format_size(1024), "1 KB");
    }
}
