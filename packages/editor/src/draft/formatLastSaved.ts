/**
 * formatLastSaved — human-readable auto-save status for the file menu.
 *
 * Returns "Edited" when savedAt is null (changes pending, no save yet).
 * Returns "Saved" / "Saved 2m ago" / etc. after a save fires.
 * Always returns a string — the file menu always shows save status.
 *
 * Follows the Google Docs / Figma pattern: brief, non-alarming.
 */

function formatLastSaved(savedAt: number | null, now?: number): string {
  if (savedAt === null) return "Edited";

  const elapsed = (now ?? Date.now()) - savedAt;
  const seconds = Math.floor(elapsed / 1000);

  if (seconds < 60) return "Saved";

  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `Saved ${minutes}m ago`;

  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `Saved ${hours}h ago`;

  const days = Math.floor(hours / 24);
  return `Saved ${days}d ago`;
}

export { formatLastSaved };
