// =============================================================================
// Shared Test Utilities for bnto-image
// =============================================================================
//
// WHAT IS THIS FILE?
// Test helper functions used across multiple test modules in bnto-image.
// These functions create test fixtures (images with specific properties)
// that would be tedious and error-prone to duplicate in each test file.
//
// WHY NOT USE STATIC FIXTURE FILES?
// For orientation testing, we need JPEG files with specific EXIF orientation
// tags. Creating these programmatically is more reliable than storing binary
// fixtures because:
//   1. The test code documents exactly what the fixture contains
//   2. We can create any orientation without maintaining 8+ fixture files
//   3. Non-square dimensions prove rotation by checking width/height swap
//
// NOTE: This module is gated behind `#[cfg(test)]` in lib.rs, so there's
// no need for a `#![cfg(test)]` attribute here — it's already test-only.
// This file is completely invisible to production code.

/// Create a JPEG image with known dimensions and a gradient pixel pattern.
///
/// The image is intentionally non-square (you should pass different width
/// and height values) so we can detect rotation by checking output dimensions.
/// A 60×40 image rotated 90° becomes 40×60 — the dimension swap proves
/// rotation worked.
///
/// The gradient pattern:
///   - Red channel increases left → right (0 at x=0, 255 at x=max)
///   - Green channel increases top → bottom (0 at y=0, 255 at y=max)
///   - Blue channel is always 0
///
/// This pattern lets us verify pixel positions after rotation. For example,
/// if the top-left pixel was (0, 0, 0) and after 90° CW rotation it should
/// be at a different position.
///
/// RUST CONCEPT: `image::RgbImage`
/// An alias for `ImageBuffer<Rgb<u8>, Vec<u8>>` — a 2D grid of RGB pixels
/// stored as a flat Vec of bytes (R, G, B, R, G, B, ...). Each pixel is
/// 3 bytes. A 60×40 image uses 60 × 40 × 3 = 7,200 bytes.
pub fn create_test_jpeg(width: u32, height: u32) -> Vec<u8> {
    // Create an RGB image with a gradient pattern.
    let mut img = image::RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            // Red increases left to right, green increases top to bottom.
            // This gives us a unique color at each position, making it
            // possible to verify pixel locations after transforms.
            let r = ((x as f32 / width.max(1) as f32) * 255.0) as u8;
            let g = ((y as f32 / height.max(1) as f32) * 255.0) as u8;
            img.put_pixel(x, y, image::Rgb([r, g, 0]));
        }
    }

    // Encode as JPEG with quality 95 (high quality to preserve the gradient
    // pattern well — we're testing orientation, not compression artifacts).
    let dyn_img = image::DynamicImage::ImageRgb8(img);
    let mut buffer = Vec::new();
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 95);

    // RUST CONCEPT: `.unwrap()`
    // In test code, `.unwrap()` is fine because a test failure (panic) is
    // exactly what we want if encoding fails. In production code, we'd
    // use `?` or `.map_err()` to handle errors gracefully.
    dyn_img.write_with_encoder(encoder).unwrap();

    buffer
}

