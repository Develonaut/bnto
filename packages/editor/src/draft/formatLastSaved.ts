/**
 * formatLastSaved — human-readable auto-save status for the file menu.
 *
 * Returns differentiated status based on sync state:
 * - "Syncing..." — Convex sync in flight
 * - "Saved locally just now" / "Saved locally Nm ago" — has cloudId but not yet synced
 * - "Saved just now" / "Saved Nm ago" — synced to cloud, or local-only recipe
 * - "Unsaved changes" — dirty, autosave hasn't fired yet
 *
 * Always shows relative time so the user knows when the last save happened.
 */

interface FormatLastSavedOptions {
  lastSavedAt: number | null;
  isSyncing?: boolean;
  cloudId?: string | null;
  syncedAt?: number | null;
  now?: number;
}

function formatRelative(prefix: string, savedAt: number, now: number): string {
  const elapsed = now - savedAt;
  const seconds = Math.floor(elapsed / 1000);

  if (seconds < 10) return `${prefix} just now`;
  if (seconds < 60) return `${prefix} ${seconds}s ago`;

  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${prefix} ${minutes}m ago`;

  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${prefix} ${hours}h ago`;

  const days = Math.floor(hours / 24);
  return `${prefix} ${days}d ago`;
}

function formatLastSaved(opts: FormatLastSavedOptions): string;
function formatLastSaved(savedAt: number | null, now?: number): string;
function formatLastSaved(
  optsOrSavedAt: FormatLastSavedOptions | number | null,
  nowArg?: number,
): string {
  // Legacy signature: formatLastSaved(savedAt, now?)
  if (optsOrSavedAt === null || typeof optsOrSavedAt === "number") {
    const savedAt = optsOrSavedAt;
    if (savedAt === null) return "Unsaved changes";
    return formatRelative("Saved", savedAt, nowArg ?? Date.now());
  }

  // Options signature
  const { lastSavedAt, isSyncing = false, cloudId, syncedAt, now = Date.now() } = optsOrSavedAt;

  if (isSyncing) return "Syncing\u2026";
  if (lastSavedAt === null) return "Unsaved changes";

  // Has cloudId but never synced, or local save is newer than sync
  if (cloudId && (syncedAt == null || lastSavedAt > syncedAt)) {
    return formatRelative("Saved locally", lastSavedAt, now);
  }

  return formatRelative("Saved", lastSavedAt, now);
}

export { formatLastSaved };
export type { FormatLastSavedOptions };
