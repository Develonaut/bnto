// Categorize an error message into a telemetry-safe bucket.

const PATTERNS: [RegExp, string][] = [
  [/RuntimeError|CompileError|LinkError/i, "wasm_crash"],
  [/timed?\s*out|timeout/i, "timeout"],
  [/unsupported (format|input|type)/i, "unsupported_format"],
  [/network|failed to fetch|net::/i, "network"],
  [/out of memory|OOM/i, "out_of_memory"],
];

export function categorizeError(error: string): string {
  for (const [pattern, category] of PATTERNS) {
    if (pattern.test(error)) return category;
  }
  return "unknown";
}
