import { describe, it, expect } from "vitest";
import { formatLastSaved } from "./formatLastSaved";

describe("formatLastSaved", () => {
  const BASE = 1710000000000;

  it('returns "Edited" for null input', () => {
    expect(formatLastSaved(null)).toBe("Edited");
  });

  it('returns "Saved" for < 60 seconds', () => {
    expect(formatLastSaved(BASE, BASE)).toBe("Saved");
    expect(formatLastSaved(BASE, BASE + 30_000)).toBe("Saved");
    expect(formatLastSaved(BASE, BASE + 59_000)).toBe("Saved");
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
});
