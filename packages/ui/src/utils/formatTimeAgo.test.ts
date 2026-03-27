import { describe, it, expect } from "vitest";
import { formatTimeAgo } from "./formatTimeAgo";

const SECOND = 1_000;
const MINUTE = 60 * SECOND;
const HOUR = 60 * MINUTE;
const DAY = 24 * HOUR;
const WEEK = 7 * DAY;

const NOW = 1_700_000_000_000;

describe("formatTimeAgo", () => {
  it("returns seconds for < 1 minute", () => {
    expect(formatTimeAgo(NOW - 5 * SECOND, NOW)).toBe("5s ago");
    expect(formatTimeAgo(NOW - 30 * SECOND, NOW)).toBe("30s ago");
    expect(formatTimeAgo(NOW - 500, NOW)).toBe("1s ago");
  });

  it("returns minutes for < 1 hour", () => {
    expect(formatTimeAgo(NOW - MINUTE, NOW)).toBe("1m ago");
    expect(formatTimeAgo(NOW - 5 * MINUTE, NOW)).toBe("5m ago");
    expect(formatTimeAgo(NOW - 59 * MINUTE, NOW)).toBe("59m ago");
  });

  it("returns hours for < 1 day", () => {
    expect(formatTimeAgo(NOW - HOUR, NOW)).toBe("1h ago");
    expect(formatTimeAgo(NOW - 12 * HOUR, NOW)).toBe("12h ago");
  });

  it("returns days for < 1 week", () => {
    expect(formatTimeAgo(NOW - DAY, NOW)).toBe("1d ago");
    expect(formatTimeAgo(NOW - 6 * DAY, NOW)).toBe("6d ago");
  });

  it("returns weeks for < 1 month", () => {
    expect(formatTimeAgo(NOW - WEEK, NOW)).toBe("1w ago");
    expect(formatTimeAgo(NOW - 3 * WEEK, NOW)).toBe("3w ago");
  });

  it("returns months for < 1 year", () => {
    expect(formatTimeAgo(NOW - 30 * DAY, NOW)).toBe("1mo ago");
    expect(formatTimeAgo(NOW - 180 * DAY, NOW)).toBe("6mo ago");
  });

  it("returns years for >= 1 year", () => {
    expect(formatTimeAgo(NOW - 365 * DAY, NOW)).toBe("1y ago");
    expect(formatTimeAgo(NOW - 730 * DAY, NOW)).toBe("2y ago");
  });
});
