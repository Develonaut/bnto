/**
 * Create a ZIP archive from browser execution results.
 *
 * Uses fflate (4.8KB gzipped) for fast, pure-JS zip creation.
 * Returns a Blob ready for download via `downloadBlob()`.
 *
 * Handles filename collisions by appending " (2)", " (3)", etc.
 */

import { zipSync } from "fflate";
import type { BrowserFileResult } from "../../types/browser";

/**
 * Bundle multiple file results into a single ZIP blob.
 *
 * @param results - Processed files from browser execution.
 * @returns A Blob containing the ZIP archive.
 */
export async function createZipBlob(
  results: BrowserFileResult[],
): Promise<Blob> {
  // Build a filename → Uint8Array map for fflate.
  // Deduplicate filenames to prevent zip entry collisions.
  const entries: Record<string, Uint8Array> = {};
  const seen = new Map<string, number>();

  for (const result of results) {
    let name = result.filename;

    // Handle duplicate filenames
    const count = seen.get(name) ?? 0;
    if (count > 0) {
      const dot = name.lastIndexOf(".");
      const base = dot > 0 ? name.slice(0, dot) : name;
      const ext = dot > 0 ? name.slice(dot) : "";
      name = `${base} (${count + 1})${ext}`;
    }
    seen.set(result.filename, count + 1);

    // Convert Blob to Uint8Array for fflate
    const buffer = await result.blob.arrayBuffer();
    entries[name] = new Uint8Array(buffer);
  }

  // Create the zip synchronously (fast for small-to-medium files).
  // fflate's zipSync stores files without re-compression by default
  // when given level 0, which is ideal since our files (JPEG, PNG,
  // WebP, CSV) are already compressed or small enough.
  const zipped = zipSync(entries, { level: 0 });

  // Copy into a plain ArrayBuffer for Blob constructor compatibility.
  // fflate's zipSync returns a Uint8Array backed by ArrayBufferLike,
  // which TS considers potentially SharedArrayBuffer.
  const copy = new Uint8Array(zipped.length);
  copy.set(zipped);

  return new Blob([copy.buffer as ArrayBuffer], { type: "application/zip" });
}
