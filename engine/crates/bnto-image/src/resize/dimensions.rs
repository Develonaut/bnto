// Dimension math — target size resolution, bounds checks, and filter choice.

use bnto_core::errors::BntoError;
use image::imageops::FilterType;

use super::processor::ResizeImages;

/// Max dimension — 16384px is the practical limit for WASM memory.
pub(crate) const MAX_DIMENSION: u32 = 16384;
pub(crate) const MIN_DIMENSION: u32 = 1;

impl ResizeImages {
    /// Calculate final (width, height) from params and original dimensions.
    /// With `maintain_aspect`, a single dimension derives the other from the ratio,
    /// and both dimensions fit the image inside the box (like CSS `object-fit: contain`).
    pub(super) fn calculate_dimensions(
        target_width: Option<u32>,
        target_height: Option<u32>,
        original_width: u32,
        original_height: u32,
        maintain_aspect: bool,
    ) -> Result<(u32, u32), BntoError> {
        validate_inputs(target_width, target_height, original_width, original_height)?;
        let (final_width, final_height) = match (target_width, target_height) {
            (Some(w), Some(h)) => {
                resolve_both(w, h, original_width, original_height, maintain_aspect)
            }
            (Some(w), None) => {
                resolve_width_only(w, original_width, original_height, maintain_aspect)
            }
            (None, Some(h)) => {
                resolve_height_only(h, original_width, original_height, maintain_aspect)
            }
            (None, None) => unreachable!(),
        };
        validate_final_dimensions(final_width, final_height)?;
        Ok((final_width, final_height))
    }

    /// Lanczos3 for downscaling (sharp), CatmullRom for upscaling (smooth).
    pub(super) fn choose_filter(original_pixels: u32, target_pixels: u32) -> FilterType {
        if target_pixels < original_pixels {
            FilterType::Lanczos3
        } else {
            FilterType::CatmullRom
        }
    }
}

/// Reject requests with no target dimension or a zero-sized original.
fn validate_inputs(
    target_width: Option<u32>,
    target_height: Option<u32>,
    original_width: u32,
    original_height: u32,
) -> Result<(), BntoError> {
    if target_width.is_none() && target_height.is_none() {
        return Err(BntoError::InvalidInput(
            "At least one dimension (width or height) must be specified".to_string(),
        ));
    }
    if original_width == 0 || original_height == 0 {
        return Err(BntoError::InvalidInput(
            "Original image has zero dimensions".to_string(),
        ));
    }
    Ok(())
}

/// Fit inside the w×h box: the side that hits its limit first is exact, the other
/// follows the ratio. Cross-multiplying in u64 picks that side without float error.
fn resolve_both(w: u32, h: u32, orig_w: u32, orig_h: u32, maintain_aspect: bool) -> (u32, u32) {
    if !maintain_aspect {
        return (w, h);
    }
    if w as u64 * orig_h as u64 <= h as u64 * orig_w as u64 {
        resolve_width_only(w, orig_w, orig_h, true)
    } else {
        resolve_height_only(h, orig_w, orig_h, true)
    }
}

fn resolve_width_only(w: u32, orig_w: u32, orig_h: u32, maintain_aspect: bool) -> (u32, u32) {
    if maintain_aspect {
        let ratio = orig_h as f64 / orig_w as f64;
        let h = (w as f64 * ratio).round() as u32;
        (w, h.max(MIN_DIMENSION))
    } else {
        (w, orig_h)
    }
}

fn resolve_height_only(h: u32, orig_w: u32, orig_h: u32, maintain_aspect: bool) -> (u32, u32) {
    if maintain_aspect {
        let ratio = orig_w as f64 / orig_h as f64;
        let w = (h as f64 * ratio).round() as u32;
        (w.max(MIN_DIMENSION), h)
    } else {
        (orig_w, h)
    }
}

