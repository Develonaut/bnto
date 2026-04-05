// yt-dlp adapter — concrete VideoDownloader implementation.
//
// Invokes `yt-dlp` via ProcessContext::run_command(), writing output to a
// temp file and reading the result back. This is the only file that knows
// about yt-dlp's CLI arguments — everything else talks through the
// VideoDownloader trait.

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;

use super::{DownloadConfig, DownloadResult, VideoDownloader};

/// VideoDownloader backed by the yt-dlp CLI tool.
pub struct YtDlpAdapter;

impl YtDlpAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for YtDlpAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoDownloader for YtDlpAdapter {
    fn download(
        &self,
        config: &DownloadConfig,
        ctx: &dyn ProcessContext,
    ) -> Result<DownloadResult, BntoError> {
        // Create a temp file for yt-dlp to write to.
        let suffix = format!(".{}", config.format);
        let output_path = ctx.temp_file(&suffix)?;
        let output_str = output_path.to_string_lossy().to_string();

        // Build and run the yt-dlp command.
        let mut args: Vec<&str> = vec![
            config.url,
            "-o",
            &output_str,
            "--no-playlist",
            "--no-warnings",
        ];

        if config.format != "best" {
            args.extend_from_slice(&["--merge-output-format", config.format]);
        }

        // Build the -S (format sort) argument for codec + quality preferences.
        // yt-dlp accepts comma-separated sort fields in a single -S flag.
        let format_sort = build_format_sort(config.format, config.quality);
        if !format_sort.is_empty() {
            args.extend_from_slice(&["-S", &format_sort]);
        }

        // Append user-provided extra args (last-wins for yt-dlp flags).
        let extra_parts: Vec<&str> = config.extra_args.split_whitespace().collect();
        args.extend_from_slice(&extra_parts);

        ctx.run_command("yt-dlp", &args)?;

        // Read the downloaded file.
        let data = std::fs::read(&output_path).map_err(|e| {
            BntoError::ProcessingFailed(format!(
                "Failed to read downloaded file at '{}': {e}",
                output_str
            ))
        })?;

        // Clean up temp file (best-effort).
        let _ = std::fs::remove_file(&output_path);

        if data.is_empty() {
            return Err(BntoError::ProcessingFailed(
                "yt-dlp produced an empty output file".to_string(),
            ));
        }

        // Get the video title from yt-dlp for a human-readable filename.
        // Falls back to URL-derived name or "video.{format}" if title extraction fails.
        let filename = fetch_title(config.url, ctx)
            .map(|title| format!("{}.{}", sanitize_filename(&title), config.format))
            .unwrap_or_else(|| url_to_filename(config.url, config.format));
        let mime_type = format_to_mime(config.format).to_string();

        Ok(DownloadResult {
            data,
            filename,
            mime_type,
        })
    }
}

/// Fetch the video title from yt-dlp without downloading.
/// Returns None if the title can't be extracted (network error, unsupported URL, etc.).
fn fetch_title(url: &str, ctx: &dyn ProcessContext) -> Option<String> {
    let output = ctx
        .run_command("yt-dlp", &["--print", "title", "--no-warnings", url])
        .ok()?;
    let title = String::from_utf8_lossy(&output).trim().to_string();
    if title.is_empty() { None } else { Some(title) }
}

/// Sanitize a video title for use as a filename.
/// Removes filesystem-unsafe characters and trims to a reasonable length.
fn sanitize_filename(title: &str) -> String {
    let sanitized: String = title
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            _ => c,
        })
        .collect();

    // Trim whitespace and dots from edges (Windows/macOS safety).
    let trimmed = sanitized.trim().trim_matches('.').trim();

    // Cap at 200 chars to avoid filesystem limits.
    if trimmed.len() > 200 {
        trimmed[..200].trim_end().to_string()
    } else {
        trimmed.to_string()
    }
}

/// Build the yt-dlp -S (sort) argument combining codec and quality preferences.
/// MP4/best formats prefer H.264+AAC for universal playback (QuickTime, mobile, web).
/// Without codec preference, yt-dlp picks VP9/AV1 which many players can't decode.
fn build_format_sort(format: &str, quality: &str) -> String {
    let mut parts = Vec::new();

    // Prefer H.264 video + AAC audio for MP4-family formats.
    if format == "mp4" || format == "best" {
        parts.push("vcodec:h264".to_string());
        parts.push("acodec:m4a".to_string());
    }

    // Cap resolution when quality isn't "best".
    if quality != "best" {
        parts.push(format!("res:{quality}"));
    }

    parts.join(",")
}

/// Extract a reasonable filename from a URL, falling back to "video.{format}".
/// Filters out non-media extensions (m3u8 manifests, DASH mpd, HTML pages)
/// so the output filename always reflects the actual content format.
fn url_to_filename(url: &str, format: &str) -> String {
    url.rsplit('/')
        .next()
        .filter(|seg| !seg.is_empty())
        .map(|seg| seg.split('?').next().unwrap_or(seg).to_string())
        .filter(|name| name.len() < 200 && has_media_extension(name))
        .unwrap_or_else(|| format!("video.{format}"))
}

