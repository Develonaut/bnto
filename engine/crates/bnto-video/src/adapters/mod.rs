// Adapter layer — abstraction boundary between processors and external tools.
//
// Each adapter trait defines a capability (download, transcode) independently
// of the underlying tool. Processors depend on the trait, not the concrete
// implementation. This pattern is standard across all engine crates that wrap
// external dependencies:
//
//   Processor → Adapter trait → Concrete impl (yt-dlp, ffmpeg, etc.)
//
// Swapping the underlying tool only changes the concrete impl file — the
// processor and its tests remain untouched.

/// Adapter contract version. Bump when the `VideoDownloader` trait changes.
pub const ADAPTER_VERSION: u32 = 1;

pub mod ytdlp;

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;

/// Result of a successful video download.
pub struct DownloadResult {
    /// The raw video file bytes.
    pub data: Vec<u8>,
    /// The output filename (e.g., "video.mp4").
    pub filename: String,
    /// MIME type of the downloaded video.
    pub mime_type: String,
}

/// Configuration for a video download request.
pub struct DownloadConfig<'a> {
    /// The URL to download from.
    pub url: &'a str,
    /// Desired output format (e.g., "mp4", "webm", "mp3").
    pub format: &'a str,
    /// Maximum video quality (e.g., "1080", "720", "best").
    pub quality: &'a str,
    /// Raw yt-dlp arguments, space-separated. Appended after built-in flags.
    pub extra_args: &'a str,
}

/// Abstraction boundary for video downloading.
///
/// Processors call this trait — never the underlying tool directly.
/// Today: `YtDlpAdapter`. Tomorrow: a pure Rust downloader, or a
/// different CLI tool. The processor doesn't care.
pub trait VideoDownloader: Send + Sync {
    /// Download a video from a URL, returning the file bytes.
    fn download(
        &self,
        config: &DownloadConfig,
        ctx: &dyn ProcessContext,
    ) -> Result<DownloadResult, BntoError>;
}