fn validate_final_dimensions(width: u32, height: u32) -> Result<(), BntoError> {
    if width > MAX_DIMENSION || height > MAX_DIMENSION {
        return Err(BntoError::InvalidInput(format!(
            "Target dimensions {}x{} exceed maximum allowed {}x{}",
            width, height, MAX_DIMENSION, MAX_DIMENSION
        )));
    }
    if width < MIN_DIMENSION || height < MIN_DIMENSION {
        return Err(BntoError::InvalidInput(format!(
            "Target dimensions {}x{} are below minimum {}x{}",
            width, height, MIN_DIMENSION, MIN_DIMENSION
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Dimension Calculation Tests — Pure Logic, No Image Processing
    // =========================================================================

    #[test]
    fn test_calculate_dimensions_both_specified_matching_ratio() {
        // 300×200 box has the same 3:2 ratio as 1200×800, so it fills exactly.
        let result = ResizeImages::calculate_dimensions(Some(300), Some(200), 1200, 800, true);
        assert_eq!(result.unwrap(), (300, 200));
    }

    #[test]
    fn test_calculate_dimensions_both_with_aspect_fits_width() {
        // Square into a wide 50×20 box: height is the tight side → 20×20.
        let result = ResizeImages::calculate_dimensions(Some(50), Some(20), 100, 100, true);
        assert_eq!(result.unwrap(), (20, 20));
    }

    #[test]
    fn test_calculate_dimensions_both_with_aspect_fits_height() {
        // 1200×800 into a 300×300 box: width is the tight side → 300×200.
        let result = ResizeImages::calculate_dimensions(Some(300), Some(300), 1200, 800, true);
        assert_eq!(result.unwrap(), (300, 200));
    }

    #[test]
    fn test_calculate_dimensions_both_with_aspect_upscales_to_fit() {
        // Small image grows until one side touches the box: 100×50 → 400×200.
        let result = ResizeImages::calculate_dimensions(Some(400), Some(400), 100, 50, true);
        assert_eq!(result.unwrap(), (400, 200));
    }

    #[test]
    fn test_calculate_dimensions_both_with_aspect_favicon_svg() {
        // The 3117×2744 favicon SVG into a 512×512 box: 512 × 2744/3117 = 450.7 → 451.
        let result = ResizeImages::calculate_dimensions(Some(512), Some(512), 3117, 2744, true);
        assert_eq!(result.unwrap(), (512, 451));
    }

    #[test]
    fn test_calculate_dimensions_both_with_aspect_minimum_1px() {
        // 1000×1 into a 2×2 box: height 0.002 rounds to 0 → clamped to 1.
        let result = ResizeImages::calculate_dimensions(Some(2), Some(2), 1000, 1, true);
        assert_eq!(result.unwrap(), (2, 1));
    }

    #[test]
    fn test_calculate_dimensions_both_no_aspect_stretches() {
        // maintainAspect=false keeps the exact box, distorting the image.
        let result = ResizeImages::calculate_dimensions(Some(50), Some(20), 100, 100, false);
        assert_eq!(result.unwrap(), (50, 20));
    }

    #[test]
    fn test_calculate_dimensions_width_only_with_aspect() {
        // Width=600, original=1200×800 → height = 600 × (800/1200) = 400
        let result = ResizeImages::calculate_dimensions(Some(600), None, 1200, 800, true);
        assert_eq!(result.unwrap(), (600, 400));
    }

    #[test]
    fn test_calculate_dimensions_height_only_with_aspect() {
        // Height=400, original=1200×800 → width = 400 × (1200/800) = 600
        let result = ResizeImages::calculate_dimensions(None, Some(400), 1200, 800, true);
        assert_eq!(result.unwrap(), (600, 400));
    }

    #[test]
    fn test_calculate_dimensions_width_only_no_aspect() {
        // Width=600, maintainAspect=false → height stays at original (800)
        let result = ResizeImages::calculate_dimensions(Some(600), None, 1200, 800, false);
        assert_eq!(result.unwrap(), (600, 800));
    }

    #[test]
    fn test_calculate_dimensions_height_only_no_aspect() {
        // Height=400, maintainAspect=false → width stays at original (1200)
        let result = ResizeImages::calculate_dimensions(None, Some(400), 1200, 800, false);
        assert_eq!(result.unwrap(), (1200, 400));
    }

    #[test]
    fn test_calculate_dimensions_neither_specified_is_error() {
        // No width or height — that's an error.
        let result = ResizeImages::calculate_dimensions(None, None, 1200, 800, true);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("At least one dimension"));
    }

    #[test]
    fn test_calculate_dimensions_zero_original_is_error() {
        // Original image has zero dimensions — can't compute aspect ratio.
        let result = ResizeImages::calculate_dimensions(Some(100), None, 0, 800, true);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("zero dimensions"));
    }

    #[test]
    fn test_calculate_dimensions_exceeds_max_is_error() {
        // Target exceeds MAX_DIMENSION (16384).
        let result = ResizeImages::calculate_dimensions(Some(20000), Some(20000), 100, 100, false);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("exceed maximum"));
    }

    #[test]
    fn test_calculate_dimensions_aspect_ratio_rounds_correctly() {
        // 1920×1080 resized to width=1280
        // height = 1280 × (1080/1920) = 1280 × 0.5625 = 720.0 (exact)
        let result = ResizeImages::calculate_dimensions(Some(1280), None, 1920, 1080, true);
        assert_eq!(result.unwrap(), (1280, 720));
    }

    #[test]
    fn test_calculate_dimensions_aspect_ratio_rounds_up() {
        // 1000×333 resized to width=500
        // height = 500 × (333/1000) = 500 × 0.333 = 166.5 → rounds to 167
        let result = ResizeImages::calculate_dimensions(Some(500), None, 1000, 333, true);
        let (_, h) = result.unwrap();
        assert_eq!(h, 167); // rounded from 166.5
    }

    #[test]
    fn test_calculate_dimensions_tiny_aspect_minimum_1px() {
        // 1000×1 resized to width=2
        // height = 2 × (1/1000) = 0.002 → rounds to 0 → clamped to 1
        let result = ResizeImages::calculate_dimensions(Some(2), None, 1000, 1, true);
        let (w, h) = result.unwrap();
        assert_eq!(w, 2);
        assert_eq!(h, 1); // minimum 1px
    }

    // =========================================================================
    // Filter Selection Tests
    // =========================================================================

    #[test]
    fn test_choose_filter_downscale_uses_lanczos3() {
        // Shrinking: use Lanczos3 for best quality
        let filter = ResizeImages::choose_filter(1000 * 1000, 500 * 500);
        assert_eq!(filter, FilterType::Lanczos3);
    }

    #[test]
    fn test_choose_filter_upscale_uses_catmullrom() {
        // Enlarging: use CatmullRom for smooth interpolation
        let filter = ResizeImages::choose_filter(500 * 500, 1000 * 1000);
        assert_eq!(filter, FilterType::CatmullRom);
    }

    #[test]
    fn test_choose_filter_same_size_uses_catmullrom() {
        // Same size: CatmullRom (upscale path — cheaper, same result)
        let filter = ResizeImages::choose_filter(500 * 500, 500 * 500);
        assert_eq!(filter, FilterType::CatmullRom);
    }
}
