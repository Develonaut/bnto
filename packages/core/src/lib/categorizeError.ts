// Categorize an error message into a telemetry-safe bucket.
// Each pattern matches wording browsers or the engine really produce; the
// unit tests pin the verbatim messages seen in `recipe_run_failed` events.

const PATTERNS: [RegExp, string][] = [
  [/RuntimeError|CompileError|LinkError/i, "wasm_crash"],
  [/timed?\s*out|timeout/i, "timeout"],
  [/unsupported (format|input|type)/i, "unsupported_format"],
  [/network|failed to fetch|net::/i, "network"],
  [/out of memory|OOM/i, "out_of_memory"],
  // Safari's wording for a failed fetch (Chrome says "Failed to fetch").
  [/load failed/i, "network"],
  // A dropped file the browser can no longer read (Chrome's NotReadableError).
  [/file could not be read/i, "file_unreadable"],
  // The engine couldn't decode an input (empty, corrupt or unknown format).
  [/failed to decode/i, "decode_failed"],
];

export function categorizeError(error: string): string {
  for (const [pattern, category] of PATTERNS) {
    if (pattern.test(error)) return category;
  }
  return "unknown";
}
