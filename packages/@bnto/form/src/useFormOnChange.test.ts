import { describe, expect, it, vi } from "vitest";

import { renderHookOnce } from "./renderHookOnce";
import { useFormOnChange } from "./useFormOnChange";

describe("useFormOnChange", () => {
  it("returns the provider's change handler", () => {
    const onChange = vi.fn();
    const store = { values: {}, files: [], onChange };
    const formOnChange = renderHookOnce(() => useFormOnChange(), store);
    expect(formOnChange).toBe(onChange);
    formOnChange("offsetX", 0);
    expect(onChange).toHaveBeenCalledWith("offsetX", 0);
  });

  it("returns a handler that does nothing outside a provider", () => {
    const formOnChange = renderHookOnce(() => useFormOnChange());
    expect(() => formOnChange("offsetX", 0)).not.toThrow();
  });
});
