// Bucket a byte count into a human-readable size range for anonymous telemetry.

const KB = 1_000;
const MB = 1_000_000;

export function bucketFileSize(bytes: number): string {
  if (bytes < 100 * KB) return "<100KB";
  if (bytes < MB) return "100KB-1MB";
  if (bytes < 10 * MB) return "1-10MB";
  if (bytes < 100 * MB) return "10-100MB";
  return ">100MB";
}
