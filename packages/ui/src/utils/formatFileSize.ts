/**
 * Format a byte count into a human-readable string.
 *
 * @example formatFileSize(1024) → "1 KB"
 * @example formatFileSize(2621440) → "2.5 MB"
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const value = bytes / Math.pow(k, i);
  return `${parseFloat(value.toFixed(1))} ${units[i]}`;
}
