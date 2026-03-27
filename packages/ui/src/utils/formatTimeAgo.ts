const SECOND = 1_000;
const MINUTE = 60 * SECOND;
const HOUR = 60 * MINUTE;
const DAY = 24 * HOUR;
const WEEK = 7 * DAY;
const MONTH = 30 * DAY;
const YEAR = 365 * DAY;

/**
 * Format a timestamp as a human-readable relative time string.
 *
 * Returns "just now" for < 1 minute, then "Xs", "Xm", "Xh", "Xd", "Xw", "Xmo", "Xy".
 *
 * @example formatTimeAgo(Date.now() - 5000) → "5s ago"
 * @example formatTimeAgo(Date.now() - 120000) → "2m ago"
 */
export function formatTimeAgo(timestamp: number, now = Date.now()): string {
  const diff = now - timestamp;

  if (diff < MINUTE) {
    const secs = Math.max(1, Math.floor(diff / SECOND));
    return `${secs}s ago`;
  }
  if (diff < HOUR) return `${Math.floor(diff / MINUTE)}m ago`;
  if (diff < DAY) return `${Math.floor(diff / HOUR)}h ago`;
  if (diff < WEEK) return `${Math.floor(diff / DAY)}d ago`;
  if (diff < MONTH) return `${Math.floor(diff / WEEK)}w ago`;
  if (diff < YEAR) return `${Math.floor(diff / MONTH)}mo ago`;
  return `${Math.floor(diff / YEAR)}y ago`;
}
