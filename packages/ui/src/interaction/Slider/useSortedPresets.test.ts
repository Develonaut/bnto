import { describe, expect, it } from "vitest";

import { renderHookOnce } from "./renderHookOnce";
import { useSortedPresets } from "./useSortedPresets";

const presets = [
  { value: 80, label: "High" },
  { value: 20, label: "Low" },
  { value: 50, label: "Mid" },
];

describe("useSortedPresets", () => {
  it("sorts presets by value, lowest first", () => {
    const sorted = renderHookOnce(() => useSortedPresets(presets));
    expect(sorted?.map((p) => p.label)).toEqual(["Low", "Mid", "High"]);
  });

  it("leaves the caller's array in its original order", () => {
    renderHookOnce(() => useSortedPresets(presets));
    expect(presets.map((p) => p.value)).toEqual([80, 20, 50]);
  });

  it("returns undefined when there are no presets", () => {
    expect(renderHookOnce(() => useSortedPresets(undefined))).toBeUndefined();
    expect(renderHookOnce(() => useSortedPresets([]))).toBeUndefined();
  });
});
