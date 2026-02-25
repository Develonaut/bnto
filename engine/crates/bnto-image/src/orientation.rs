// =============================================================================
// Image Orientation — EXIF Orientation Extraction and Application
// =============================================================================
//
// WHAT IS THIS FILE?
// When you take a photo with your smartphone, the camera sensor always captures
// pixels in the same physical orientation (landscape). But if you hold the phone
// upright (portrait), the camera stores the photo with the pixels sideways and
// adds a tiny tag in the EXIF metadata that says "rotate 90° to view correctly."
//
// This tag is called the EXIF Orientation tag (tag 0x0112). It has 8 possible
// values, but the most common ones are:
//   - 1 (Normal): no rotation needed
//   - 6 (Rotate 90° CW): phone was held upright (portrait)
//   - 3 (Rotate 180°): phone was upside down
//   - 8 (Rotate 270° CW): phone was rotated left (portrait, other way)
//
// WHY DOES THIS MATTER?
// The `image` crate's `decode()` method reads pixels as-stored in the file,
// without applying the EXIF orientation tag. If we don't read and apply the
// tag ourselves, a portrait photo will come out sideways in the output file.
//
// The browser shows the ORIGINAL file correctly because browsers read EXIF
// orientation automatically — but our re-encoded output has no EXIF tag
// (the `image` crate strips all metadata when re-encoding), so the output
// displays with whatever pixel orientation we gave it.
//
// THE FIX:
// Before processing, we:
//   1. Read the EXIF orientation tag from the original file bytes
//   2. Decode the image to pixels (standard decode)
//   3. Physically rotate/flip the pixels to match the intended display
//
// The output file has correctly-oriented pixels and doesn't need an EXIF
// tag to display correctly — it looks right in every viewer, on every platform.
//
// PIPELINE:
//   Raw bytes → extract orientation → decode pixels → apply rotation → process
//
// PERFORMANCE NOTE:
// We create two cursors over the same in-memory data — one to extract
// orientation (reads only the file header), one to decode pixels (reads
// everything). Since the data is already in memory (not disk), this is
// essentially free — no I/O, no copies.

use std::io::Cursor;

use bnto_core::errors::BntoError;

// `ImageReader` is the main entry point for decoding images from bytes.
// It wraps a reader (anything that implements `Read`) and provides format
// detection and decoding.
use image::ImageReader;

// `ImageDecoder` is the trait that all format-specific decoders implement.
// We need it in scope to call `.orientation()` on the decoder returned by
// `ImageReader::into_decoder()`. Without this import, Rust can't find the
// method even though the concrete type implements it.
//
// RUST CONCEPT: Trait methods require the trait to be in scope
// In Rust, even if a type implements a trait, you can't call the trait's
// methods unless the trait itself is imported. This is to prevent ambiguity
// when multiple traits define methods with the same name.
use image::ImageDecoder;

// `Orientation` represents the 8 possible EXIF orientation transforms.
// It maps directly to EXIF orientation tag values 1-8.
use image::metadata::Orientation;

// =============================================================================
// Public API
// =============================================================================

/// Decode an image from raw bytes, automatically applying EXIF orientation.
///
/// This is the safe way to decode images from user input. It handles the
/// common case of smartphone photos that have EXIF orientation tags.
///
/// PIPELINE:
///   1. Read the EXIF orientation tag from the raw bytes (if present)
///   2. Decode the image to a pixel grid (DynamicImage)
///   3. Rotate/flip the pixels to match the intended display orientation
///   4. Return the correctly-oriented image
///
/// If the image has no EXIF orientation (or it's already "normal"), this
/// behaves identically to a plain `decode()` — no rotation is applied.
///
/// SUPPORTED FORMATS:
///   - JPEG: Full EXIF orientation support (this is where ~99% of rotated
///     images come from — smartphone cameras)
///   - PNG: No EXIF in PNG files — always returns unmodified
///   - WebP: EXIF support varies — the `image` crate may or may not parse it
///
/// RUST CONCEPT: `image::DynamicImage`
/// A `DynamicImage` is an in-memory image as a grid of pixels. It's format-
/// agnostic — whether the source was JPEG, PNG, or WebP, the pixels are
/// the same once decoded. We can then re-encode to any format.
pub fn decode_with_orientation(data: &[u8]) -> Result<image::DynamicImage, BntoError> {
    // --- Step 1: Extract EXIF orientation from the raw bytes ---
    //
    // We do this BEFORE decoding because `decode()` consumes the reader.
    // We create two separate cursors over the same data — one for orientation
    // extraction, one for decoding. Since the data is already in memory
    // (`&[u8]`), creating a second `Cursor` is essentially free (it's just
    // a pointer + offset, no data copying).
    let orientation = extract_orientation(data);

    // --- Step 2: Decode the image to pixels ---
    //
    // This is the standard decode path: wrap bytes in a Cursor, create an
    // ImageReader, guess the format from magic bytes, and decode.
    let cursor = Cursor::new(data);
    let mut img = ImageReader::new(cursor)
        .with_guessed_format()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to read image: {e}")))?
        .decode()
        .map_err(|e| BntoError::ProcessingFailed(format!("Failed to decode image: {e}")))?;

    // --- Step 3: Apply the orientation transform ---
    //
    // `apply_orientation()` physically rotates and/or flips the pixel grid.
    //
    // For `Orientation::NoTransforms` (the common case for PNGs and correctly-
    // oriented JPEGs), this is a no-op — the image is returned unchanged.
    //
    // For `Orientation::Rotate90` (the most common smartphone case), this
    // rotates the pixel grid 90° clockwise. A 100×50 image becomes 50×100.
    //
    // RUST CONCEPT: `&mut self` method
    // `apply_orientation` takes `&mut self` — it modifies the image in place
    // rather than creating a copy. This is more memory-efficient since we
    // don't need two copies of the full pixel grid in memory at once.
    img.apply_orientation(orientation);

    Ok(img)
}

