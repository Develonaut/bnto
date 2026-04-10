// bnto-encode — Shared image format detection and encoding.
//
// Extracted from bnto-image so that both bnto-image and bnto-vector can
// encode raster output without circular dependencies.

pub mod encode;
pub mod format;

pub use encode::encode_image;
pub use format::ImageFormat;
