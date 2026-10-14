import type { BrowserFileResult } from "../types/browser";
import { getOriginalSize } from "./getOriginalSize";

/**
 * Sum the bytes saved across a set of browser execution results.
 *
 * Returns 0 when no results have original size metadata (e.g. rename-files).
 */
function computeTotalSaved(results: BrowserFileResult[]): number {
  return results.reduce((acc, r) => {
    const orig = getOriginalSize(r.metadata);
    return orig != null ? acc + (orig - r.blob.size) : acc;
  }, 0);
}

export { computeTotalSaved };