// =============================================================================
// Internal Functions
// =============================================================================

/// Extract the EXIF orientation tag from raw image bytes.
///
/// Returns `Orientation::NoTransforms` if:
/// - The image format doesn't support EXIF (e.g., PNG)
/// - The EXIF data is missing or malformed
/// - Any error occurs during parsing
///
/// This function never fails — it always returns a valid Orientation value.
/// Errors are silently treated as "no rotation needed" because:
///   1. Most images don't need rotation (EXIF orientation = 1 or absent)
///   2. Failing to read orientation should not block image processing
///   3. The worst case is a slightly rotated output, not a crash
///
/// RUST CONCEPT: Defensive programming with fallbacks
/// Rather than returning `Result<Orientation, Error>` and forcing callers
/// to handle errors, we use a "best effort" approach. The function always
/// succeeds. If something goes wrong, we default to "no transform."
fn extract_orientation(data: &[u8]) -> Orientation {
    // --- Create a reader and get the underlying decoder ---
    //
    // `into_decoder()` gives us the format-specific decoder (JpegDecoder,
    // PngDecoder, etc.). Unlike `decode()`, it doesn't produce pixels yet —
    // it just parses the file header enough to expose metadata like
    // dimensions, color type, and orientation.
    let cursor = Cursor::new(data);

    // RUST CONCEPT: `let ... else` (let-else pattern, stabilized in Rust 1.65)
    // If the pattern doesn't match (i.e., the Result is Err), execute the
    // `else` block. This is like an early return for error cases — much
    // cleaner than nested `match` statements when you just want to bail
    // out on any error.
    let Ok(reader) = ImageReader::new(cursor).with_guessed_format() else {
        return Orientation::NoTransforms;
    };

    let Ok(mut decoder) = reader.into_decoder() else {
        return Orientation::NoTransforms;
    };

    // --- Read the orientation from the decoder ---
    //
    // For JPEG: the decoder parses the APP1 segment (EXIF data) and
    //   extracts the orientation tag (0x0112). Returns the matching
    //   Orientation enum variant.
    // For PNG: the decoder's default implementation returns NoTransforms
    //   (PNG doesn't have EXIF orientation).
    // For WebP: depends on the decoder implementation — may or may not
    //   parse EXIF from WebP containers.
    //
    // `.unwrap_or()` handles the case where the decoder encounters an
    // error parsing the orientation — we just treat it as "no rotation."
    decoder.orientation().unwrap_or(Orientation::NoTransforms)
}

// =============================================================================
// Tests
// =============================================================================
//
// Testing orientation is tricky because we need JPEG files with specific
// EXIF orientation tags. Rather than storing binary fixtures, we construct
// them programmatically:
//   1. Create a non-square image (e.g., 60×40) with a known pixel pattern
//   2. Encode as JPEG
//   3. Inject an EXIF APP1 segment with the desired orientation tag
//   4. Decode with our function and verify the output dimensions/pixels

#[cfg(test)]
mod tests {
    use super::*;

    // Import shared test helpers for creating oriented JPEG fixtures.
    use crate::test_utils::{create_test_jpeg, inject_exif_orientation};

    // =========================================================================
    // decode_with_orientation — Dimension Tests
    // =========================================================================
    //
    // These tests verify that EXIF orientation is correctly applied by
    // checking output dimensions. A non-square image (60×40) will have
    // swapped dimensions (40×60) after a 90° or 270° rotation. This is
    // the simplest and most reliable way to detect that rotation happened.

