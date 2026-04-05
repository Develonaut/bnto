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
        let format_sort;
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

        if config.quality != "best" {
            format_sort = format_sort_arg(config.quality);
            args.extend_from_slice(&["-S", &format_sort]);
        }

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

        // Derive filename from URL (last path segment, or fallback).
        let filename = url_to_filename(config.url, config.format);
        let mime_type = format_to_mime(config.format).to_string();

        Ok(DownloadResult {
            data,
            filename,
            mime_type,
        })
    }
}

/// Build the yt-dlp -S (sort) argument for quality capping.
/// e.g., "res:1080" limits to 1080p or below.
fn format_sort_arg(quality: &str) -> String {
    format!("res:{quality}")
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
    fn test_format_sort_arg() {
        assert_eq!(format_sort_arg("1080"), "res:1080");
        assert_eq!(format_sort_arg("720"), "res:720");
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
    fn test_format_to_mime() {
        assert_eq!(format_to_mime("mp4"), "video/mp4");
        assert_eq!(format_to_mime("webm"), "video/webm");
        assert_eq!(format_to_mime("mp3"), "audio/mpeg");
        assert_eq!(format_to_mime("unknown"), "video/mp4");
    }
}
