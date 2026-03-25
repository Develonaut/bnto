import { describe, it, expect } from "vitest";
import { toUsageAnalytics } from "./toUsageAnalytics";

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
