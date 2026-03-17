// =============================================================================
// Shared Test Utilities for bnto-image
// =============================================================================
//
// Test helpers for creating image fixtures with specific properties.
// Programmatic fixtures are more reliable than stored binaries because:
//   1. The code documents exactly what the fixture contains
//   2. We can create any EXIF orientation without maintaining 8+ files
//   3. Non-square dimensions prove rotation by checking width/height swap

/// Create a JPEG with known dimensions and a gradient pixel pattern.
///
/// Use different width and height values so rotation can be detected by
/// checking dimension swaps (60x40 rotated 90° becomes 40x60).
///
/// Gradient: red increases left->right, green increases top->bottom.
/// This gives unique colors at each position for verifying pixel locations
/// after transforms.
pub fn create_test_jpeg(width: u32, height: u32) -> Vec<u8> {
    let mut img = image::RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let r = ((x as f32 / width.max(1) as f32) * 255.0) as u8;
            let g = ((y as f32 / height.max(1) as f32) * 255.0) as u8;
            img.put_pixel(x, y, image::Rgb([r, g, 0]));
        }
    }

    // High quality (95) to preserve gradient — we're testing orientation,
    // not compression artifacts.
    let dyn_img = image::DynamicImage::ImageRgb8(img);
    let mut buffer = Vec::new();
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 95);
    dyn_img.write_with_encoder(encoder).unwrap();

    buffer
}

/// Inject an EXIF APP1 segment with an orientation tag into a JPEG file.
///
/// Inserts a minimal valid EXIF structure right after the SOI marker.
///
/// EXIF orientation values:
///   1 = Normal, 2 = Flip H, 3 = Rotate 180°, 4 = Flip V
///   5 = Rotate 270° CW + Flip H, 6 = Rotate 90° CW (common portrait)
///   7 = Rotate 90° CW + Flip H, 8 = Rotate 270° CW
pub fn inject_exif_orientation(jpeg_data: &[u8], orientation: u8) -> Vec<u8> {
    assert!(
        jpeg_data.len() >= 2 && jpeg_data[0] == 0xFF && jpeg_data[1] == 0xD8,
        "Input is not a valid JPEG (missing SOI marker FF D8)"
    );

    let mut result = Vec::with_capacity(jpeg_data.len() + 36);

    // SOI marker (Start Of Image) — always first 2 bytes of a JPEG
    result.extend_from_slice(&jpeg_data[0..2]);

    // Minimal valid EXIF APP1 segment with orientation tag.
    //
    // JPEG structure: FF D8 [SOI] -> FF E1 [APP1/EXIF] -> FF E0 [JFIF] -> ...
    // APP1 contains: "Exif\0\0" header -> TIFF header -> IFD with orientation entry
    result.extend_from_slice(&[
        // APP1 marker
        0xFF,
        0xE1,
        // Segment length: 34 bytes (2 self + 6 Exif header + 8 TIFF header + 18 IFD)
        0x00,
        0x22,
        // "Exif\0\0" header
        0x45,
        0x78,
        0x69,
        0x66,
        0x00,
        0x00,
        // TIFF header: "MM" (big-endian) + magic 42 + offset to IFD0
        0x4D,
        0x4D,
        0x00,
        0x2A,
        0x00,
        0x00,
        0x00,
        0x08,
        // IFD0: 1 entry
        0x00,
        0x01,
        // IFD entry: tag 0x0112 (Orientation), type SHORT, count 1, value
        0x01,
        0x12,
        0x00,
        0x03,
        0x00,
        0x00,
        0x00,
        0x01,
        0x00,
        orientation,
        0x00,
        0x00,
        // Next IFD offset: 0 (no more IFDs)
        0x00,
        0x00,
        0x00,
        0x00,
    ]);

    // Rest of JPEG data (everything after SOI)
    result.extend_from_slice(&jpeg_data[2..]);

    result
}
