// =============================================================================
// Image Format Detection — What Kind of Image Is This?
// =============================================================================
//
// WHAT IS THIS FILE?
// Before we can compress an image, we need to know what format it is.
// Is it a JPEG? A PNG? A WebP? This module figures that out.
//
// HOW DOES IT DETECT THE FORMAT?
// Two strategies, in order of reliability:
//
//   1. "Magic bytes" — the first few bytes of a file are a signature
//      that identifies the format. JPEG files always start with FF D8 FF,
//      PNG files start with 89 50 4E 47, etc. This is the most reliable
//      method because it looks at the actual file content, not the name.
//
//   2. File extension — if magic bytes are inconclusive, fall back to
//      checking the filename extension (.jpg, .png, .webp). Less reliable
//      because users can name files whatever they want.
//
// WHY NOT JUST USE THE MIME TYPE?
// The browser provides MIME types from the File API, but they're not
// always reliable (some browsers guess, some use OS file associations).
// Magic bytes are the ground truth.

// =============================================================================
// The ImageFormat Enum
// =============================================================================

/// The image formats we support for compression.
///
/// RUST CONCEPT: `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`
/// These are "derived traits" — the compiler generates code for:
///   - `Debug` — lets us print the value with `{:?}` (like console.dir)
///   - `Clone` — lets us make a copy with `.clone()`
///   - `Copy` — lets us copy implicitly (without calling .clone()).
///     Only small, simple types can be `Copy`. Our enum has no
///     heap data, so it qualifies. Think of it like passing a
///     number — you don't "move" the number 42, you just copy it.
///   - `PartialEq` + `Eq` — lets us compare with `==`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// JPEG — lossy compression, great for photos.
    /// Supports quality setting (1-100). Lower quality = smaller file.
    /// Most common image format on the web.
    Jpeg,

    /// PNG — lossless compression, great for graphics with sharp edges.
    /// Supports compression level (how hard the encoder tries to shrink).
    /// Always lossless — no quality loss, but larger files than JPEG.
    Png,

    /// WebP — modern format by Google. Supports both lossy and lossless.
    /// NOTE: Our Rust encoder currently only supports LOSSLESS WebP.
    /// Lossy WebP encoding will be added later via a JS fallback (jSquash).
    WebP,
}

// =============================================================================
// Format Detection Functions
// =============================================================================

impl ImageFormat {
    /// Detect the image format by examining the first few bytes of the file.
    ///
    /// "Magic bytes" (also called "file signatures") are specific byte
    /// patterns at the start of a file that identify its format. Every
    /// JPEG starts with FF D8 FF, every PNG starts with 89 50 4E 47, etc.
    /// This is more reliable than checking file extensions.
    ///
    /// RUST CONCEPT: `&[u8]`
    /// This is a "slice" — a reference to a contiguous chunk of bytes.
    /// Think of it as a read-only view into an array. The `&` means
    /// we're borrowing (reading) the data, not taking ownership of it.
    /// The caller still owns the bytes after this function returns.
    ///
    /// RUST CONCEPT: `Option<Self>`
    /// Returns `Some(ImageFormat::Jpeg)` if we recognized the format,
    /// or `None` if we couldn't identify it. `Self` is shorthand for
    /// the type we're implementing (`ImageFormat`).
    pub fn from_magic_bytes(data: &[u8]) -> Option<Self> {
        // We need at least a few bytes to check the signature.
        // If the file is too small to contain a valid header, bail out.
        if data.len() < 4 {
            return None;
        }

        // --- Check JPEG ---
        // JPEG files start with FF D8 FF (three specific bytes).
        // FF D8 is the "Start of Image" marker. The third byte FF
        // starts the next marker segment (which varies, so we only
        // check the first two bytes to be safe).
        //
        // RUST CONCEPT: `data[0]` and hex literals
        // `0xFF` is hexadecimal for 255. We're comparing individual bytes.
        // In Rust, `data[0]` accesses the first byte (index 0).
        if data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF {
            return Some(Self::Jpeg);
        }

        // --- Check PNG ---
        // PNG files start with an 8-byte signature:
        //   89 50 4E 47 0D 0A 1A 0A
        // The first 4 bytes (89 50 4E 47) spell out ".PNG" (with a
        // leading 0x89 to detect transmission damage). We check all 8
        // bytes for maximum reliability.
        //
        // RUST CONCEPT: `data.len() >= 8`
        // We check the length first to avoid "index out of bounds" panics.
        // In Rust, accessing an array index beyond its length crashes
        // the program (unlike JS, which returns `undefined`).
        if data.len() >= 8
            && data[0] == 0x89
            && data[1] == 0x50  // 'P'
            && data[2] == 0x4E  // 'N'
            && data[3] == 0x47  // 'G'
            && data[4] == 0x0D  // Carriage Return
            && data[5] == 0x0A  // Line Feed
            && data[6] == 0x1A  // EOF character
            && data[7] == 0x0A
        // Line Feed
        {
            return Some(Self::Png);
        }

        // --- Check WebP ---
        // WebP files are stored in a RIFF container. They start with:
        //   "RIFF" (4 bytes) + file size (4 bytes) + "WEBP" (4 bytes)
        // So bytes 0-3 are "RIFF" and bytes 8-11 are "WEBP".
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

        // We didn't recognize the format from the magic bytes.
        None
    }

