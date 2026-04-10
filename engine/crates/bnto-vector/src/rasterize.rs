// SVG rasterization — parse SVG bytes and render to a pixel buffer.

use thiserror::Error;

/// Errors from SVG parsing and rasterization.
#[derive(Debug, Error)]
pub enum VectorError {
    #[error("failed to parse SVG: {0}")]
    ParseError(String),

    #[error("failed to render SVG to pixels")]
    RenderError,

    #[error("invalid DPI {0}: must be between {MIN_DPI} and {MAX_DPI}")]
    InvalidDpi(u32),
}

/// Minimum allowed DPI for rasterization.
pub const MIN_DPI: u32 = 72;
/// Maximum allowed DPI for rasterization.
pub const MAX_DPI: u32 = 300;
/// Default DPI — matches SVG spec's 1:1 mapping (96 CSS px per inch).
pub const DEFAULT_DPI: u32 = 96;

/// Options for SVG rasterization.
#[derive(Debug, Clone)]
pub struct RasterizeOptions {
    /// Output DPI. Default 96 (1:1 with SVG units). Higher = larger output.
    pub dpi: u32,
}

impl Default for RasterizeOptions {
    fn default() -> Self {
        Self { dpi: DEFAULT_DPI }
    }
}

/// Rasterize SVG bytes to a pixel buffer.
///
/// Parses the SVG via usvg, scales to the requested DPI, and renders
/// via resvg into a `tiny_skia::Pixmap` (RGBA premultiplied).
pub fn rasterize_svg(
    svg_bytes: &[u8],
    options: RasterizeOptions,
) -> Result<tiny_skia::Pixmap, VectorError> {
    if options.dpi < MIN_DPI || options.dpi > MAX_DPI {
        return Err(VectorError::InvalidDpi(options.dpi));
    }

    let svg_str =
        std::str::from_utf8(svg_bytes).map_err(|e| VectorError::ParseError(e.to_string()))?;

    let tree = usvg::Tree::from_str(svg_str, &usvg::Options::default())
        .map_err(|e| VectorError::ParseError(e.to_string()))?;

    let scale = options.dpi as f32 / DEFAULT_DPI as f32;
    let size = tree.size();
    let width = (size.width() * scale).ceil() as u32;
    let height = (size.height() * scale).ceil() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or(VectorError::RenderError)?;

    let transform = tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    Ok(pixmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_svg() -> Vec<u8> {
        std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../test-fixtures/images/small.svg"
        ))
        .expect("small.svg fixture must exist")
    }

    #[test]
    fn default_dpi_produces_100x100() {
        let pixmap = rasterize_svg(&fixture_svg(), RasterizeOptions::default()).unwrap();
        assert_eq!(pixmap.width(), 100);
        assert_eq!(pixmap.height(), 100);
    }

    #[test]
    fn custom_dpi_192_doubles_dimensions() {
        let opts = RasterizeOptions { dpi: 192 };
        let pixmap = rasterize_svg(&fixture_svg(), opts).unwrap();
        assert_eq!(pixmap.width(), 200);
        assert_eq!(pixmap.height(), 200);
    }

    #[test]
    fn min_dpi_72_accepted() {
        let opts = RasterizeOptions { dpi: 72 };
        let pixmap = rasterize_svg(&fixture_svg(), opts).unwrap();
        assert_eq!(pixmap.width(), 75); // 100 * 72/96 = 75
        assert_eq!(pixmap.height(), 75);
    }

    #[test]
    fn max_dpi_300_accepted() {
        let opts = RasterizeOptions { dpi: 300 };
        let pixmap = rasterize_svg(&fixture_svg(), opts).unwrap();
        // 100 * 300/96 = 312.5 -> ceil = 313
        assert_eq!(pixmap.width(), 313);
        assert_eq!(pixmap.height(), 313);
    }

    #[test]
    fn dpi_below_min_rejected() {
        let opts = RasterizeOptions { dpi: 71 };
        let err = rasterize_svg(&fixture_svg(), opts).unwrap_err();
        assert!(matches!(err, VectorError::InvalidDpi(71)));
    }

    #[test]
    fn dpi_above_max_rejected() {
        let opts = RasterizeOptions { dpi: 301 };
        let err = rasterize_svg(&fixture_svg(), opts).unwrap_err();
        assert!(matches!(err, VectorError::InvalidDpi(301)));
    }

    #[test]
    fn invalid_svg_returns_parse_error() {
        let err = rasterize_svg(b"not valid svg", RasterizeOptions::default()).unwrap_err();
        assert!(matches!(err, VectorError::ParseError(_)));
    }

    #[test]
    fn non_utf8_returns_parse_error() {
        let err = rasterize_svg(&[0xFF, 0xFE, 0x00], RasterizeOptions::default()).unwrap_err();
        assert!(matches!(err, VectorError::ParseError(_)));
    }

    #[test]
    fn empty_data_returns_error() {
        let err = rasterize_svg(b"", RasterizeOptions::default()).unwrap_err();
        assert!(matches!(err, VectorError::ParseError(_)));
    }

    #[test]
    fn pixmap_has_non_zero_data() {
        let pixmap = rasterize_svg(&fixture_svg(), RasterizeOptions::default()).unwrap();
        // The fixture has a blue rect + orange circle — pixels shouldn't all be transparent
        let has_visible_pixels = pixmap.data().iter().any(|&b| b != 0);
        assert!(
            has_visible_pixels,
            "rendered pixmap should contain visible pixels"
        );
    }

    // --- Complex real-world SVG (mascot illustration from Figma) ---

    fn fixture_mascot() -> Vec<u8> {
        std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../test-fixtures/images/mascot-sushi-friends.svg"
        ))
        .expect("mascot-sushi-friends.svg fixture must exist")
    }

    #[test]
    fn complex_svg_rasterizes_at_default_dpi() {
        // 2144x1569 mascot SVG with paths, opacity groups, and fills
        let pixmap = rasterize_svg(&fixture_mascot(), RasterizeOptions::default()).unwrap();
        assert_eq!(pixmap.width(), 2144);
        assert_eq!(pixmap.height(), 1569);
        let has_visible_pixels = pixmap.data().iter().any(|&b| b != 0);
        assert!(has_visible_pixels, "mascot should render visible pixels");
    }

    #[test]
    fn complex_svg_scales_with_dpi() {
        let opts = RasterizeOptions { dpi: 192 };
        let pixmap = rasterize_svg(&fixture_mascot(), opts).unwrap();
        // 2x scale: 2144 * 2 = 4288, 1569 * 2 = 3138
        assert_eq!(pixmap.width(), 4288);
        assert_eq!(pixmap.height(), 3138);
    }
}