    #[test]
    fn test_orientation_normal_preserves_dimensions() {
        // Orientation 1 (Normal) — no rotation needed.
        // Output should match the stored pixel dimensions exactly.
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
        // Orientation 6 (Rotate 90° CW) — the most common smartphone portrait.
        //
        // When you hold your phone upright and take a photo, the sensor captures
        // a landscape image (e.g., 60 pixels wide × 40 pixels tall). The camera
        // adds EXIF orientation=6, meaning "rotate 90° CW to view correctly."
        //
        // After applying orientation: 60×40 → 40×60 (width and height swap).
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
        // Orientation 3 (Rotate 180°) — phone was upside down.
        // Dimensions stay the same (rotating 180° doesn't swap width/height),
        // but pixels are flipped both horizontally and vertically.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 3);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 60, "Width unchanged for 180° rotation");
        assert_eq!(img.height(), 40, "Height unchanged for 180° rotation");
    }

    #[test]
    fn test_orientation_rotate270_swaps_dimensions() {
        // Orientation 8 (Rotate 270° CW) — phone was rotated left.
        // Like orientation=6, this swaps width and height: 60×40 → 40×60.
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
        // Orientation 2 (Flip horizontal) — mirror left-to-right.
        // Dimensions are unchanged; only pixel order changes.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 2);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 60);
        assert_eq!(img.height(), 40);
    }

    #[test]
    fn test_orientation_flip_vertical_preserves_dimensions() {
        // Orientation 4 (Flip vertical) — mirror top-to-bottom.
        // Dimensions are unchanged.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 4);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 60);
        assert_eq!(img.height(), 40);
    }

    #[test]
    fn test_orientation_rotate270_flip_h_swaps_dimensions() {
        // Orientation 5 (Rotate 270° CW + flip horizontal).
        // The rotation swaps dimensions: 60×40 → 40×60.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 5);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 40);
        assert_eq!(img.height(), 60);
    }

    #[test]
    fn test_orientation_rotate90_flip_h_swaps_dimensions() {
        // Orientation 7 (Rotate 90° CW + flip horizontal).
        // The rotation swaps dimensions: 60×40 → 40×60.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 7);

        let img = decode_with_orientation(&exif_jpeg).unwrap();
        assert_eq!(img.width(), 40);
        assert_eq!(img.height(), 60);
    }

    // =========================================================================
    // No-EXIF and Non-JPEG Tests
    // =========================================================================
    //
    // Verify that images without EXIF data (or formats that don't support
    // EXIF) work correctly — decode_with_orientation should be a no-op.

    #[test]
    fn test_no_exif_is_treated_as_normal() {
        // A plain JPEG without any EXIF data (just JFIF) should decode
        // with dimensions unchanged — no rotation applied.
        let jpeg = create_test_jpeg(60, 40);

        let img = decode_with_orientation(&jpeg).unwrap();
        assert_eq!(img.width(), 60);
        assert_eq!(img.height(), 40);
    }

    #[test]
    fn test_png_has_no_orientation() {
        // PNG files don't have EXIF orientation. Verify decode_with_orientation
        // works correctly (just passes through without modification).
        let png_data = include_bytes!("../../../../test-fixtures/images/small.png");

        let img = decode_with_orientation(png_data).unwrap();
        // small.png is 100×100
        assert_eq!(img.width(), 100);
        assert_eq!(img.height(), 100);
    }

    #[test]
    fn test_webp_has_no_orientation() {
        // WebP: same as PNG — no EXIF orientation in our test fixtures.
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
        // EXIF value 6 should map to Orientation::Rotate90.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 6);

        let orientation = extract_orientation(&exif_jpeg);
        assert_eq!(orientation, Orientation::Rotate90);
    }

    #[test]
    fn test_extract_orientation_from_exif_rotate180() {
        // EXIF value 3 should map to Orientation::Rotate180.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 3);

        let orientation = extract_orientation(&exif_jpeg);
        assert_eq!(orientation, Orientation::Rotate180);
    }

    #[test]
    fn test_extract_orientation_from_exif_rotate270() {
        // EXIF value 8 should map to Orientation::Rotate270.
        let jpeg = create_test_jpeg(60, 40);
        let exif_jpeg = inject_exif_orientation(&jpeg, 8);

        let orientation = extract_orientation(&exif_jpeg);
        assert_eq!(orientation, Orientation::Rotate270);
    }

    #[test]
    fn test_extract_orientation_no_exif_returns_no_transforms() {
        // A plain JPEG without EXIF should return NoTransforms.
        let jpeg = create_test_jpeg(60, 40);

        let orientation = extract_orientation(&jpeg);
        assert_eq!(orientation, Orientation::NoTransforms);
    }

    #[test]
    fn test_extract_orientation_corrupt_data_returns_no_transforms() {
        // Random bytes that aren't a valid image — should gracefully
        // return NoTransforms, never panic.
        let corrupt = b"this is not an image at all";

        let orientation = extract_orientation(corrupt);
        assert_eq!(orientation, Orientation::NoTransforms);
    }

    #[test]
    fn test_extract_orientation_empty_data_returns_no_transforms() {
        // Empty byte slice — should return NoTransforms, never panic.
        let orientation = extract_orientation(&[]);
        assert_eq!(orientation, Orientation::NoTransforms);
    }
}
