// =============================================================================
// Image Format Detection — Magic Bytes and Extension Matching
// =============================================================================
//
// Detects image format via two strategies (in order of reliability):
//   1. Magic bytes — file signature in the first few bytes (ground truth)
//   2. File extension — fallback when magic bytes are inconclusive
//
// Magic bytes are more reliable than browser MIME types, which can be
// guessed from OS file associations rather than actual file content.

// =============================================================================
// The ImageFormat Enum
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// Lossy compression, quality 1-100. Most common web image format.
    Jpeg,

    /// Lossless compression. Larger files than JPEG but no quality loss.
    Png,

    /// Modern format by Google. Our Rust encoder currently supports
    /// lossless only — lossy WebP will be added via JS fallback (jSquash).
    WebP,
}

// =============================================================================
// Format Detection Functions
// =============================================================================

impl ImageFormat {
    /// Detect format from the first few bytes of a file ("magic bytes").
    /// More reliable than file extensions since it examines actual content.
    pub fn from_magic_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 4 {
            return None;
        }

        // JPEG: FF D8 FF (Start of Image marker + next marker)
        if data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF {
            return Some(Self::Jpeg);
        }

        // PNG: 89 50 4E 47 0D 0A 1A 0A (8-byte signature)
        if data.len() >= 8
            && data[0] == 0x89
            && data[1] == 0x50  // 'P'
            && data[2] == 0x4E  // 'N'
            && data[3] == 0x47  // 'G'
            && data[4] == 0x0D  // CR
            && data[5] == 0x0A  // LF
            && data[6] == 0x1A  // EOF
            && data[7] == 0x0A
        // LF
        {
            return Some(Self::Png);
        }

        // WebP: RIFF container — "RIFF" at 0-3, "WEBP" at 8-11
        if data.len() >= 12
            && data[0] == b'R'
            && data[1] == b'I'
            && data[2] == b'F'
            && data[3] == b'F'
            && data[8] == b'W'
            && data[9] == b'E'
            && data[10] == b'B'
            && data[11] == b'P'
        {
            return Some(Self::WebP);
        }

