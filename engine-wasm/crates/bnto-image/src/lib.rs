// =============================================================================
// bnto-image — Image Processing for the Browser
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the entry point for the bnto-image crate. It's like `index.ts` —
// it defines what this crate makes available to the outside world.
//
// WHAT DOES THIS CRATE DO?
// It provides image processing "nodes" that run entirely in the browser:
//   1. compress-images — reduce file size (JPEG quality, PNG optimization)
//   2. (future) resize-images — change dimensions
//   3. (future) convert-image-format — JPEG ↔ PNG ↔ WebP
//
// Each node implements the `NodeProcessor` trait from bnto-core, so the
// Web Worker orchestrator can run them all the same way.
//
// HOW FILES FLOW:
//   1. User drops images in the browser
//   2. Web Worker reads File objects as raw bytes (Vec<u8>)
//   3. We decode the bytes into pixels (using the `image` crate)
//   4. We re-encode the pixels with compression settings
//   5. We return the compressed bytes to the Web Worker
//   6. Web Worker creates a download Blob for the user
//
// Files never leave the user's machine. Zero network. Zero cost.

// --- Modules ---

/// Image format detection — figures out what kind of image we're looking at
/// (JPEG, PNG, WebP) by examining the file extension and magic bytes.
pub mod format;

/// The compress-images node — reduces image file size by re-encoding
/// with optimized settings (JPEG quality, PNG compression level).
pub mod compress;

/// The resize-images node — changes image dimensions using high-quality
/// resampling filters (Lanczos3 for downscale, CatmullRom for upscale).
pub mod resize;

/// WASM bridge — the JavaScript-callable functions that the Web Worker
/// uses to invoke image processing. This is the "door" between JS and Rust.
pub mod wasm_bridge;

// --- Re-exports ---
// So consumers can write `use bnto_image::CompressImages` directly.

pub use compress::CompressImages;
pub use format::ImageFormat;
pub use resize::ResizeImages;
