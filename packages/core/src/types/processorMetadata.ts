// ---------------------------------------------------------------------------
// Processor Metadata — Typed stats from the Rust WASM pipeline
//
// The engine attaches metadata to each processed file (compression ratio,
// original size, etc.) as a JSON object. This interface types the known
// fields so UI consumers don't need unsafe `as` casts.
//
// The index signature preserves extensibility — processors can attach
// fields the TypeScript side hasn't typed yet.
// ---------------------------------------------------------------------------

/** Typed metadata from the Rust WASM engine's processor output. */
export interface ProcessorMetadata {
  /** Original file size in bytes (before processing). */
  originalSize?: number;
  /** Compressed/processed file size in bytes. */
  compressedSize?: number;
  /** Compression ratio as a percentage (0-100). Higher = more compression. */
  compressionRatio?: number;

  /** Allow additional processor-specific fields. */
  [key: string]: unknown;
}
