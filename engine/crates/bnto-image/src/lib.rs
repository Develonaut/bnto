// bnto-image — Image processing nodes for the browser (compress, resize, convert).
//
// Each node implements `NodeProcessor` from bnto-core. Files flow as raw bytes
// through decode -> transform -> re-encode, never leaving the user's machine.

/// Shared helpers for image processors (accepts list, quality param def).
mod common;

pub mod compress;
pub mod convert;
pub mod format;
pub mod orientation;
pub mod quantize;
pub mod resize;
pub mod wasm_bridge;

#[cfg(test)]
mod test_utils;

// --- Re-exports ---
// So consumers can write `use bnto_image::CompressImages` directly.

pub use compress::CompressImages;
pub use convert::ConvertImageFormat;
pub use format::ImageFormat;
pub use resize::ResizeImages;
