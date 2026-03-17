// bnto-image — Image processing nodes for browser-side execution (WASM).
// Provides compress, resize, and convert nodes. Each implements `NodeProcessor`.

pub mod compress;
pub mod convert;
pub mod format;
pub mod orientation;
pub mod resize;

/// PNG color quantization — lossy palette reduction (24-bit to 8-bit indexed)
/// using quantizr (median cut + Floyd-Steinberg dithering). Same technique as
/// TinyPNG: ~57% reduction on photographic PNGs.
pub mod quantize;

/// WASM bridge — JS-callable functions the Web Worker uses to invoke processing.
pub mod wasm_bridge;

/// Shared test utilities for creating fixtures with specific properties.
#[cfg(test)]
mod test_utils;

// --- Re-exports ---

pub use compress::CompressImages;
pub use convert::ConvertImageFormat;
pub use format::ImageFormat;
pub use resize::ResizeImages;
