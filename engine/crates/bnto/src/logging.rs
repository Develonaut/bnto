// File-based logger for CLI/TUI sessions.
//
// Writes timestamped entries to `{state}/logs/session-{timestamp}.log`.
// One log file per session. Buffered writes behind a Mutex for thread safety.

use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::SystemTime;

use bnto_core::logging::{LogEntry, LogLevel, Logger};

/// File-backed logger that writes to a per-session log file.
pub struct FileLogger {
    writer: Mutex<BufWriter<File>>,
    min_level: LogLevel,
    path: PathBuf,
}

impl FileLogger {
    /// Create a new file logger under the given logs directory.
    ///
    /// Creates a file named `session-{timestamp}.log` and returns the
    /// logger. Returns `None` if the file cannot be created.
    pub fn new(logs_dir: &std::path::Path, min_level: LogLevel) -> Option<Self> {
        std::fs::create_dir_all(logs_dir).ok()?;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let filename = format!("session-{timestamp}.log");
        let path = logs_dir.join(filename);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .ok()?;

        Some(Self {
            writer: Mutex::new(BufWriter::new(file)),
            min_level,
            path,
        })
    }

    /// Path to the active log file.
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl Logger for FileLogger {
    fn log(&self, entry: LogEntry) {
        if !self.is_enabled(entry.level) {
            return;
        }

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        let secs = now.as_secs();
        let millis = now.subsec_millis();

        let timing = match entry.elapsed_us {
            Some(us) if us >= 1000 => format!(" ({}ms)", us / 1000),
            Some(us) => format!(" ({us}µs)"),
            None => String::new(),
        };

        let line = format!(
            "[{secs}.{millis:03}] [{level}] [{target}] {msg}{timing}\n",
            level = entry.level,
            target = entry.target,
            msg = entry.message,
        );

        if let Ok(mut w) = self.writer.lock() {
            let _ = w.write_all(line.as_bytes());
        }
    }

    fn is_enabled(&self, level: LogLevel) -> bool {
        level >= self.min_level
    }

    fn flush(&self) {
        if let Ok(mut w) = self.writer.lock() {
            let _ = w.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_session_log_file() {
        let tmp = tempfile::tempdir().unwrap();
        let logger = FileLogger::new(tmp.path(), LogLevel::Trace).expect("should create logger");
        assert!(logger.path().exists());
        assert!(logger.path().to_string_lossy().contains("session-"));
    }

    #[test]
    fn writes_and_flushes_entries() {
        let tmp = tempfile::tempdir().unwrap();
        let logger = FileLogger::new(tmp.path(), LogLevel::Trace).unwrap();
        logger.log(LogEntry {
            level: LogLevel::Info,
            target: "test",
            message: "hello world".into(),
            elapsed_us: None,
        });
        logger.flush();

        let content = std::fs::read_to_string(logger.path()).unwrap();
        assert!(content.contains("[INFO]"));
        assert!(content.contains("[test]"));
        assert!(content.contains("hello world"));
    }

    #[test]
    fn includes_timing_in_microseconds() {
        let tmp = tempfile::tempdir().unwrap();
        let logger = FileLogger::new(tmp.path(), LogLevel::Trace).unwrap();
        logger.log(LogEntry {
            level: LogLevel::Debug,
            target: "tui",
            message: "fast op".into(),
            elapsed_us: Some(84),
        });
        logger.flush();

        let content = std::fs::read_to_string(logger.path()).unwrap();
        assert!(content.contains("(84µs)"));
    }

    #[test]
    fn includes_timing_in_milliseconds() {
        let tmp = tempfile::tempdir().unwrap();
        let logger = FileLogger::new(tmp.path(), LogLevel::Trace).unwrap();
        logger.log(LogEntry {
            level: LogLevel::Debug,
            target: "tui",
            message: "slow op".into(),
            elapsed_us: Some(16_500),
        });
        logger.flush();

        let content = std::fs::read_to_string(logger.path()).unwrap();
        assert!(content.contains("(16ms)"));
    }

    #[test]
    fn respects_min_level() {
        let tmp = tempfile::tempdir().unwrap();
        let logger = FileLogger::new(tmp.path(), LogLevel::Warn).unwrap();
        assert!(!logger.is_enabled(LogLevel::Debug));
        assert!(logger.is_enabled(LogLevel::Warn));
        assert!(logger.is_enabled(LogLevel::Error));

        logger.log(LogEntry {
            level: LogLevel::Debug,
            target: "test",
            message: "should be filtered".into(),
            elapsed_us: None,
        });
        logger.flush();

        let content = std::fs::read_to_string(logger.path()).unwrap();
        assert!(!content.contains("should be filtered"));
    }

    #[test]
    fn returns_none_for_nonexistent_dir() {
        let bad_path = std::path::PathBuf::from("/nonexistent/deep/path/logs");
        assert!(FileLogger::new(&bad_path, LogLevel::Trace).is_none());
    }
}
