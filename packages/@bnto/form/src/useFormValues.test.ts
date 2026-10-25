import { describe, expect, it } from "vitest";

import { renderHookOnce } from "./renderHookOnce";
import { useFormValues } from "./useFormValues";

describe("useFormValues", () => {
  it("returns the provider's values map", () => {
    const values = { size: 25, opacity: 80 };
    const store = { values, files: [], onChange: () => {} };
    expect(renderHookOnce(() => useFormValues(), store)).toBe(values);
  });

  it("returns an empty map outside a provider", () => {
    expect(renderHookOnce(() => useFormValues())).toEqual({});
  });
});
