import { describe, it, expect } from "vitest";
import type { NodeParamControl } from "@bnto/core";
import { getFieldLayout } from "./fieldLayout";

describe("getFieldLayout", () => {
  it("returns 'inline' for switch", () => {
    expect(getFieldLayout("switch")).toBe("inline");
  });

  it("returns 'inline' for select", () => {
    expect(getFieldLayout("select")).toBe("inline");
  });

  it("returns 'self-labeled' for slider", () => {
    expect(getFieldLayout("slider")).toBe("self-labeled");
  });

  it("returns 'column' for text", () => {
    expect(getFieldLayout("text")).toBe("column");
  });

  it("returns 'column' for number", () => {
    expect(getFieldLayout("number")).toBe("column");
  });

  it("returns 'column' for textarea", () => {
    expect(getFieldLayout("textarea")).toBe("column");
  });

  it("returns 'column' for tagPicker", () => {
    expect(getFieldLayout("tagPicker")).toBe("column");
  });

  it("returns 'column' for keyValue", () => {
    expect(getFieldLayout("keyValue")).toBe("column");
  });

  it("covers all NodeParamControl values", () => {
    const allControls: NodeParamControl[] = [
      "select",
      "switch",
      "slider",
      "number",
      "text",
      "textarea",
      "tagPicker",
      "keyValue",
    ];
    for (const control of allControls) {
      const layout = getFieldLayout(control);
      expect(["inline", "self-labeled", "column"]).toContain(layout);
    }
  });
});
