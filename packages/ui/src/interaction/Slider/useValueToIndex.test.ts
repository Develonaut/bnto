import { describe, expect, it } from "vitest";

import { renderHookOnce } from "./renderHookOnce";
import { useValueToIndex } from "./useValueToIndex";

const sorted = [
  { value: 10, label: "Low" },
  { value: 50, label: "Mid" },
  { value: 90, label: "High" },
];

describe("useValueToIndex", () => {
  it("maps a value to the index of the closest preset", () => {
    const toIndex = renderHookOnce(() => useValueToIndex(sorted));
    expect(toIndex(10)).toBe(0);
    expect(toIndex(45)).toBe(1);
    expect(toIndex(200)).toBe(2);
    expect(toIndex(-5)).toBe(0);
  });

  it("picks the lower preset when a value sits halfway between two", () => {
    const toIndex = renderHookOnce(() => useValueToIndex(sorted));
    expect(toIndex(30)).toBe(0);
    expect(toIndex(70)).toBe(1);
  });

  it("returns 0 when there are no presets", () => {
    const toIndex = renderHookOnce(() => useValueToIndex(undefined));
    expect(toIndex(42)).toBe(0);
  });
});
