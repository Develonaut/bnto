// Pad Images Node — center each image on a canvas of a set size.
//
// Pairs with image-resize: resize fits an image inside a box, pad grows the
// canvas to the full box (e.g. a 512x451 logo onto a 512x512 square).

mod canvas;
mod color;
mod param_defs;
mod params;
mod processor;
mod validate;

pub use processor::PadImages;

#[cfg(test)]
mod tests;