    /// Detect the image format from a filename extension.
    ///
    /// This is the fallback when magic bytes don't work (e.g., the file
    /// data isn't available yet, or it's corrupted). Less reliable than
    /// magic bytes because users can rename files to anything.
    ///
    /// RUST CONCEPT: `&str`
    /// A string slice — a reference to text. We don't need to own the
    /// filename, just read it. The caller keeps ownership.
    pub fn from_extension(filename: &str) -> Option<Self> {
        // Convert to lowercase so "Photo.JPG" and "photo.jpg" both work.
        //
        // RUST CONCEPT: `.to_lowercase()`
        // Creates a new String with all characters lowered. We call this
        // on the filename so the extension check is case-insensitive.
        let lower = filename.to_lowercase();

        // RUST CONCEPT: `if ... else if ... else` chain
        // Rust doesn't have `switch` — we use `if` chains or `match`.
        // `.ends_with()` is like JavaScript's `.endsWith()`.
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

    /// Try to detect the format using both strategies: magic bytes first,
    /// then fall back to the file extension.
    ///
    /// This is the primary function consumers should call. It gives the
    /// most reliable result by trying the most reliable method first.
    pub fn detect(data: &[u8], filename: &str) -> Option<Self> {
        // Try magic bytes first (most reliable)
        Self::from_magic_bytes(data)
            // If magic bytes returned None, try the extension
            //
            // RUST CONCEPT: `.or_else(|| ...)`
            // `.or_else()` on an Option says: "if this is None, try this
            // other thing instead". The `||` is a closure (arrow function
            // in JS). It's only called if the first attempt was None.
            .or_else(|| Self::from_extension(filename))
    }

    /// Get the MIME type string for this format.
    /// Used when creating output files — the browser needs the MIME type
    /// to construct a proper Blob for download.
    pub fn mime_type(&self) -> &'static str {
        // RUST CONCEPT: `match`
        // `match` is like a super-powered `switch` in JS. The compiler
        // ensures you handle EVERY possible variant — if you add a new
        // format to the enum and forget to add it here, the code won't
        // compile. This prevents bugs!
        //
        // RUST CONCEPT: `&'static str`
        // A string literal that lives for the entire program's lifetime.
        // "image/jpeg" is baked into the compiled binary — it's never
        // allocated or freed. The `'static` lifetime means "forever".
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::WebP => "image/webp",
        }
    }

    /// Get the standard file extension for this format (without the dot).
    /// Used when generating output filenames.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::WebP => "webp",
        }
    }
}

// =============================================================================
// Tests — Written BEFORE the implementation (TDD!)
// =============================================================================
//
// These tests were written first to define the expected behavior.
// The implementation above was then written to make them pass.

#[cfg(test)]
mod tests {
    use super::*;

    // --- Magic Bytes Detection Tests ---

    #[test]
    fn test_detect_jpeg_from_magic_bytes() {
        // Real JPEG files start with FF D8 FF. We'll use the actual
        // test fixture to make sure we detect real files correctly.
        //
        // RUST CONCEPT: `include_bytes!()`
        // This macro reads a file at COMPILE TIME and embeds its bytes
        // directly into the binary. The path is relative to the source
        // file (this .rs file). The result is a `&[u8]` — a byte slice.
        // This is perfect for test fixtures because:
        //   1. No filesystem access needed at runtime (works in WASM too!)
        //   2. If the file is missing, the BUILD fails (not the test)
        //   3. Zero cost at runtime — bytes are already in memory
        let jpeg_data = include_bytes!("../../../../test-fixtures/images/small.jpg");

        let format = ImageFormat::from_magic_bytes(jpeg_data);

        // RUST CONCEPT: `assert_eq!(a, b)`
        // Like `assert.equal(a, b)` in JS. If they're not equal, the
        // test fails and shows both values. Needs `PartialEq` trait.
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
