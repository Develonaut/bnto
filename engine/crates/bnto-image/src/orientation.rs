// =============================================================================
// Image Orientation — EXIF Orientation Extraction and Application
// =============================================================================
//
// Smartphone cameras store pixels in a fixed physical orientation (landscape)
// and record viewing rotation in an EXIF tag (0x0112). Common values:
//   1 = Normal, 6 = Rotate 90° CW (portrait), 3 = 180°, 8 = Rotate 270° CW
//
// The `image` crate's decode() reads pixels as-stored without applying EXIF.
// If we don't apply the tag ourselves, portrait photos come out sideways in
// the output — because our re-encoded files have no EXIF tag (the `image`
// crate strips all metadata when re-encoding).
//
// Fix: extract orientation tag -> decode pixels -> physically rotate/flip
// the pixel grid. Output has correctly-oriented pixels and needs no EXIF tag.
//
// Two cursors over the same in-memory data (one for orientation extraction,
// one for decoding) — essentially free since data is already in memory.

use std::io::Cursor;

use bnto_core::errors::BntoError;

use image::ImageReader;
// ImageDecoder trait must be in scope to call .orientation() on the decoder.
use image::ImageDecoder;
use image::metadata::Orientation;

// =============================================================================
// Public API
// =============================================================================

/// Decode an image from raw bytes, automatically applying EXIF orientation.
///
/// JPEG: full EXIF support (where ~99% of rotated images come from)
/// PNG: no EXIF — always returns unmodified
/// WebP: EXIF support varies by decoder implementation
///
/// If no orientation tag is present, behaves identically to plain decode().
pub fn decode_with_orientation(data: &[u8]) -> Result<image::DynamicImage, BntoError> {
    let orientation = extract_orientation(data);

    let cursor = Cursor::new(data);
    let mut img = ImageReader::new(cursor)
        .with_guessed_format()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read image: {e}")))?
        .decode()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to decode image: {e}")))?;

    // For NoTransforms (common case), this is a no-op.
    // For Rotate90, physically rotates the pixel grid (100x50 becomes 50x100).
    // Modifies in place to avoid doubling memory usage.
    img.apply_orientation(orientation);

    Ok(img)
}

// =============================================================================
// Internal Functions
// =============================================================================

/// Extract EXIF orientation tag from raw image bytes.
///
/// Never fails — returns NoTransforms on any error. This is intentional:
///   1. Most images don't need rotation
///   2. Failing to read orientation shouldn't block processing
///   3. Worst case is a slightly rotated output, not a crash
fn extract_orientation(data: &[u8]) -> Orientation {
    let cursor = Cursor::new(data);

    // into_decoder() parses the file header (not full pixel decode) to
    // expose metadata like dimensions, color type, and orientation.
    let Ok(reader) = ImageReader::new(cursor).with_guessed_format() else {
        return Orientation::NoTransforms;
    };

    let Ok(mut decoder) = reader.into_decoder() else {
        return Orientation::NoTransforms;
    };

    // JPEG: parses APP1 segment for orientation tag 0x0112
    // PNG: default impl returns NoTransforms (no EXIF in PNG)
    // WebP: depends on decoder implementation
    decoder.orientation().unwrap_or(Orientation::NoTransforms)
}

