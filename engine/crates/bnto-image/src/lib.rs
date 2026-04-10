// bnto-image — Image processing nodes for the browser (compress, resize, convert).
//
// Each node implements `NodeProcessor` from bnto-core. Files flow as raw bytes
// through decode -> transform -> re-encode, never leaving the user's machine.
//
// Encoding (ImageFormat, encode_image) lives in `bnto-encode` — a shared crate
// that both bnto-image and bnto-vector can depend on without circular deps.

/// Shared helpers for image processors (accepts list, quality param def).
mod common;

/// SVG detection and rasterization (convert-only concern).
pub(crate) mod svg;

pub mod compress;
pub mod convert;
pub mod orientation;
pub mod overlay;
pub mod quantize;
pub mod resize;
pub mod strip_exif;
pub mod wasm_bridge;

#[cfg(test)]
mod test_utils;

// --- Re-exports ---
// Encoding moved to bnto-encode. Re-export here so existing consumers
// (`use bnto_image::encode`, `use bnto_image::ImageFormat`) keep working.

pub use bnto_encode::ImageFormat;
pub use bnto_encode::encode;
pub use bnto_encode::format;

pub use compress::CompressImages;
pub use convert::ConvertImageFormat;
pub use overlay::OverlayImage;
pub use resize::ResizeImages;
pub use strip_exif::StripExif;
