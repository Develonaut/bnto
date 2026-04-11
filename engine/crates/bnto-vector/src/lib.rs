// bnto-vector — SVG processing library and processors.
//
// Provides vector-rasterize (SVG → raster) and vector-optimize (SVG size
// reduction) NodeProcessors for the engine pipeline.

mod common;
pub mod optimize;
pub mod processor;
pub mod rasterize;

pub use optimize::OptimizeSvg;
pub use processor::VectorRasterize;
pub use rasterize::{DEFAULT_DPI, MAX_DPI, MIN_DPI, RasterizeOptions, VectorError, rasterize_svg};
