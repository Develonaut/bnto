// bnto-vector — SVG rasterization library.
//
// Converts SVG bytes to raster pixels via resvg/usvg/tiny-skia.
// No coupling to the image processing pipeline — this is a pure
// SVG→Pixmap conversion library consumed by bnto-image.

pub mod rasterize;

pub use rasterize::{DEFAULT_DPI, MAX_DPI, MIN_DPI, RasterizeOptions, VectorError, rasterize_svg};
