// Anonymize exact values into telemetry-safe ranges.

/// Bucket a duration in milliseconds into a human-readable range.
pub fn bucket_duration(ms: u64) -> &'static str {
    match ms {
        0..1000 => "<1s",
        1000..5000 => "1-5s",
        5000..30_000 => "5-30s",
        30_000..120_000 => "30s-2m",
        _ => ">2m",
    }
}

/// Bucket a file size in bytes into a human-readable range.
pub fn bucket_file_size(bytes: u64) -> &'static str {
    match bytes {
        0..100_000 => "<100KB",
        100_000..1_000_000 => "100KB-1MB",
        1_000_000..10_000_000 => "1-10MB",
        10_000_000..100_000_000 => "10-100MB",
        _ => ">100MB",
    }
}

/// Bucket a file count into a range.
pub fn bucket_file_count(count: usize) -> &'static str {
    match count {
        0..=1 => "1",
        2..=5 => "2-5",
        6..=20 => "6-20",
        _ => ">20",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_buckets() {
        assert_eq!(bucket_duration(0), "<1s");
        assert_eq!(bucket_duration(500), "<1s");
        assert_eq!(bucket_duration(999), "<1s");
        assert_eq!(bucket_duration(1000), "1-5s");
        assert_eq!(bucket_duration(3000), "1-5s");
        assert_eq!(bucket_duration(5000), "5-30s");
        assert_eq!(bucket_duration(29_999), "5-30s");
        assert_eq!(bucket_duration(30_000), "30s-2m");
        assert_eq!(bucket_duration(120_000), ">2m");
        assert_eq!(bucket_duration(999_999), ">2m");
    }

    #[test]
    fn file_size_buckets() {
        assert_eq!(bucket_file_size(0), "<100KB");
        assert_eq!(bucket_file_size(50_000), "<100KB");
        assert_eq!(bucket_file_size(99_999), "<100KB");
        assert_eq!(bucket_file_size(100_000), "100KB-1MB");
        assert_eq!(bucket_file_size(999_999), "100KB-1MB");
        assert_eq!(bucket_file_size(1_000_000), "1-10MB");
        assert_eq!(bucket_file_size(5_000_000), "1-10MB");
        assert_eq!(bucket_file_size(10_000_000), "10-100MB");
        assert_eq!(bucket_file_size(100_000_000), ">100MB");
        assert_eq!(bucket_file_size(500_000_000), ">100MB");
    }

    #[test]
    fn file_count_buckets() {
        assert_eq!(bucket_file_count(0), "1");
        assert_eq!(bucket_file_count(1), "1");
        assert_eq!(bucket_file_count(2), "2-5");
        assert_eq!(bucket_file_count(5), "2-5");
        assert_eq!(bucket_file_count(6), "6-20");
        assert_eq!(bucket_file_count(20), "6-20");
        assert_eq!(bucket_file_count(21), ">20");
        assert_eq!(bucket_file_count(100), ">20");
    }
}
