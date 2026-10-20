// Background fill parsing — `transparent` or a CSS-style hex color.

use image::Rgba;

pub(super) const TRANSPARENT: &str = "transparent";

/// Parse `transparent`, `#rgb`, `#rrggbb`, or `#rrggbbaa` into an RGBA pixel.
pub(super) fn parse_background(value: &str) -> Option<Rgba<u8>> {
    let value = value.trim();
    if value.eq_ignore_ascii_case(TRANSPARENT) {
        return Some(Rgba([0, 0, 0, 0]));
    }
    let hex = value.strip_prefix('#')?;
    // All-ASCII hex digits, so byte slicing below can't split a char.
    if !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    match hex.len() {
        3 => Some(short_hex(hex)),
        6 => Some(Rgba([byte(hex, 0), byte(hex, 2), byte(hex, 4), 255])),
        8 => Some(Rgba([
            byte(hex, 0),
            byte(hex, 2),
            byte(hex, 4),
            byte(hex, 6),
        ])),
        _ => None,
    }
}

/// `#f80` is `#ff8800`: each digit doubles.
fn short_hex(hex: &str) -> Rgba<u8> {
    let digit = |i: usize| u8::from_str_radix(&hex[i..=i], 16).unwrap_or(0) * 17;
    Rgba([digit(0), digit(1), digit(2), 255])
}

fn byte(hex: &str, at: usize) -> u8 {
    u8::from_str_radix(&hex[at..at + 2], 16).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transparent_in_any_case() {
        assert_eq!(parse_background("transparent"), Some(Rgba([0, 0, 0, 0])));
        assert_eq!(parse_background(" Transparent "), Some(Rgba([0, 0, 0, 0])));
    }

    #[test]
    fn test_six_digit_hex_is_opaque() {
        assert_eq!(parse_background("#FF8000"), Some(Rgba([255, 128, 0, 255])));
    }

    #[test]
    fn test_three_digit_hex_doubles_each_digit() {
        assert_eq!(parse_background("#f80"), Some(Rgba([255, 136, 0, 255])));
    }

    #[test]
    fn test_eight_digit_hex_carries_alpha() {
        assert_eq!(
            parse_background("#ff800080"),
            Some(Rgba([255, 128, 0, 128]))
        );
    }

    #[test]
    fn test_rejects_anything_else() {
        for bad in [
            "", "red", "ff8000", "#", "#12", "#12345", "#gggggg", "#ff80é0",
        ] {
            assert_eq!(parse_background(bad), None, "{bad:?}");
        }
    }
}
