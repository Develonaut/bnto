import { describe, it, expect } from "vitest";
import { toSlugAggregate } from "./toSlugAggregate";

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
