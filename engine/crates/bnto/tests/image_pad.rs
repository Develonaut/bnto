// image-pad through the built binary — a wide SVG becomes a square PNG icon
// by composing image-convert, image-resize (fit the width), and image-pad.

mod helpers;

use helpers::{fixture_image, output_files, run_custom_recipe_ok};

/// PNG width, height, and color type, read from the IHDR chunk.
fn png_header(data: &[u8]) -> (u32, u32, u8) {
    assert_eq!(&data[1..4], b"PNG", "not a PNG");
    let be = |at: usize| u32::from_be_bytes(data[at..at + 4].try_into().unwrap());
    (be(16), be(20), data[25])
}

#[test]
fn test_square_icon_pads_a_wide_svg_to_a_square_png() {
    // 2144x1569: fits to 64x47, then pads to 64x64.
    let (out, stderr) =
        run_custom_recipe_ok("square-icon", &fixture_image("mascot-sushi-friends.svg"));
    let files = output_files(&out);
    assert_eq!(files.len(), 1, "stderr: {stderr}");
    assert_eq!(
        files[0].file_name().to_string_lossy(),
        "mascot-sushi-friends-resized-padded.png"
    );

    let data = std::fs::read(files[0].path()).unwrap();
    const RGBA: u8 = 6;
    assert_eq!(png_header(&data), (64, 64, RGBA), "stderr: {stderr}");
}
