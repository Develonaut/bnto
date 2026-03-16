import { describe, it, expect } from "vitest";
import { formatLastSaved } from "./formatLastSaved";

describe("formatLastSaved", () => {
  const BASE = 1710000000000;

  it("returns null for null input", () => {
    expect(formatLastSaved(null)).toBeNull();
  });

  it('returns "just now" for < 60 seconds', () => {
    expect(formatLastSaved(BASE, BASE)).toBe("just now");
    expect(formatLastSaved(BASE, BASE + 30_000)).toBe("just now");
    expect(formatLastSaved(BASE, BASE + 59_000)).toBe("just now");
  });

  it('returns "Nm ago" for minutes', () => {
    expect(formatLastSaved(BASE, BASE + 60_000)).toBe("1m ago");
    expect(formatLastSaved(BASE, BASE + 120_000)).toBe("2m ago");
    expect(formatLastSaved(BASE, BASE + 59 * 60_000)).toBe("59m ago");
  });

  it('returns "Nh ago" for hours', () => {
    expect(formatLastSaved(BASE, BASE + 3600_000)).toBe("1h ago");
    expect(formatLastSaved(BASE, BASE + 2 * 3600_000)).toBe("2h ago");
    expect(formatLastSaved(BASE, BASE + 23 * 3600_000)).toBe("23h ago");
  });

  it('returns "Nd ago" for days', () => {
    expect(formatLastSaved(BASE, BASE + 24 * 3600_000)).toBe("1d ago");
    expect(formatLastSaved(BASE, BASE + 7 * 24 * 3600_000)).toBe("7d ago");
  });
});
