/**
 * formatLastSaved — human-readable relative time for the "Last saved" label.
 *
 * Returns null if savedAt is null (never saved).
 * Uses a simple threshold approach — no i18n library needed.
 */

function formatLastSaved(savedAt: number | null, now?: number): string | null {
  if (savedAt === null) return null;

  const elapsed = (now ?? Date.now()) - savedAt;
  const seconds = Math.floor(elapsed / 1000);

  if (seconds < 60) return "just now";

  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${minutes}m ago`;

  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h ago`;

  const days = Math.floor(hours / 24);
  return `${days}d ago`;
}

export { formatLastSaved };