/// Check if a filename has a recognized media file extension.
fn has_media_extension(name: &str) -> bool {
    name.rsplit('.')
        .next()
        .map(|ext| {
            matches!(
                ext,
                "mp4" | "webm" | "mkv" | "mp3" | "m4a" | "wav" | "flac" | "avi" | "mov" | "ogg"
            )
        })
        .unwrap_or(false)
}

/// Map format string to MIME type.
fn format_to_mime(format: &str) -> &str {
    match format {
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mkv" => "video/x-matroska",
        "mp3" => "audio/mpeg",
        "m4a" => "audio/mp4",
        "wav" => "audio/wav",
        "flac" => "audio/flac",
        _ => "video/mp4",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_format_sort_mp4_best() {
        assert_eq!(build_format_sort("mp4", "best"), "vcodec:h264,acodec:m4a");
    }

    #[test]
    fn test_build_format_sort_mp4_with_quality() {
        assert_eq!(
            build_format_sort("mp4", "720"),
            "vcodec:h264,acodec:m4a,res:720"
        );
    }

    #[test]
    fn test_build_format_sort_webm_best() {
        assert_eq!(build_format_sort("webm", "best"), "");
    }

    #[test]
    fn test_build_format_sort_webm_with_quality() {
        assert_eq!(build_format_sort("webm", "1080"), "res:1080");
    }

    #[test]
    fn test_build_format_sort_best_format() {
        assert_eq!(build_format_sort("best", "best"), "vcodec:h264,acodec:m4a");
    }

    #[test]
    fn test_url_to_filename_with_path() {
        assert_eq!(
            url_to_filename("https://example.com/video.mp4", "mp4"),
            "video.mp4"
        );
    }

    #[test]
    fn test_url_to_filename_with_query() {
        assert_eq!(
            url_to_filename("https://example.com/clip.mp4?token=abc", "mp4"),
            "clip.mp4"
        );
    }

    #[test]
    fn test_url_to_filename_fallback() {
        assert_eq!(
            url_to_filename("https://example.com/", "webm"),
            "video.webm"
        );
    }

    #[test]
    fn test_url_to_filename_no_extension_fallback() {
        assert_eq!(
            url_to_filename("https://example.com/watch", "mp4"),
            "video.mp4"
        );
    }

    #[test]
    fn test_url_to_filename_m3u8_falls_back() {
        // m3u8 is a manifest, not a media file — should fall back to video.{format}
        assert_eq!(
            url_to_filename("https://example.com/master.m3u8", "mp4"),
            "video.mp4"
        );
    }

    #[test]
    fn test_url_to_filename_m3u8_with_query_falls_back() {
        assert_eq!(
            url_to_filename(
                "https://cdn.example.com/live/stream.m3u8?token=abc123",
                "mp4"
            ),
            "video.mp4"
        );
    }

    #[test]
    fn test_url_to_filename_mpd_falls_back() {
        // DASH manifests should also fall back
        assert_eq!(
            url_to_filename("https://example.com/manifest.mpd", "webm"),
            "video.webm"
        );
    }

    #[test]
    fn test_sanitize_filename_basic() {
        assert_eq!(sanitize_filename("Me at the zoo"), "Me at the zoo");
    }

    #[test]
    fn test_sanitize_filename_special_chars() {
        assert_eq!(
            sanitize_filename("Video: The Best? Yes/No"),
            "Video_ The Best_ Yes_No"
        );
    }

    #[test]
    fn test_sanitize_filename_dots_trimmed() {
        assert_eq!(sanitize_filename("...title..."), "title");
    }

    #[test]
    fn test_sanitize_filename_long_title_truncated() {
        let long = "a".repeat(300);
        let result = sanitize_filename(&long);
        assert!(result.len() <= 200);
    }

    #[test]
    fn test_sanitize_filename_empty_after_sanitize() {
        assert_eq!(sanitize_filename("..."), "");
    }

    #[test]
    fn test_format_to_mime() {
        assert_eq!(format_to_mime("mp4"), "video/mp4");
        assert_eq!(format_to_mime("webm"), "video/webm");
        assert_eq!(format_to_mime("mp3"), "audio/mpeg");
        assert_eq!(format_to_mime("unknown"), "video/mp4");
    }

    // --- Extra args splitting (unit) ---

    #[test]
    fn test_extra_args_empty_is_noop() {
        let parts: Vec<&str> = "".split_whitespace().collect();
        assert!(parts.is_empty());
    }

    #[test]
    fn test_extra_args_single_flag() {
        let parts: Vec<&str> = "--verbose".split_whitespace().collect();
        assert_eq!(parts, vec!["--verbose"]);
    }

    #[test]
    fn test_extra_args_multiple_flags() {
        let parts: Vec<&str> = "--geo-bypass --proxy http://proxy:8080"
            .split_whitespace()
            .collect();
        assert_eq!(parts, vec!["--geo-bypass", "--proxy", "http://proxy:8080"]);
    }

    #[test]
    fn test_extra_args_excess_whitespace() {
        let parts: Vec<&str> = "  --verbose   --no-warnings  ".split_whitespace().collect();
        assert_eq!(parts, vec!["--verbose", "--no-warnings"]);
    }

    // --- Extra args integration (SpyContext captures the full command line) ---

    use std::path::{Path, PathBuf};
    use std::sync::Mutex;

    /// ProcessContext that records the args passed to run_command.
    struct SpyContext {
        captured_args: Mutex<Vec<Vec<String>>>,
    }

    impl SpyContext {
        fn new() -> Self {
            Self {
                captured_args: Mutex::new(Vec::new()),
            }
        }

        /// Return args from the first run_command call (the download invocation).
        fn first_call_args(&self) -> Vec<String> {
            self.captured_args.lock().unwrap()[0].clone()
        }
    }

    impl ProcessContext for SpyContext {
        fn run_command(&self, _cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
            self.captured_args
                .lock()
                .unwrap()
                .push(args.iter().map(|s| s.to_string()).collect());
            // Return non-empty output to simulate yt-dlp --print title
            Ok(b"Spy Title".to_vec())
        }
        fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
            // Create a real temp file with fake video data so fs::read succeeds.
            let path = std::env::temp_dir().join(format!("spy_test{suffix}"));
            std::fs::write(&path, b"fake video data").unwrap();
            Ok(path)
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Err(BntoError::ProcessingFailed("no work dir".to_string()))
        }
    }

    #[test]
    fn test_extra_args_appended_to_command() {
        let adapter = YtDlpAdapter::new();
        let spy = SpyContext::new();
        let config = DownloadConfig {
            url: "https://example.com/video",
            format: "mp4",
            quality: "best",
            extra_args: "--verbose --geo-bypass",
        };

        let _ = adapter.download(&config, &spy);

        let args = spy.first_call_args();
        // Extra args should appear at the tail, after all built-in flags.
        assert!(args.contains(&"--verbose".to_string()), "args: {args:?}");
        assert!(args.contains(&"--geo-bypass".to_string()), "args: {args:?}");
        // They must be AFTER the built-in -S flag.
        let s_pos = args.iter().position(|a| a == "-S").unwrap();
        let verbose_pos = args.iter().position(|a| a == "--verbose").unwrap();
        assert!(
            verbose_pos > s_pos,
            "extra args should be after built-in -S: {args:?}"
        );
    }

    #[test]
    fn test_extra_args_empty_no_extra_tokens() {
        let adapter = YtDlpAdapter::new();
        let spy = SpyContext::new();
        let config = DownloadConfig {
            url: "https://example.com/video",
            format: "mp4",
            quality: "best",
            extra_args: "",
        };

        let _ = adapter.download(&config, &spy);

        let args = spy.first_call_args();
        // With empty extra_args, the last arg should be the format sort value,
        // not any extra tokens.
        let last = args.last().unwrap();
        assert_eq!(last, "vcodec:h264,acodec:m4a", "args: {args:?}");
    }

    #[test]
    fn test_extra_args_override_format_sort() {
        // User passes their own -S flag — it appears after ours (last-wins).
        let adapter = YtDlpAdapter::new();
        let spy = SpyContext::new();
        let config = DownloadConfig {
            url: "https://example.com/video",
            format: "mp4",
            quality: "best",
            extra_args: "-S vcodec:vp9",
        };

        let _ = adapter.download(&config, &spy);

        let args = spy.first_call_args();
        // There should be two -S flags: ours and the user's.
        let s_positions: Vec<usize> = args
            .iter()
            .enumerate()
            .filter(|(_, a)| *a == "-S")
            .map(|(i, _)| i)
            .collect();
        assert_eq!(s_positions.len(), 2, "expected two -S flags: {args:?}");
        // User's -S should come last (higher index).
        let user_s_val = &args[s_positions[1] + 1];
        assert_eq!(user_s_val, "vcodec:vp9", "args: {args:?}");
    }

    #[test]
    fn test_extra_args_with_tabs_and_mixed_whitespace() {
        let adapter = YtDlpAdapter::new();
        let spy = SpyContext::new();
        let config = DownloadConfig {
            url: "https://example.com/video",
            format: "webm",
            quality: "best",
            extra_args: "--verbose\t\t--no-check-certificates   --socket-timeout\t30",
        };

        let _ = adapter.download(&config, &spy);

        let args = spy.first_call_args();
        assert!(args.contains(&"--verbose".to_string()), "args: {args:?}");
        assert!(
            args.contains(&"--no-check-certificates".to_string()),
            "args: {args:?}"
        );
        assert!(
            args.contains(&"--socket-timeout".to_string()),
            "args: {args:?}"
        );
        assert!(args.contains(&"30".to_string()), "args: {args:?}");
    }
}
