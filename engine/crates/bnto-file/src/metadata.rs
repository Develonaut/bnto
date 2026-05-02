// file-metadata — Extract and expose file metadata as pipeline output.
//
// Pass-through enrichment node: file data flows unchanged, but the output
// metadata map gains fields like file_size, extension, mime_type, and
// an optional hash (SHA-256). Works in both browser and CLI — no
// filesystem access needed since all info comes from the input bytes.

use bnto_core::metadata::{NodeCategory, NodeMetadata, ParameterDef, ParameterType};
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};

/// File-metadata processor — enrich pipeline output with file info.
pub struct FileMetadata;

impl FileMetadata {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileMetadata {
    fn name(&self) -> &str {
        "file-metadata"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-metadata".to_string(),
            name: "File Metadata".to_string(),
            description: "Extract file metadata (size, extension, MIME type, hash) and attach it to the pipeline output.".to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["browser".to_string(), "cli".to_string(), "desktop".to_string()],
            parameters: build_metadata_params(),
            input_cardinality: Default::default(),
            requires: vec![],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(0, "Extracting metadata...");

        let include_hash = input
            .params
            .get("include_hash")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let file_size = input.data.len();
        let extension = extract_extension(&input.filename);
        let mime_type = guess_mime_type(&input.filename);

        progress.report(50, "Building metadata...");

        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "filename".to_string(),
            serde_json::Value::String(input.filename.clone()),
        );
        metadata.insert("file_size".to_string(), serde_json::json!(file_size));
        metadata.insert(
            "extension".to_string(),
            serde_json::Value::String(extension),
        );
        metadata.insert(
            "mime_type".to_string(),
            serde_json::Value::String(mime_type.clone()),
        );

        if include_hash {
            let hash = sha256_hex(&input.data);
            metadata.insert("sha256".to_string(), serde_json::Value::String(hash));
        }

        progress.report(100, "Done");

        Ok(NodeOutput {
            files: vec![OutputFile {
                data: FileData::Bytes(input.data),
                filename: input.filename,
                mime_type: input.mime_type.unwrap_or(mime_type),
                metadata: serde_json::Map::new(),
            }],
            metadata,
        })
    }
}

// --- Parameter Definitions ---

fn build_metadata_params() -> Vec<ParameterDef> {
    vec![ParameterDef {
        name: "include_hash".to_string(),
        label: "Include Hash".to_string(),
        description: "Compute and include a SHA-256 hash of the file content.".to_string(),
        param_type: ParameterType::Boolean,
        default: Some(serde_json::json!(false)),
        ..Default::default()
    }]
}

// --- Helpers ---

/// Extract the file extension (lowercase, without dot).
fn extract_extension(filename: &str) -> String {
    filename
        .rsplit('.')
        .next()
        .filter(|_| filename.contains('.') && !filename.starts_with('.'))
        .unwrap_or("")
        .to_lowercase()
}

