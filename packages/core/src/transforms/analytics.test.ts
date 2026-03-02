import { describe, it, expect } from "vitest";
import { toUsageAnalytics, toSlugAggregate } from "./analytics";

describe("toUsageAnalytics", () => {
  it("maps raw analytics doc to UsageAnalytics", () => {
    const result = toUsageAnalytics({
      plan: "free",
      totalRuns: 42,
      lastRunAt: 1700000000000,
    });

    expect(result).toEqual({
      plan: "free",
      totalRuns: 42,
      lastRunAt: 1700000000000,
    });
  });

  it("maps pro plan correctly", () => {
    const result = toUsageAnalytics({
      plan: "pro",
      totalRuns: 500,
      lastRunAt: 1700000000000,
    });

    expect(result.plan).toBe("pro");
  });

  it("maps legacy 'starter' plan to 'free'", () => {
    const result = toUsageAnalytics({
      plan: "starter",
      totalRuns: 10,
      lastRunAt: null,
    });

    expect(result.plan).toBe("free");
  });

  it("preserves null lastRunAt for new users", () => {
    const result = toUsageAnalytics({
      plan: "free",
      totalRuns: 0,
      lastRunAt: null,
    });

    expect(result.lastRunAt).toBeNull();
    expect(result.totalRuns).toBe(0);
  });
});

describe("toSlugAggregate", () => {
  it("maps raw slug aggregate doc", () => {
    const result = toSlugAggregate({
      slug: "compress-images",
      totalRuns: 25,
      completedRuns: 22,
      failedRuns: 3,
      avgDurationMs: 1500,
      lastRunAt: 1700000000000,
    });

    expect(result).toEqual({
      slug: "compress-images",
      totalRuns: 25,
      completedRuns: 22,
      failedRuns: 3,
      avgDurationMs: 1500,
      lastRunAt: 1700000000000,
    });
  });

  it("preserves null avgDurationMs when no durations recorded", () => {
    const result = toSlugAggregate({
      slug: "clean-csv",
      totalRuns: 5,
      completedRuns: 5,
      failedRuns: 0,
      avgDurationMs: null,
      lastRunAt: 1700000000000,
    });

    expect(result.avgDurationMs).toBeNull();
  });
});