// =============================================================================
// Tests
// =============================================================================
//
// Testing orientation requires JPEG files with specific EXIF tags.
// We construct them programmatically: create non-square image (60x40),
// encode as JPEG, inject EXIF APP1 segment, then verify dimension swaps.

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::{create_test_jpeg, inject_exif_orientation};

    // =========================================================================
    // decode_with_orientation — Dimension Tests
    // =========================================================================
    //
    // Non-square images (60x40) will have swapped dimensions (40x60) after
    // 90°/270° rotation — the simplest way to detect that rotation happened.

    #[test]
    fn test_orientation_normal_preserves_dimensions() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 1);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(
            img.width(),
            60,
            "Width should be unchanged for normal orientation"
        );
        assert_eq!(
            img.height(),
            40,
            "Height should be unchanged for normal orientation"
        );
    }

    #[test]
    fn test_orientation_rotate90_swaps_dimensions() {
        // Orientation 6 — most common smartphone portrait.
        // Sensor captures 60x40 landscape, tag says "rotate 90° CW".
        // After applying: 60x40 -> 40x60.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 40, "Width should be 40 after 90° CW rotation");
        assert_eq!(
            img.height(),
            60,
            "Height should be 60 after 90° CW rotation"
        );
    }

    #[test]
    fn test_orientation_rotate180_preserves_dimensions() {
        // 180° doesn't swap width/height, only flips pixels.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 3);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 60, "Width unchanged for 180° rotation");
        assert_eq!(img.height(), 40, "Height unchanged for 180° rotation");
    }

    #[test]
    fn test_orientation_rotate270_swaps_dimensions() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 8);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 40, "Width should be 40 after 270° CW rotation");
        assert_eq!(
            img.height(),
            60,
            "Height should be 60 after 270° CW rotation"
        );
    }

    #[test]
    fn test_orientation_flip_horizontal_preserves_dimensions() {
        // Orientation 2: mirror left-to-right. Dimensions unchanged.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 2);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 60);
        assert_eq!(img.height(), 40);
    }

    #[test]
    fn test_orientation_flip_vertical_preserves_dimensions() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 4);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 60);
        assert_eq!(img.height(), 40);
    }

    #[test]
    fn test_orientation_rotate270_flip_h_swaps_dimensions() {
        // Orientation 5: rotation component swaps dimensions.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 5);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 40);
        assert_eq!(img.height(), 60);
    }

    #[test]
    fn test_orientation_rotate90_flip_h_swaps_dimensions() {
        // Orientation 7: rotation component swaps dimensions.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 7);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 40);
        assert_eq!(img.height(), 60);
    }

    // =========================================================================
    // No-EXIF and Non-JPEG Tests
    // =========================================================================

    #[test]
    fn test_no_exif_is_treated_as_normal() {
        let jpeg = create_test_jpeg(60, 40);

        let img = decode_with_orientation(&jpeg).unwrap();
        assert_eq!(img.width(), 60);
        assert_eq!(img.height(), 40);
    }

    #[test]
    fn test_png_has_no_orientation() {
        let png_data = include_bytes!("../../../../test-fixtures/images/small.png");

        let img = decode_with_orientation(png_data).unwrap();
        assert_eq!(img.width(), 100);
        assert_eq!(img.height(), 100);
    }

    #[test]
    fn test_webp_has_no_orientation() {
        let webp_data = include_bytes!("../../../../test-fixtures/images/small.webp");

        let img = decode_with_orientation(webp_data).unwrap();
        assert_eq!(img.width(), 100);
        assert_eq!(img.height(), 100);
    }

    // =========================================================================
    // extract_orientation Unit Tests
    // =========================================================================

    #[test]
    fn test_extract_orientation_from_exif_rotate90() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let orientation = extract_orientation(&exif_jpeg);
        assert_eq!(orientation, Orientation::Rotate90);
    }

    #[test]
    fn test_extract_orientation_from_exif_rotate180() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 3);

        let orientation = extract_orientation(&exif_jpeg);
        assert_eq!(orientation, Orientation::Rotate180);
    }

    #[test]
    fn test_extract_orientation_from_exif_rotate270() {
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 8);

        let orientation = extract_orientation(&exif_jpeg);
        assert_eq!(orientation, Orientation::Rotate270);
    }

    #[test]
    fn test_extract_orientation_no_exif_returns_no_transforms() {
        let jpeg = create_test_jpeg(60, 40);

        let orientation = extract_orientation(&jpeg);
        assert_eq!(orientation, Orientation::NoTransforms);
    }

    #[test]
    fn test_extract_orientation_corrupt_data_returns_no_transforms() {
        let corrupt = b"this is not an image at all";

        let orientation = extract_orientation(corrupt);
        assert_eq!(orientation, Orientation::NoTransforms);
    }

    #[test]
    fn test_extract_orientation_empty_data_returns_no_transforms() {
        let orientation = extract_orientation(&[]);
        assert_eq!(orientation, Orientation::NoTransforms);
    }
}
