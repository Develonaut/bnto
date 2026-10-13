// Overlay placement — gravity regions with a 2% edge margin plus pixel offsets.

/// Calculate top-left (x, y) for placing the overlay on the source image.
///
/// Position names are gravity regions: "top-left" places the overlay
/// fully visible in the top-left corner with a 2% margin from the edges.
/// Offsets shift from there: positive = right/down, negative = left/up.
pub(super) fn calculate_position(
    position: &str,
    src_w: u32,
    src_h: u32,
    wm_w: u32,
    wm_h: u32,
    offset_x: i64,
    offset_y: i64,
) -> (i64, i64) {
    let x = gravity_x(position, src_w, wm_w);
    let y = gravity_y(position, src_h, wm_h);
    (x + offset_x, y + offset_y)
}

/// 2% margin from edges, proportional to the axis length.
fn edge_margin(src_len: u32) -> i64 {
    (src_len as f64 * 0.02) as i64
}

fn gravity_x(position: &str, src_w: u32, wm_w: u32) -> i64 {
    let (sw, ww) = (src_w as i64, wm_w as i64);
    match position {
        "top-left" | "middle-left" | "bottom-left" => edge_margin(src_w),
        "top-right" | "middle-right" | "bottom-right" => sw - ww - edge_margin(src_w),
        _ => (sw - ww) / 2, // center column
    }
}

fn gravity_y(position: &str, src_h: u32, wm_h: u32) -> i64 {
    let (sh, wh) = (src_h as i64, wm_h as i64);
    match position {
        "top-left" | "top-center" | "top-right" => edge_margin(src_h),
        "bottom-left" | "bottom-center" | "bottom-right" => sh - wh - edge_margin(src_h),
        _ => (sh - wh) / 2, // middle row
    }
}
