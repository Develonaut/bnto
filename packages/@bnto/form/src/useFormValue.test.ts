import { describe, expect, it } from "vitest";

import { renderHookOnce } from "./renderHookOnce";
import { useFormValue } from "./useFormValue";

const store = { values: { size: 25, overlay: "data:x" }, files: [], onChange: () => {} };

describe("useFormValue", () => {
  it("reads one value from the provider by param name", () => {
    expect(renderHookOnce(() => useFormValue("size"), store)).toBe(25);
    expect(renderHookOnce(() => useFormValue("overlay"), store)).toBe("data:x");
  });

  it("returns undefined for a name the form doesn't have", () => {
    expect(renderHookOnce(() => useFormValue("missing"), store)).toBeUndefined();
  });

  it("returns undefined outside a provider", () => {
    expect(renderHookOnce(() => useFormValue("size"))).toBeUndefined();
  });
});