/// Guess MIME type from file extension.
fn guess_mime_type(filename: &str) -> String {
    let ext = extract_extension(filename);
    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "webp" => "image/webp".to_string(),
        "svg" => "image/svg+xml".to_string(),
        "gif" => "image/gif".to_string(),
        "csv" => "text/csv".to_string(),
        "json" => "application/json".to_string(),
        "txt" | "md" => "text/plain".to_string(),
        "pdf" => "application/pdf".to_string(),
        "html" | "htm" => "text/html".to_string(),
        "xml" => "application/xml".to_string(),
        "zip" => "application/zip".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

/// Compute SHA-256 hash of data, returned as lowercase hex string.
///
/// Uses a pure-Rust implementation to stay WASM-compatible — no OpenSSL.
fn sha256_hex(data: &[u8]) -> String {
    use std::fmt::Write;

    // SHA-256 constants: first 32 bits of the fractional parts of the
    // cube roots of the first 64 primes.
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    // Initial hash values: first 32 bits of the fractional parts of the
    // square roots of the first 8 primes.
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    // Pre-processing: pad the message.
    let bit_len = (data.len() as u64) * 8;
    let mut padded = data.to_vec();
    padded.push(0x80);
    while (padded.len() % 64) != 56 {
        padded.push(0x00);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    // Process each 512-bit (64-byte) block.
    for chunk in padded.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut hex = String::with_capacity(64);
    for word in &h {
        write!(hex, "{word:08x}").unwrap();
    }
    hex
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::context::NoopContext;
    use bnto_core::progress::ProgressReporter;

    fn make_input(
        data: &[u8],
        filename: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data: data.to_vec(),
            filename: filename.to_string(),
            mime_type: None,
            params,
        }
    }

    fn params(pairs: &[(&str, serde_json::Value)]) -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), v.clone());
        }
        map
    }

    // --- Trait basics ---

    #[test]
    fn test_name_returns_correct_key() {
        assert_eq!(FileMetadata::new().name(), "file-metadata");
    }

    #[test]
    fn test_metadata_is_browser_compatible() {
        let meta = FileMetadata::new().metadata();
        assert_eq!(meta.category, NodeCategory::File);
        assert!(meta.platforms.contains(&"browser".to_string()));
        assert!(meta.platforms.contains(&"cli".to_string()));
    }

    // --- Basic metadata extraction ---

    #[test]
    fn test_extracts_file_size() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();
        let data = b"hello world";

        let input = make_input(data, "test.txt", serde_json::Map::new());
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        let size = output.metadata.get("file_size").unwrap();
        assert_eq!(size.as_u64().unwrap(), 11);
    }

    #[test]
    fn test_extracts_extension() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(b"data", "photo.JPG", serde_json::Map::new());
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        let ext = output.metadata.get("extension").unwrap();
        assert_eq!(ext.as_str().unwrap(), "jpg");
    }

    #[test]
    fn test_extracts_mime_type() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(b"data", "image.png", serde_json::Map::new());
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        let mime = output.metadata.get("mime_type").unwrap();
        assert_eq!(mime.as_str().unwrap(), "image/png");
    }

    #[test]
    fn test_extracts_filename() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(b"data", "report.csv", serde_json::Map::new());
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        let name = output.metadata.get("filename").unwrap();
        assert_eq!(name.as_str().unwrap(), "report.csv");
    }

    // --- Hash ---

    #[test]
    fn test_no_hash_by_default() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(b"data", "file.txt", serde_json::Map::new());
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert!(output.metadata.get("sha256").is_none());
    }

    #[test]
    fn test_hash_when_enabled() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            b"hello",
            "file.txt",
            params(&[("include_hash", serde_json::json!(true))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        let hash = output.metadata.get("sha256").unwrap().as_str().unwrap();
        // SHA-256 of "hello" is a well-known value.
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_hash_empty_input() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(
            b"",
            "empty.bin",
            params(&[("include_hash", serde_json::json!(true))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        let hash = output.metadata.get("sha256").unwrap().as_str().unwrap();
        // SHA-256 of empty string.
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    // --- Data passthrough ---

    #[test]
    fn test_data_passes_through_unchanged() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();
        let data = b"original file content";

        let input = make_input(data, "file.bin", serde_json::Map::new());
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].data.clone().into_bytes().unwrap(), data);
        assert_eq!(output.files[0].filename, "file.bin");
    }

    #[test]
    fn test_filename_passes_through() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(b"data", "my-file.csv", serde_json::Map::new());
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files[0].filename, "my-file.csv");
    }

    // --- Edge cases ---

    #[test]
    fn test_no_extension_file() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(b"data", "Makefile", serde_json::Map::new());
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        let ext = output.metadata.get("extension").unwrap();
        assert_eq!(ext.as_str().unwrap(), "");
        let mime = output.metadata.get("mime_type").unwrap();
        assert_eq!(mime.as_str().unwrap(), "application/octet-stream");
    }

    #[test]
    fn test_dotfile() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(b"data", ".gitignore", serde_json::Map::new());
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        // Dotfiles have no real extension.
        let ext = output.metadata.get("extension").unwrap();
        assert_eq!(ext.as_str().unwrap(), "");
    }

    #[test]
    fn test_empty_file() {
        let processor = FileMetadata::new();
        let progress = ProgressReporter::new_noop();

        let input = make_input(b"", "empty.txt", serde_json::Map::new());
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        let size = output.metadata.get("file_size").unwrap();
        assert_eq!(size.as_u64().unwrap(), 0);
    }

    // --- Pure function tests ---

    #[test]
    fn test_extract_extension_various() {
        assert_eq!(extract_extension("photo.jpg"), "jpg");
        assert_eq!(extract_extension("PHOTO.JPG"), "jpg");
        assert_eq!(extract_extension("archive.tar.gz"), "gz");
        assert_eq!(extract_extension("README"), "");
        assert_eq!(extract_extension(".gitignore"), "");
    }

    #[test]
    fn test_guess_mime_type_various() {
        assert_eq!(guess_mime_type("photo.jpg"), "image/jpeg");
        assert_eq!(guess_mime_type("photo.jpeg"), "image/jpeg");
        assert_eq!(guess_mime_type("icon.png"), "image/png");
        assert_eq!(guess_mime_type("data.csv"), "text/csv");
        assert_eq!(guess_mime_type("config.json"), "application/json");
        assert_eq!(guess_mime_type("doc.pdf"), "application/pdf");
        assert_eq!(guess_mime_type("unknown.xyz"), "application/octet-stream");
    }

    #[test]
    fn test_sha256_known_values() {
        // "abc" -> well-known SHA-256
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
