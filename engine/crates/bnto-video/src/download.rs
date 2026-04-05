// VideoDownload processor — download video from URLs via adapter pattern.
//
// This is a "source" node: input is a URL parameter, not file uploads.
// The processor delegates to a `VideoDownloader` adapter (injected at
// construction) — today that's `YtDlpAdapter`, but the processor is
// decoupled from any specific CLI tool.

use bnto_core::context::ProcessContext;
use bnto_core::errors::BntoError;
use bnto_core::metadata::{
    Constraints, Dependency, InputCardinality, NodeCategory, NodeMetadata, ParameterDef,
    ParameterType,
};
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;

use crate::adapters::{DownloadConfig, VideoDownloader};

const DEFAULT_FORMAT: &str = "mp4";
const DEFAULT_QUALITY: &str = "best";

/// Video download processor. Wraps a `VideoDownloader` adapter injected
/// at construction — production uses `YtDlpAdapter`, tests use a mock.
pub struct VideoDownload {
    adapter: Box<dyn VideoDownloader>,
}

impl VideoDownload {
    pub fn new(adapter: Box<dyn VideoDownloader>) -> Self {
        Self { adapter }
    }

    /// Convenience constructor for production use with yt-dlp.
    pub fn with_ytdlp() -> Self {
        Self::new(Box::new(crate::adapters::ytdlp::YtDlpAdapter::new()))
    }
}

impl NodeProcessor for VideoDownload {
    fn name(&self) -> &str {
        "video-download"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "video-download".to_string(),
            name: "Download Video".to_string(),
            description: "Download video from URLs using yt-dlp.".to_string(),
            category: NodeCategory::Video,
            accepts: vec![],
            platforms: vec![
                "cli".to_string(),
                "server".to_string(),
                "desktop".to_string(),
            ],
            parameters: vec![url_param(), format_param(), quality_param(), args_param()],
            input_cardinality: InputCardinality::Source,
            requires: vec![
                Dependency {
                    binary: "yt-dlp".to_string(),
                    version: String::new(),
                    install_hint: "brew install yt-dlp".to_string(),
                    homepage: "https://github.com/yt-dlp/yt-dlp".to_string(),
                },
                Dependency {
                    binary: "ffmpeg".to_string(),
                    version: String::new(),
                    install_hint: "brew install ffmpeg".to_string(),
                    homepage: "https://ffmpeg.org".to_string(),
                },
            ],
        }
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        let mut errors = Vec::new();

        let url = params.get("url").and_then(|v| v.as_str()).unwrap_or("");

        if url.is_empty() {
            errors.push("url is required".to_string());
        } else if !url.starts_with("http://") && !url.starts_with("https://") {
            errors.push("url must start with http:// or https://".to_string());
        }

        errors
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let url = input
            .params
            .get("url")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if url.is_empty() {
            return Err(BntoError::InvalidInput(
                "url parameter is required".to_string(),
            ));
        }

        let format = input
            .params
            .get("format")
            .and_then(|v| v.as_str())
            .unwrap_or(DEFAULT_FORMAT);

        let quality = input
            .params
            .get("quality")
            .and_then(|v| v.as_str())
            .unwrap_or(DEFAULT_QUALITY);

        let extra_args = input
            .params
            .get("args")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        progress.report(10, "Starting download...");

        let config = DownloadConfig {
            url,
            format,
            quality,
            extra_args,
        };

        let result = self.adapter.download(&config, ctx)?;

        progress.report(90, "Download complete");

        let mut metadata = serde_json::Map::new();
        metadata.insert("filename".to_string(), serde_json::json!(result.filename));
        metadata.insert("format".to_string(), serde_json::json!(format));
        metadata.insert(
            "sizeBytes".to_string(),
            serde_json::json!(result.data.len()),
        );

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: result.data,
                filename: result.filename,
                mime_type: result.mime_type,
            }],
            metadata,
        })
    }
}

// --- Parameter Definitions ---

fn url_param() -> ParameterDef {
    ParameterDef {
        name: "url".to_string(),
        label: "URL".to_string(),
        description: "Video URL to download (YouTube, m3u8/HLS, or direct link).".to_string(),
        param_type: ParameterType::String,
        default: None,
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        placeholder: Some("https://youtube.com/watch?v=...".to_string()),
        ..Default::default()
    }
}

