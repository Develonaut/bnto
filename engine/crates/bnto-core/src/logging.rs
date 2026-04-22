// Engine-level logging — trait + types for structured diagnostics.
//
// Defines the interface that all engine crates can import. No dependencies
// on `std::fs` or external crates — WASM-compatible. Concrete adapters
// (FileLogger for CLI, NoopLogger for browser) implement the trait.

use std::fmt;

/// Severity levels for log entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trace => write!(f, "TRACE"),
            Self::Debug => write!(f, "DEBUG"),
            Self::Info => write!(f, "INFO"),
            Self::Warn => write!(f, "WARN"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

/// A single log entry with optional timing data.
pub struct LogEntry {
    pub level: LogLevel,
    /// Subsystem that produced this entry ("tui", "engine", "image", etc.).
    pub target: &'static str,
    pub message: String,
    /// Optional elapsed time in microseconds for performance diagnostics.
    pub elapsed_us: Option<u64>,
}

/// Logging interface for engine diagnostics.
///
/// Follows the same adapter pattern as `ProcessContext`: browser gets
/// `NoopLogger`, CLI gets `FileLogger`, desktop gets whatever it needs.
pub trait Logger: Send + Sync {
    fn log(&self, entry: LogEntry);
    fn is_enabled(&self, level: LogLevel) -> bool;
    fn flush(&self);
}

/// No-op logger for browser (WASM) and test contexts.
pub struct NoopLogger;

impl Logger for NoopLogger {
    fn log(&self, _entry: LogEntry) {}
    fn is_enabled(&self, _level: LogLevel) -> bool {
        false
    }
    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_level_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }

    #[test]
    fn log_level_display() {
        assert_eq!(LogLevel::Trace.to_string(), "TRACE");
        assert_eq!(LogLevel::Info.to_string(), "INFO");
        assert_eq!(LogLevel::Error.to_string(), "ERROR");
    }

    #[test]
    fn noop_logger_accepts_entries() {
        let logger = NoopLogger;
        logger.log(LogEntry {
            level: LogLevel::Info,
            target: "test",
            message: "hello".into(),
            elapsed_us: None,
        });
        // Should not panic.
    }

    #[test]
    fn noop_logger_is_never_enabled() {
        let logger = NoopLogger;
        assert!(!logger.is_enabled(LogLevel::Trace));
        assert!(!logger.is_enabled(LogLevel::Error));
    }

    #[test]
    fn noop_logger_flush_is_safe() {
        let logger = NoopLogger;
        logger.flush();
    }

    #[test]
    fn log_entry_with_timing() {
        let entry = LogEntry {
            level: LogLevel::Debug,
            target: "tui",
            message: "frame render".into(),
            elapsed_us: Some(1234),
        };
        assert_eq!(entry.elapsed_us, Some(1234));
        assert_eq!(entry.target, "tui");
    }
}
