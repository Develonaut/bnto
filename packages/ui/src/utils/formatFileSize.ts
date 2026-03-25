/**
 * Format a byte count into a human-readable string (decimal / SI).
 *
 * Uses k=1000 (SI / decimal) to match macOS Finder and most OS file managers.
 *
 * @example formatFileSize(1000) → "1 KB"
 * @example formatFileSize(2500000) → "2.5 MB"
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1000;
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const value = bytes / Math.pow(k, i);
  return `${parseFloat(value.toFixed(1))} ${units[i]}`;
}