fn format_param() -> ParameterDef {
    ParameterDef {
        name: "format".to_string(),
        label: "Format".to_string(),
        description: "Output format for the downloaded video or audio.".to_string(),
        param_type: ParameterType::Enum {
            options: vec![
                "mp4".to_string(),
                "webm".to_string(),
                "mkv".to_string(),
                "mp3".to_string(),
                "m4a".to_string(),
                "wav".to_string(),
                "flac".to_string(),
            ],
        },
        default: Some(serde_json::json!("mp4")),
        ..Default::default()
    }
}

fn quality_param() -> ParameterDef {
    ParameterDef {
        name: "quality".to_string(),
        label: "Quality".to_string(),
        description: "Maximum video quality. 'best' downloads the highest available.".to_string(),
        param_type: ParameterType::Enum {
            options: vec![
                "best".to_string(),
                "1080".to_string(),
                "720".to_string(),
                "480".to_string(),
                "360".to_string(),
            ],
        },
        default: Some(serde_json::json!("best")),
        ..Default::default()
    }
}

fn args_param() -> ParameterDef {
    ParameterDef {
        name: "args".to_string(),
        label: "Extra Arguments".to_string(),
        description: "Raw yt-dlp arguments, space-separated. Appended after built-in flags."
            .to_string(),
        param_type: ParameterType::String,
        default: Some(serde_json::json!("")),
        placeholder: Some("--cookies-from-browser chrome".to_string()),
        surfaceable: false,
        ..Default::default()
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::DownloadResult;
    use std::path::{Path, PathBuf};
    use std::sync::Mutex;

    /// Mock adapter that returns canned video data without invoking yt-dlp.
    struct MockVideoDownloader {
        response: Result<DownloadResult, BntoError>,
    }

    impl MockVideoDownloader {
        fn ok(data: Vec<u8>, filename: &str, mime: &str) -> Self {
            Self {
                response: Ok(DownloadResult {
                    data,
                    filename: filename.to_string(),
                    mime_type: mime.to_string(),
                }),
            }
        }

        fn err(msg: &str) -> Self {
            Self {
                response: Err(BntoError::ProcessingFailed(msg.to_string())),
            }
        }
    }

    impl VideoDownloader for MockVideoDownloader {
        fn download(
            &self,
            _config: &DownloadConfig,
            _ctx: &dyn ProcessContext,
        ) -> Result<DownloadResult, BntoError> {
            match &self.response {
                Ok(r) => Ok(DownloadResult {
                    data: r.data.clone(),
                    filename: r.filename.clone(),
                    mime_type: r.mime_type.clone(),
                }),
                Err(e) => Err(BntoError::ProcessingFailed(e.to_string())),
            }
        }
    }

    use std::sync::Arc;

    /// Shared state for capturing what the processor passes to the adapter.
    type CapturedArgs = Arc<Mutex<Option<String>>>;

    /// Capturing mock that records the DownloadConfig.extra_args it received.
    struct CapturingDownloader {
        captured: CapturedArgs,
    }

    impl CapturingDownloader {
        fn new(captured: CapturedArgs) -> Self {
            Self { captured }
        }
    }

    impl VideoDownloader for CapturingDownloader {
        fn download(
            &self,
            config: &DownloadConfig,
            _ctx: &dyn ProcessContext,
        ) -> Result<DownloadResult, BntoError> {
            *self.captured.lock().unwrap() = Some(config.extra_args.to_string());
            Ok(DownloadResult {
                data: vec![1],
                filename: "v.mp4".to_string(),
                mime_type: "video/mp4".to_string(),
            })
        }
    }

    /// Create a processor with a capturing adapter, returning the shared state handle.
    fn make_capturing_processor() -> (VideoDownload, CapturedArgs) {
        let captured: CapturedArgs = Arc::new(Mutex::new(None));
        let adapter = CapturingDownloader::new(Arc::clone(&captured));
        (VideoDownload::new(Box::new(adapter)), captured)
    }

    /// Mock ProcessContext for unit tests — no real system access.
    struct MockContext;

    impl ProcessContext for MockContext {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            Ok(Vec::new())
        }
        fn temp_file(&self, _suffix: &str) -> Result<PathBuf, BntoError> {
            Ok(PathBuf::from("/tmp/mock.mp4"))
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Err(BntoError::ProcessingFailed("no work dir".to_string()))
        }
    }

    fn make_processor(mock: MockVideoDownloader) -> VideoDownload {
        VideoDownload::new(Box::new(mock))
    }

    fn make_input(url: &str, format: &str, quality: &str) -> NodeInput {
        let mut params = serde_json::Map::new();
        params.insert("url".to_string(), serde_json::json!(url));
        params.insert("format".to_string(), serde_json::json!(format));
        params.insert("quality".to_string(), serde_json::json!(quality));
        NodeInput {
            data: vec![],
            filename: String::new(),
            mime_type: None,
            params,
        }
    }

    #[test]
    fn test_name() {
        let p = make_processor(MockVideoDownloader::ok(vec![1], "v.mp4", "video/mp4"));
        assert_eq!(p.name(), "video-download");
    }

    #[test]
    fn test_metadata_node_type() {
        let p = make_processor(MockVideoDownloader::ok(vec![], "v.mp4", "video/mp4"));
        let m = p.metadata();
        assert_eq!(m.node_type, "video-download");
        assert_eq!(m.category, NodeCategory::Video);
        assert!(!m.platforms.contains(&"browser".to_string()));
        assert!(m.platforms.contains(&"cli".to_string()));
        assert_eq!(m.requires.len(), 2);
        assert_eq!(m.requires[0].binary, "yt-dlp");
        assert_eq!(m.requires[1].binary, "ffmpeg");
    }

    #[test]
    fn test_metadata_has_four_params() {
        let p = make_processor(MockVideoDownloader::ok(vec![], "v.mp4", "video/mp4"));
        let m = p.metadata();
        assert_eq!(m.parameters.len(), 4);
        let names: Vec<&str> = m.parameters.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, vec!["url", "format", "quality", "args"]);
    }

    #[test]
    fn test_args_param_not_surfaceable() {
        let p = make_processor(MockVideoDownloader::ok(vec![], "v.mp4", "video/mp4"));
        let m = p.metadata();
        let args = m.parameters.iter().find(|p| p.name == "args").unwrap();
        assert!(!args.surfaceable);
    }

    #[test]
    fn test_happy_path_download() {
        let video_data = vec![0x00, 0x00, 0x00, 0x1c, 0x66, 0x74, 0x79, 0x70]; // fake mp4 header
        let mock = MockVideoDownloader::ok(video_data.clone(), "video.mp4", "video/mp4");
        let p = make_processor(mock);
        let progress = ProgressReporter::new_noop();
        let input = make_input("https://example.com/video", "mp4", "best");

        let output = p.process(input, &progress, &MockContext).unwrap();

        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].data, video_data);
        assert_eq!(output.files[0].filename, "video.mp4");
        assert_eq!(output.files[0].mime_type, "video/mp4");
    }

    #[test]
    fn test_missing_url_error() {
        let mock = MockVideoDownloader::ok(vec![1], "v.mp4", "video/mp4");
        let p = make_processor(mock);
        let progress = ProgressReporter::new_noop();

        let input = NodeInput {
            data: vec![],
            filename: String::new(),
            mime_type: None,
            params: serde_json::Map::new(),
        };

        let result = p.process(input, &progress, &MockContext);
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("url"));
    }

    #[test]
    fn test_adapter_error_propagates() {
        let mock = MockVideoDownloader::err("yt-dlp not found");
        let p = make_processor(mock);
        let progress = ProgressReporter::new_noop();
        let input = make_input("https://example.com/video", "mp4", "best");

        let result = p.process(input, &progress, &MockContext);
        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("yt-dlp not found")
        );
    }

    #[test]
    fn test_output_metadata() {
        let data = vec![0u8; 1024];
        let mock = MockVideoDownloader::ok(data, "clip.webm", "video/webm");
        let p = make_processor(mock);
        let progress = ProgressReporter::new_noop();
        let input = make_input("https://example.com/clip", "webm", "720");

        let output = p.process(input, &progress, &MockContext).unwrap();

        assert_eq!(output.metadata["filename"], "clip.webm");
        assert_eq!(output.metadata["format"], "webm");
        assert_eq!(output.metadata["sizeBytes"], 1024);
    }

    #[test]
    fn test_default_params_used() {
        let mock = MockVideoDownloader::ok(vec![1], "v.mp4", "video/mp4");
        let p = make_processor(mock);
        let progress = ProgressReporter::new_noop();

        // Only provide url — format and quality should use defaults.
        let mut params = serde_json::Map::new();
        params.insert(
            "url".to_string(),
            serde_json::json!("https://example.com/v"),
        );
        let input = NodeInput {
            data: vec![],
            filename: String::new(),
            mime_type: None,
            params,
        };

        let output = p.process(input, &progress, &MockContext).unwrap();
        assert_eq!(output.files.len(), 1);
    }

    #[test]
    fn test_validate_missing_url() {
        let mock = MockVideoDownloader::ok(vec![1], "v.mp4", "video/mp4");
        let p = make_processor(mock);
        let params = serde_json::Map::new();
        let errors = p.validate(&params);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("required"));
    }

    #[test]
    fn test_validate_invalid_url_scheme() {
        let mock = MockVideoDownloader::ok(vec![1], "v.mp4", "video/mp4");
        let p = make_processor(mock);
        let mut params = serde_json::Map::new();
        params.insert("url".to_string(), serde_json::json!("ftp://bad.com/video"));
        let errors = p.validate(&params);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("http"));
    }

    #[test]
    fn test_validate_valid_url() {
        let mock = MockVideoDownloader::ok(vec![1], "v.mp4", "video/mp4");
        let p = make_processor(mock);
        let mut params = serde_json::Map::new();
        params.insert(
            "url".to_string(),
            serde_json::json!("https://youtube.com/watch?v=123"),
        );
        let errors = p.validate(&params);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_m3u8_url() {
        let mock = MockVideoDownloader::ok(vec![1], "v.mp4", "video/mp4");
        let p = make_processor(mock);
        let mut params = serde_json::Map::new();
        params.insert(
            "url".to_string(),
            serde_json::json!("https://example.com/stream/master.m3u8"),
        );
        let errors = p.validate(&params);
        assert!(errors.is_empty(), "m3u8 URLs should pass validation");
    }

    #[test]
    fn test_m3u8_url_download() {
        let video_data = vec![0x00, 0x00, 0x00, 0x1c, 0x66, 0x74, 0x79, 0x70];
        let mock = MockVideoDownloader::ok(video_data.clone(), "video.mp4", "video/mp4");
        let p = make_processor(mock);
        let progress = ProgressReporter::new_noop();
        let input = make_input("https://example.com/live/master.m3u8", "mp4", "best");

        let output = p.process(input, &progress, &MockContext).unwrap();

        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].data, video_data);
        assert_eq!(output.files[0].mime_type, "video/mp4");
    }

    // --- Extra args (capturing mock verifies DownloadConfig.extra_args) ---

    fn make_input_with_args(url: &str, format: &str, quality: &str, args: &str) -> NodeInput {
        let mut params = serde_json::Map::new();
        params.insert("url".to_string(), serde_json::json!(url));
        params.insert("format".to_string(), serde_json::json!(format));
        params.insert("quality".to_string(), serde_json::json!(quality));
        params.insert("args".to_string(), serde_json::json!(args));
        NodeInput {
            data: vec![],
            filename: String::new(),
            mime_type: None,
            params,
        }
    }

    #[test]
    fn test_args_passed_to_adapter() {
        let (processor, captured) = make_capturing_processor();
        let progress = ProgressReporter::new_noop();
        let input = make_input_with_args(
            "https://example.com/v",
            "mp4",
            "best",
            "--verbose --geo-bypass",
        );

        processor.process(input, &progress, &MockContext).unwrap();

        let value = captured.lock().unwrap().clone().unwrap();
        assert_eq!(value, "--verbose --geo-bypass");
    }

    #[test]
    fn test_args_defaults_to_empty_when_absent() {
        let (processor, captured) = make_capturing_processor();
        let progress = ProgressReporter::new_noop();

        // Only url — no args param provided.
        let mut params = serde_json::Map::new();
        params.insert("url".to_string(), serde_json::json!("https://example.com/v"));
        let input = NodeInput {
            data: vec![],
            filename: String::new(),
            mime_type: None,
            params,
        };

        processor.process(input, &progress, &MockContext).unwrap();

        let value = captured.lock().unwrap().clone().unwrap();
        assert_eq!(value, "", "args should default to empty string");
    }

    #[test]
    fn test_args_whitespace_only_passed_verbatim() {
        let (processor, captured) = make_capturing_processor();
        let progress = ProgressReporter::new_noop();
        let input = make_input_with_args("https://example.com/v", "mp4", "best", "   ");

        processor.process(input, &progress, &MockContext).unwrap();

        let value = captured.lock().unwrap().clone().unwrap();
        assert_eq!(value, "   ", "processor passes raw string to adapter");
    }

    #[test]
    fn test_args_complex_flags() {
        let (processor, captured) = make_capturing_processor();
        let progress = ProgressReporter::new_noop();
        let input = make_input_with_args(
            "https://example.com/v",
            "mp4",
            "best",
            "--cookies-from-browser chrome --proxy http://proxy:8080 -f bestvideo+bestaudio",
        );

        processor.process(input, &progress, &MockContext).unwrap();

        let value = captured.lock().unwrap().clone().unwrap();
        assert_eq!(
            value,
            "--cookies-from-browser chrome --proxy http://proxy:8080 -f bestvideo+bestaudio"
        );
    }
}
