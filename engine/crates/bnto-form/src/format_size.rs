// Human-readable byte size formatting.

/// Format bytes as human-readable size (e.g., "283 KB", "2.2 MB").
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
    fn bytes_under_1kb() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(1023), "1023 B");
    }

    #[test]
    fn exact_1kb() {
        assert_eq!(format_size(1024), "1 KB");
    }

    #[test]
    fn kilobytes() {
        assert_eq!(format_size(290_000), "283 KB");
    }

    #[test]
    fn megabytes() {
        assert_eq!(format_size(2_340_000), "2.2 MB");
    }
}
