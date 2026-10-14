import type { BrowserFileResult } from "../types/browser";
import { getOriginalSize } from "./getOriginalSize";

/** Whole percent a result is smaller than its original, or undefined when it isn't smaller. */
function getPercentSmaller(result: BrowserFileResult): number | undefined {
  const original = getOriginalSize(result.metadata);
  if (original == null || original <= 0) return undefined;
  const percent = Math.round((1 - result.blob.size / original) * 100);
  return percent > 0 ? percent : undefined;
}

export { getPercentSmaller };