/// Inject an EXIF APP1 segment with an orientation tag into a JPEG file.
///
/// This takes an existing JPEG (from `create_test_jpeg`) and inserts EXIF
/// metadata right after the SOI (Start Of Image) marker. The JPEG decoder
/// will read this EXIF data and report the orientation.
///
/// HOW EXIF IS STORED IN JPEG:
/// A JPEG file is a sequence of "markers" — each starts with 0xFF followed
/// by a marker type byte. The structure looks like:
///
///   FF D8                    ← SOI (Start Of Image) — always first
///   FF E1 [length] [data]   ← APP1 (EXIF) — we insert this
///   FF E0 [length] [data]   ← APP0 (JFIF) — original, comes after ours
///   FF C0 [length] [data]   ← SOF (Start Of Frame) — image dimensions
///   FF C4 [length] [data]   ← DHT (Huffman tables)
///   FF DA [length] [data]   ← SOS (Start Of Scan) — pixel data
///   FF D9                    ← EOI (End Of Image)
///
/// The APP1 segment contains:
///   "Exif\0\0"              ← 6-byte header identifying this as EXIF
///   TIFF header             ← Byte order + magic number + IFD offset
///   IFD0                    ← Image File Directory with orientation entry
///
/// EXIF ORIENTATION VALUES:
///   1 = Normal (no rotation)
///   2 = Flip horizontal (mirror left-to-right)
///   3 = Rotate 180° (upside down)
///   4 = Flip vertical (mirror top-to-bottom)
///   5 = Rotate 270° CW + flip horizontal
///   6 = Rotate 90° CW (phone held upright — most common portrait case)
///   7 = Rotate 90° CW + flip horizontal
///   8 = Rotate 270° CW (phone held other way)
pub fn inject_exif_orientation(jpeg_data: &[u8], orientation: u8) -> Vec<u8> {
    // Verify the input is actually a JPEG (starts with SOI marker FF D8).
    assert!(
        jpeg_data.len() >= 2 && jpeg_data[0] == 0xFF && jpeg_data[1] == 0xD8,
        "Input is not a valid JPEG (missing SOI marker FF D8)"
    );

    // Pre-allocate with enough room for the original data + the EXIF segment.
    // The EXIF segment we're adding is 36 bytes:
    //   2 bytes (APP1 marker FF E1) +
    //   2 bytes (segment length) +
    //   32 bytes (EXIF header + TIFF header + IFD)
    let mut result = Vec::with_capacity(jpeg_data.len() + 36);

    // --- SOI marker (Start Of Image) — bytes 0-1 ---
    // Every JPEG starts with FF D8. We keep this at the front.
    result.extend_from_slice(&jpeg_data[0..2]);

    // --- APP1 segment with EXIF orientation ---
    // We construct the minimal valid EXIF structure byte-by-byte.
    // This is a hand-crafted binary blob, but each byte is documented.
    result.extend_from_slice(&[
        // --- APP1 Marker ---
        0xFF,
        0xE1, // APP1 marker (EXIF lives here)
        // --- Segment Length ---
        // 34 bytes total (includes length field bytes, excludes marker bytes).
        // Length = 2 (self) + 6 (Exif header) + 8 (TIFF header) + 18 (IFD) = 34
        0x00,
        0x22,
        // --- EXIF Header ---
        // The string "Exif" followed by two null bytes. This identifies
        // the APP1 segment as containing EXIF data (vs. XMP or other APP1 uses).
        0x45,
        0x78,
        0x69,
        0x66,
        0x00,
        0x00, // "Exif\0\0"
        // --- TIFF Header (Big-Endian) ---
        // EXIF data is stored in TIFF format. The TIFF header specifies
        // the byte order and points to the first IFD (Image File Directory).
        0x4D,
        0x4D, // "MM" = Motorola byte order (big-endian)
        0x00,
        0x2A, // Magic number 42 (the answer to everything, and also TIFF)
        0x00,
        0x00,
        0x00,
        0x08, // Offset to IFD0: 8 bytes from start of TIFF header
        // --- IFD0 (Image File Directory) ---
        // An IFD is an array of 12-byte entries, preceded by a 2-byte count.
        0x00,
        0x01, // Number of directory entries: 1
        // --- IFD Entry: Orientation Tag ---
        // Each IFD entry is exactly 12 bytes:
        //   2 bytes: tag ID
        //   2 bytes: data type
        //   4 bytes: count (number of values)
        //   4 bytes: value (or offset to value if > 4 bytes)
        0x01,
        0x12, // Tag ID: 0x0112 = Orientation
        0x00,
        0x03, // Data type: SHORT (16-bit unsigned integer)
        0x00,
        0x00,
        0x00,
        0x01, // Count: 1 value
        0x00,
        orientation,
        0x00,
        0x00, // Value: the orientation (packed in 4-byte field)
        // --- Next IFD Offset ---
        // 0 means "no more IFDs" (we only need IFD0 for orientation).
        0x00,
        0x00,
        0x00,
        0x00,
    ]);

    // --- Rest of JPEG data (everything after SOI) ---
    // This includes the original APP0/JFIF segment, quantization tables,
    // frame header, Huffman tables, scan data, and EOI marker.
    result.extend_from_slice(&jpeg_data[2..]);

    result
}
