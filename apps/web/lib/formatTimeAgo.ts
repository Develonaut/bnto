const MINUTE = 60_000;
const HOUR = 60 * MINUTE;
const DAY = 24 * HOUR;
const WEEK = 7 * DAY;
const MONTH = 30 * DAY;
const YEAR = 365 * DAY;

/**
 * Format a timestamp as a human-readable relative time string.
 *
 * Returns "just now" for < 1 minute, then "Xm", "Xh", "Xd", "Xw", "Xmo", "Xy".
 */
export function formatTimeAgo(timestamp: number, now = Date.now()): string {
  const diff = now - timestamp;

  if (diff < MINUTE) return "just now";
  if (diff < HOUR) return `${Math.floor(diff / MINUTE)}m`;
  if (diff < DAY) return `${Math.floor(diff / HOUR)}h`;
  if (diff < WEEK) return `${Math.floor(diff / DAY)}d`;
  if (diff < MONTH) return `${Math.floor(diff / WEEK)}w`;
  if (diff < YEAR) return `${Math.floor(diff / MONTH)}mo`;
  return `${Math.floor(diff / YEAR)}y`;
}
