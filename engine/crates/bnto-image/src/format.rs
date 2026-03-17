// Image format detection via magic bytes (primary) and file extension (fallback).
// Magic bytes are more reliable than browser-provided MIME types.

/// Supported image formats for compression, resize, and conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    Png,
    /// Lossless-only in Rust `image` crate. Lossy planned via jSquash JS fallback.
    WebP,
}

// --- Format Detection ---

/// JPEG: FF D8 FF, PNG: 89 50 4E 47 0D 0A 1A 0A, WebP: RIFF....WEBP
const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

impl ImageFormat {
    /// Detect format from file header bytes (magic bytes).
    pub fn from_magic_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 4 {
            return None;
        }
        if data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF {
            return Some(Self::Jpeg);
        }
        if data.len() >= 8 && data[..8] == PNG_SIGNATURE {
            return Some(Self::Png);
        }
        if data.len() >= 12 && &data[..4] == b"RIFF" && &data[8..12] == b"WEBP" {
            return Some(Self::WebP);
        }
        None
    }

    /// Fallback detection from filename extension (case-insensitive).
    pub fn from_extension(filename: &str) -> Option<Self> {
        let lower = filename.to_lowercase();
        if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
            Some(Self::Jpeg)
        } else if lower.ends_with(".png") {
            Some(Self::Png)
        } else if lower.ends_with(".webp") {
            Some(Self::WebP)
        } else {
            None
        }
    }

    /// Detect format: magic bytes first, extension fallback.
    pub fn detect(data: &[u8], filename: &str) -> Option<Self> {
        Self::from_magic_bytes(data).or_else(|| Self::from_extension(filename))
    }

    /// MIME type for this format.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::WebP => "image/webp",
        }
    }

    /// File extension without the dot.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::WebP => "webp",
        }
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    // --- Magic Bytes Detection Tests ---

    #[test]
    fn test_detect_jpeg_from_magic_bytes() {
        // Real JPEG files start with FF D8 FF. We'll use the actual
        // test fixture to make sure we detect real files correctly.
        let jpeg_data = include_bytes!("../../../../test-fixtures/images/small.jpg");

        let format = ImageFormat::from_magic_bytes(jpeg_data);

        assert_eq!(format, Some(ImageFormat::Jpeg));
    }

    #[test]
    fn test_detect_png_from_magic_bytes() {
        let png_data = include_bytes!("../../../../test-fixtures/images/small.png");
        let format = ImageFormat::from_magic_bytes(png_data);
        assert_eq!(format, Some(ImageFormat::Png));
    }

    #[test]
    fn test_detect_webp_from_magic_bytes() {
        let webp_data = include_bytes!("../../../../test-fixtures/images/small.webp");
        let format = ImageFormat::from_magic_bytes(webp_data);
        assert_eq!(format, Some(ImageFormat::WebP));
    }

    #[test]
    fn test_magic_bytes_returns_none_for_unknown_data() {
        // Random bytes that don't match any known signature.
        let unknown_data = b"Hello, I am not an image!";
        let format = ImageFormat::from_magic_bytes(unknown_data);
        assert_eq!(format, None);
    }

    #[test]
    fn test_magic_bytes_returns_none_for_too_short_data() {
        // Less than 4 bytes — not enough to check any signature.
        let short_data = b"Hi";
        let format = ImageFormat::from_magic_bytes(short_data);
        assert_eq!(format, None);
    }

    #[test]
    fn test_magic_bytes_returns_none_for_empty_data() {
        let empty: &[u8] = b"";
        let format = ImageFormat::from_magic_bytes(empty);
        assert_eq!(format, None);
    }

    // --- Extension Detection Tests ---

    #[test]
    fn test_detect_jpeg_from_extension_jpg() {
        assert_eq!(
            ImageFormat::from_extension("photo.jpg"),
            Some(ImageFormat::Jpeg)
        );
    }

    #[test]
    fn test_detect_jpeg_from_extension_jpeg() {
        assert_eq!(
            ImageFormat::from_extension("photo.jpeg"),
            Some(ImageFormat::Jpeg)
        );
    }

    #[test]
    fn test_detect_jpeg_case_insensitive() {
        // Users might have "PHOTO.JPG" from a camera.
        assert_eq!(
            ImageFormat::from_extension("PHOTO.JPG"),
            Some(ImageFormat::Jpeg)
        );
        assert_eq!(
            ImageFormat::from_extension("Photo.Jpeg"),
            Some(ImageFormat::Jpeg)
        );
    }

    #[test]
    fn test_detect_png_from_extension() {
        assert_eq!(
            ImageFormat::from_extension("screenshot.png"),
            Some(ImageFormat::Png)
        );
    }

    #[test]
    fn test_detect_webp_from_extension() {
        assert_eq!(
            ImageFormat::from_extension("image.webp"),
            Some(ImageFormat::WebP)
        );
    }

    #[test]
    fn test_extension_returns_none_for_unsupported() {
        assert_eq!(ImageFormat::from_extension("image.bmp"), None);
        assert_eq!(ImageFormat::from_extension("image.gif"), None);
        assert_eq!(ImageFormat::from_extension("image.tiff"), None);
        assert_eq!(ImageFormat::from_extension("document.pdf"), None);
    }

    #[test]
    fn test_extension_returns_none_for_no_extension() {
        assert_eq!(ImageFormat::from_extension("noextension"), None);
    }

    // --- Combined Detection Tests ---

    #[test]
    fn test_detect_uses_magic_bytes_first() {
        // Give it JPEG data but a .png extension.
        // Magic bytes should win — this IS a JPEG despite the extension.
        let jpeg_data = include_bytes!("../../../../test-fixtures/images/small.jpg");
        let format = ImageFormat::detect(jpeg_data, "misleading.png");
        assert_eq!(format, Some(ImageFormat::Jpeg));
    }

    #[test]
    fn test_detect_falls_back_to_extension() {
        // Give it unrecognizable data but a valid extension.
        // Extension should save us.
        let unknown_data = b"not a real image but trust the name";
        let format = ImageFormat::detect(unknown_data, "photo.jpg");
        assert_eq!(format, Some(ImageFormat::Jpeg));
    }

    #[test]
    fn test_detect_returns_none_when_both_fail() {
        let unknown_data = b"not a real image";
        let format = ImageFormat::detect(unknown_data, "mystery_file");
        assert_eq!(format, None);
    }

    // --- Utility Method Tests ---

    #[test]
    fn test_mime_types() {
        assert_eq!(ImageFormat::Jpeg.mime_type(), "image/jpeg");
        assert_eq!(ImageFormat::Png.mime_type(), "image/png");
        assert_eq!(ImageFormat::WebP.mime_type(), "image/webp");
    }

    #[test]
    fn test_extensions() {
        assert_eq!(ImageFormat::Jpeg.extension(), "jpg");
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::WebP.extension(), "webp");
    }

    // =========================================================================
    // Edge Case Tests — Truncated, Corrupt, and Boundary-Length Inputs
    // =========================================================================
    //
    // These tests verify that format detection handles malformed, truncated,
    // and boundary-length inputs without panicking. In the browser, users can
    // drop any file — we need graceful detection failure, not a crash.

    // --- Single-Byte and Very Short Data ---

    #[test]
    fn test_magic_bytes_single_byte_returns_none() {
        // A single byte is too short for any format signature.
        // JPEG needs 3 bytes (FF D8 FF), PNG needs 8, WebP needs 12.
        // The < 4 guard at the top of from_magic_bytes catches this.
        assert_eq!(ImageFormat::from_magic_bytes(&[0xFF]), None);
    }

    #[test]
    fn test_magic_bytes_two_bytes_returns_none() {
        // Two bytes — still too short even for JPEG detection.
        assert_eq!(ImageFormat::from_magic_bytes(&[0xFF, 0xD8]), None);
    }

    #[test]
    fn test_magic_bytes_three_bytes_returns_none() {
        // Three bytes — exactly at the < 4 boundary check, so returns None.
        // Even though FF D8 FF is a valid JPEG start, we require at least
        // 4 bytes total before we start checking.
        assert_eq!(ImageFormat::from_magic_bytes(&[0xFF, 0xD8, 0xFF]), None);
    }

    // --- JPEG Magic Byte Boundaries ---

    #[test]
    fn test_magic_bytes_exactly_4_bytes_jpeg_detected() {
        // Four bytes — the minimum length that passes the < 4 guard.
        // FF D8 FF E0 is a valid JPEG SOI + APP0 start.
        // This should successfully detect as JPEG.
        let data = [0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(
            ImageFormat::from_magic_bytes(&data),
            Some(ImageFormat::Jpeg)
        );
    }

    #[test]
    fn test_magic_bytes_jpeg_like_but_third_byte_not_ff() {
        // Starts with FF D8 (the JPEG SOI marker) but the third byte
        // is not FF, which means it's not a valid JPEG marker sequence.
        // Our detection checks data[2] == 0xFF, so this should NOT match.
        let data = [0xFF, 0xD8, 0x00, 0x00];
        assert_eq!(ImageFormat::from_magic_bytes(&data), None);
    }

    #[test]
    fn test_magic_bytes_jpeg_header_only_no_image_data() {
        // Valid JPEG header bytes (4 bytes) but nothing after.
        // Format detection only looks at the header — it doesn't try
        // to parse the full file. So this should detect as JPEG.
        // The decoder (in compress.rs) will fail later when it tries
        // to read the actual image data.
        let data = [0xFF, 0xD8, 0xFF, 0xE1, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(
            ImageFormat::from_magic_bytes(&data),
            Some(ImageFormat::Jpeg)
        );
    }

    // --- PNG Magic Byte Boundaries ---

    #[test]
    fn test_magic_bytes_7_bytes_partial_png_returns_none() {
        // Seven bytes — one short of the full 8-byte PNG signature.
        // The PNG check requires data.len() >= 8, so this should fail.
        let data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A];
        assert_eq!(ImageFormat::from_magic_bytes(&data), None);
    }

    #[test]
    fn test_magic_bytes_exactly_8_bytes_png_detected() {
        // Eight bytes — the exact minimum for PNG detection.
        // This is the complete PNG signature with nothing after it.
        let data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(ImageFormat::from_magic_bytes(&data), Some(ImageFormat::Png));
    }

    #[test]
    fn test_magic_bytes_png_with_wrong_final_byte() {
        // Correct first 7 bytes but wrong 8th byte.
        // Should NOT detect as PNG — the signature must be exact.
        let data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x00];
        assert_eq!(ImageFormat::from_magic_bytes(&data), None);
    }

    // --- WebP Magic Byte Boundaries ---

    #[test]
    fn test_magic_bytes_11_bytes_partial_webp_returns_none() {
        // Eleven bytes — one short of the 12-byte minimum for WebP detection.
        // Has "RIFF" at 0-3 and "WEB" at 8-10, but no "P" at byte 11.
        let data = [
            b'R', b'I', b'F', b'F', 0x00, 0x00, 0x00, 0x00, b'W', b'E', b'B',
        ];
        assert_eq!(ImageFormat::from_magic_bytes(&data), None);
    }

    #[test]
    fn test_magic_bytes_exactly_12_bytes_webp_detected() {
        // Twelve bytes — the exact minimum for WebP detection.
        // "RIFF" + 4 size bytes + "WEBP" = valid WebP container start.
        let data = [
            b'R', b'I', b'F', b'F', // RIFF marker
            0x00, 0x00, 0x00, 0x00, // file size (placeholder)
            b'W', b'E', b'B', b'P', // WEBP marker
        ];
        assert_eq!(
            ImageFormat::from_magic_bytes(&data),
            Some(ImageFormat::WebP)
        );
    }

    #[test]
    fn test_magic_bytes_riff_but_not_webp() {
        // RIFF container but the chunk type is "AVI " not "WEBP".
        // Should NOT detect as WebP — RIFF is used by many formats.
        let data = [
            b'R', b'I', b'F', b'F', // RIFF marker
            0x00, 0x00, 0x00, 0x00, // file size
            b'A', b'V', b'I', b' ', // AVI chunk type (not WEBP)
        ];
        assert_eq!(ImageFormat::from_magic_bytes(&data), None);
    }

    // --- Combined Detection with Truncated Data ---

    #[test]
    fn test_detect_zero_bytes_with_jpg_extension_uses_extension() {
        // Zero-byte data but a .jpg extension. Magic bytes fail (too short),
        // so extension fallback kicks in and detects as JPEG.
        let format = ImageFormat::detect(b"", "empty.jpg");
        assert_eq!(format, Some(ImageFormat::Jpeg));
    }

    #[test]
    fn test_detect_zero_bytes_no_extension_returns_none() {
        // Zero-byte data and no recognizable extension. Both detection
        // strategies fail — this is genuinely unidentifiable.
        let format = ImageFormat::detect(b"", "unknown_file");
        assert_eq!(format, None);
    }

    #[test]
    fn test_detect_single_byte_with_png_extension_uses_extension() {
        // Single byte of data with a .png extension. Magic bytes can't
        // detect anything from 1 byte, so extension wins.
        let format = ImageFormat::detect(&[0x42], "tiny.png");
        assert_eq!(format, Some(ImageFormat::Png));
    }

    #[test]
    fn test_detect_4_bytes_jpeg_ignores_wrong_extension() {
        // Valid JPEG magic bytes with a .png extension.
        // Magic bytes should win — the data IS JPEG regardless of name.
        let data = [0xFF, 0xD8, 0xFF, 0xE0];
        let format = ImageFormat::detect(&data, "lies.png");
        assert_eq!(format, Some(ImageFormat::Jpeg));
    }
}
