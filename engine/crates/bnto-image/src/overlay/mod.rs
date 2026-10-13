// Image Overlay Node — composite an overlay image onto source images.
//
// Decodes a base64-encoded overlay from params, resizes it relative to
// the source image width, applies opacity, and composites at the chosen
// position with configurable pixel offsets.

mod composite;
mod param_defs;
mod params;
mod positioning;
mod processor;
mod validate;

pub use processor::OverlayImage;
