// Resize Images Node — change image dimensions with aspect ratio support.
//
// Filter selection: Lanczos3 for downscaling (sharp, 6x6 neighborhood),
// CatmullRom for upscaling (smooth cubic interpolation, avoids ringing).

mod dimensions;
mod param_defs;
mod params;
mod processor;
mod validate;

pub use processor::ResizeImages;
