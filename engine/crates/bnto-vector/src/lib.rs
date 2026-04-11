// bnto-vector — SVG rasterization library and processor.
//
// Converts SVG bytes to raster pixels via resvg/usvg/tiny-skia, and provides
// the vector-rasterize NodeProcessor for the engine pipeline.

mod common;
pub mod optimize;
pub mod processor;
pub mod rasterize;

pub use optimize::processor::OptimizeSvg;
pub use processor::VectorRasterize;
pub use rasterize::{DEFAULT_DPI, MAX_DPI, MIN_DPI, RasterizeOptions, VectorError, rasterize_svg};
