import { describe, it, expect } from "vitest";
import type { NodeParamControl } from "@bnto/core";
import { CONTROL_REGISTRY } from "./index";

/** All control types defined by the NodeParamControl union. */
const ALL_CONTROLS: NodeParamControl[] = [
  "select",
  "switch",
  "slider",
  "number",
  "text",
  "textarea",
  "tagPicker",
  "keyValue",
];

describe("CONTROL_REGISTRY", () => {
  it("has an entry for every NodeParamControl type", () => {
    for (const control of ALL_CONTROLS) {
      expect(CONTROL_REGISTRY[control]).toBeDefined();
    }
  });

  it("maps each control to a function (React component)", () => {
    for (const control of ALL_CONTROLS) {
      expect(typeof CONTROL_REGISTRY[control]).toBe("function");
    }
  });

  it("has exactly the expected number of entries", () => {
    expect(Object.keys(CONTROL_REGISTRY)).toHaveLength(ALL_CONTROLS.length);
  });

  it("has no unexpected keys", () => {
    const registryKeys = Object.keys(CONTROL_REGISTRY).sort();
    const expectedKeys = [...ALL_CONTROLS].sort();
    expect(registryKeys).toEqual(expectedKeys);
  });
});