        None
    }

    /// Detect format from filename extension (case-insensitive).
    /// Fallback when magic bytes are unavailable or inconclusive.
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

    /// Detect format using magic bytes first, falling back to extension.
    pub fn detect(data: &[u8], filename: &str) -> Option<Self> {
        Self::from_magic_bytes(data).or_else(|| Self::from_extension(filename))
    }

    /// MIME type string for constructing browser Blobs for download.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::WebP => "image/webp",
        }
    }

    /// Standard file extension (without the dot).
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::WebP => "webp",
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Magic Bytes Detection Tests ---

    #[test]
    fn test_detect_jpeg_from_magic_bytes() {
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
        let unknown_data = b"Hello, I am not an image!";
        let format = ImageFormat::from_magic_bytes(unknown_data);
        assert_eq!(format, None);
    }

    #[test]
    fn test_magic_bytes_returns_none_for_too_short_data() {
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
        // JPEG data with .png extension — magic bytes should win.
        let jpeg_data = include_bytes!("../../../../test-fixtures/images/small.jpg");
        let format = ImageFormat::detect(jpeg_data, "misleading.png");
        assert_eq!(format, Some(ImageFormat::Jpeg));
    }

    #[test]
    fn test_detect_falls_back_to_extension() {
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

    // --- Single-Byte and Very Short Data ---

    #[test]
    fn test_magic_bytes_single_byte_returns_none() {
        assert_eq!(ImageFormat::from_magic_bytes(&[0xFF]), None);
    }

    #[test]
    fn test_magic_bytes_two_bytes_returns_none() {
        assert_eq!(ImageFormat::from_magic_bytes(&[0xFF, 0xD8]), None);
    }

    #[test]
    fn test_magic_bytes_three_bytes_returns_none() {
        // FF D8 FF is valid JPEG start but we require >= 4 bytes total.
        assert_eq!(ImageFormat::from_magic_bytes(&[0xFF, 0xD8, 0xFF]), None);
    }

    // --- JPEG Magic Byte Boundaries ---

    #[test]
    fn test_magic_bytes_exactly_4_bytes_jpeg_detected() {
        let data = [0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(
            ImageFormat::from_magic_bytes(&data),
            Some(ImageFormat::Jpeg)
        );
    }

    #[test]
    fn test_magic_bytes_jpeg_like_but_third_byte_not_ff() {
        // FF D8 but third byte is not FF — not a valid JPEG marker sequence.
        let data = [0xFF, 0xD8, 0x00, 0x00];
        assert_eq!(ImageFormat::from_magic_bytes(&data), None);
    }

    #[test]
    fn test_magic_bytes_jpeg_header_only_no_image_data() {
        // Format detection only examines the header — the decoder will
        // fail later when it tries to read actual image data.
        let data = [0xFF, 0xD8, 0xFF, 0xE1, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(
            ImageFormat::from_magic_bytes(&data),
            Some(ImageFormat::Jpeg)
        );
    }

    // --- PNG Magic Byte Boundaries ---

    #[test]
    fn test_magic_bytes_7_bytes_partial_png_returns_none() {
        // One short of the 8-byte PNG signature.
        let data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A];
        assert_eq!(ImageFormat::from_magic_bytes(&data), None);
    }

    #[test]
    fn test_magic_bytes_exactly_8_bytes_png_detected() {
        let data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(ImageFormat::from_magic_bytes(&data), Some(ImageFormat::Png));
    }

    #[test]
    fn test_magic_bytes_png_with_wrong_final_byte() {
        let data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x00];
        assert_eq!(ImageFormat::from_magic_bytes(&data), None);
    }

    // --- WebP Magic Byte Boundaries ---

    #[test]
    fn test_magic_bytes_11_bytes_partial_webp_returns_none() {
        let data = [
            b'R', b'I', b'F', b'F', 0x00, 0x00, 0x00, 0x00, b'W', b'E', b'B',
        ];
        assert_eq!(ImageFormat::from_magic_bytes(&data), None);
    }

    #[test]
    fn test_magic_bytes_exactly_12_bytes_webp_detected() {
        let data = [
            b'R', b'I', b'F', b'F', 0x00, 0x00, 0x00, 0x00, b'W', b'E', b'B', b'P',
        ];
        assert_eq!(
            ImageFormat::from_magic_bytes(&data),
            Some(ImageFormat::WebP)
        );
    }

    #[test]
    fn test_magic_bytes_riff_but_not_webp() {
        // RIFF container but AVI chunk type, not WEBP.
        let data = [
            b'R', b'I', b'F', b'F', 0x00, 0x00, 0x00, 0x00, b'A', b'V', b'I', b' ',
        ];
        assert_eq!(ImageFormat::from_magic_bytes(&data), None);
    }

    // --- Combined Detection with Truncated Data ---

    #[test]
    fn test_detect_zero_bytes_with_jpg_extension_uses_extension() {
        let format = ImageFormat::detect(b"", "empty.jpg");
        assert_eq!(format, Some(ImageFormat::Jpeg));
    }

    #[test]
    fn test_detect_zero_bytes_no_extension_returns_none() {
        let format = ImageFormat::detect(b"", "unknown_file");
        assert_eq!(format, None);
    }

    #[test]
    fn test_detect_single_byte_with_png_extension_uses_extension() {
        let format = ImageFormat::detect(&[0x42], "tiny.png");
        assert_eq!(format, Some(ImageFormat::Png));
    }

    #[test]
    fn test_detect_4_bytes_jpeg_ignores_wrong_extension() {
        // Magic bytes win over extension.
        let data = [0xFF, 0xD8, 0xFF, 0xE0];
        let format = ImageFormat::detect(&data, "lies.png");
        assert_eq!(format, Some(ImageFormat::Jpeg));
    }
}
