import { describe, it, expect } from "vitest";
import { formatTimeAgo } from "../formatTimeAgo";

const MINUTE = 60_000;
const HOUR = 60 * MINUTE;
const DAY = 24 * HOUR;
const WEEK = 7 * DAY;

const NOW = 1_700_000_000_000; // Fixed reference point

describe("formatTimeAgo", () => {
  it("returns 'just now' for < 1 minute", () => {
    expect(formatTimeAgo(NOW - 30_000, NOW)).toBe("just now");
    expect(formatTimeAgo(NOW - 0, NOW)).toBe("just now");
    expect(formatTimeAgo(NOW - 59_999, NOW)).toBe("just now");
  });

  it("returns minutes for < 1 hour", () => {
    expect(formatTimeAgo(NOW - MINUTE, NOW)).toBe("1m");
    expect(formatTimeAgo(NOW - 5 * MINUTE, NOW)).toBe("5m");
    expect(formatTimeAgo(NOW - 59 * MINUTE, NOW)).toBe("59m");
  });

  it("returns hours for < 1 day", () => {
    expect(formatTimeAgo(NOW - HOUR, NOW)).toBe("1h");
    expect(formatTimeAgo(NOW - 12 * HOUR, NOW)).toBe("12h");
    expect(formatTimeAgo(NOW - 23 * HOUR, NOW)).toBe("23h");
  });

  it("returns days for < 1 week", () => {
    expect(formatTimeAgo(NOW - DAY, NOW)).toBe("1d");
    expect(formatTimeAgo(NOW - 6 * DAY, NOW)).toBe("6d");
  });

  it("returns weeks for < 1 month", () => {
    expect(formatTimeAgo(NOW - WEEK, NOW)).toBe("1w");
    expect(formatTimeAgo(NOW - 3 * WEEK, NOW)).toBe("3w");
  });

  it("returns months for < 1 year", () => {
    expect(formatTimeAgo(NOW - 30 * DAY, NOW)).toBe("1mo");
    expect(formatTimeAgo(NOW - 180 * DAY, NOW)).toBe("6mo");
  });

  it("returns years for >= 1 year", () => {
    expect(formatTimeAgo(NOW - 365 * DAY, NOW)).toBe("1y");
    expect(formatTimeAgo(NOW - 730 * DAY, NOW)).toBe("2y");
  });
});
