import { describe, expect, it } from "vitest";

import { renderHookOnce } from "./renderHookOnce";
import { useFormFiles } from "./useFormFiles";

describe("useFormFiles", () => {
  it("returns the provider's input files", () => {
    const files = [new File(["a"], "a.png"), new File(["b"], "b.png")];
    const store = { values: {}, files, onChange: () => {} };
    expect(renderHookOnce(() => useFormFiles(), store)).toBe(files);
  });

  it("returns no files outside a provider", () => {
    expect(renderHookOnce(() => useFormFiles())).toEqual([]);
  });
});
