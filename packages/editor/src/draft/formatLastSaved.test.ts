import { describe, it, expect } from "vitest";
import { formatLastSaved } from "./formatLastSaved";

describe("formatLastSaved", () => {
  const BASE = 1710000000000;

  // --- Legacy signature (backward compat) ---

  it('returns "Unsaved changes" for null input', () => {
    expect(formatLastSaved(null)).toBe("Unsaved changes");
  });

  it('returns "Saved just now" for < 10 seconds', () => {
    expect(formatLastSaved(BASE, BASE)).toBe("Saved just now");
    expect(formatLastSaved(BASE, BASE + 5_000)).toBe("Saved just now");
    expect(formatLastSaved(BASE, BASE + 9_000)).toBe("Saved just now");
  });

  it('returns "Saved Ns ago" for 10-59 seconds', () => {
    expect(formatLastSaved(BASE, BASE + 10_000)).toBe("Saved 10s ago");
    expect(formatLastSaved(BASE, BASE + 30_000)).toBe("Saved 30s ago");
    expect(formatLastSaved(BASE, BASE + 59_000)).toBe("Saved 59s ago");
  });

  it('returns "Saved Nm ago" for minutes', () => {
    expect(formatLastSaved(BASE, BASE + 60_000)).toBe("Saved 1m ago");
    expect(formatLastSaved(BASE, BASE + 120_000)).toBe("Saved 2m ago");
    expect(formatLastSaved(BASE, BASE + 59 * 60_000)).toBe("Saved 59m ago");
  });

  it('returns "Saved Nh ago" for hours', () => {
    expect(formatLastSaved(BASE, BASE + 3600_000)).toBe("Saved 1h ago");
    expect(formatLastSaved(BASE, BASE + 2 * 3600_000)).toBe("Saved 2h ago");
    expect(formatLastSaved(BASE, BASE + 23 * 3600_000)).toBe("Saved 23h ago");
  });

  it('returns "Saved Nd ago" for days', () => {
    expect(formatLastSaved(BASE, BASE + 24 * 3600_000)).toBe("Saved 1d ago");
    expect(formatLastSaved(BASE, BASE + 7 * 24 * 3600_000)).toBe("Saved 7d ago");
  });

  // --- Options signature (sync-aware) ---

  it('returns "Syncing\u2026" when isSyncing is true', () => {
    expect(formatLastSaved({ lastSavedAt: BASE, isSyncing: true, now: BASE })).toBe(
      "Syncing\u2026",
    );
  });

  it('returns "Unsaved changes" when lastSavedAt is null (options)', () => {
    expect(formatLastSaved({ lastSavedAt: null })).toBe("Unsaved changes");
  });

  it('returns "Saved locally just now" when cloudId exists but never synced', () => {
    expect(
      formatLastSaved({ lastSavedAt: BASE, cloudId: "abc123", syncedAt: null, now: BASE }),
    ).toBe("Saved locally just now");
  });

  it('returns "Saved locally just now" when local save is newer than sync', () => {
    expect(
      formatLastSaved({
        lastSavedAt: BASE + 5000,
        cloudId: "abc123",
        syncedAt: BASE,
        now: BASE + 5000,
      }),
    ).toBe("Saved locally just now");
  });

  it('returns "Saved just now" when synced and up to date', () => {
    expect(
      formatLastSaved({ lastSavedAt: BASE, cloudId: "abc123", syncedAt: BASE, now: BASE }),
    ).toBe("Saved just now");
  });

  it('returns "Saved just now" when no cloudId (local-only recipe)', () => {
    expect(formatLastSaved({ lastSavedAt: BASE, cloudId: null, now: BASE })).toBe("Saved just now");
  });

  it("formats relative time in options signature", () => {
    expect(
      formatLastSaved({
        lastSavedAt: BASE,
        cloudId: "abc123",
        syncedAt: BASE,
        now: BASE + 120_000,
      }),
    ).toBe("Saved 2m ago");
  });

  it('returns "Saved locally Ns ago" for seconds with cloudId', () => {
    expect(
      formatLastSaved({
        lastSavedAt: BASE,
        cloudId: "abc123",
        syncedAt: null,
        now: BASE + 30_000,
      }),
    ).toBe("Saved locally 30s ago");
  });
});
