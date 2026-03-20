import { describe, it, expect } from "vitest";
import type { NodeParamFieldInfo, NodeParam } from "@bnto/core";
import type { GroupField } from "./FieldGroup";
import { partitionGroupFields } from "./partitionGroupFields";

/** Build a minimal GroupField with the given control type. */
function field(paramName: string, control: NodeParamFieldInfo["control"]): GroupField {
  return {
    paramName,
    meta: { label: paramName, description: "" } as NodeParam,
    fieldInfo: { type: "string", control, required: false } as NodeParamFieldInfo,
  };
}

describe("partitionGroupFields", () => {
  it("places switch fields in inlineFields", () => {
    const { inlineFields, gridFields } = partitionGroupFields([field("a", "switch")]);
    expect(inlineFields).toHaveLength(1);
    expect(gridFields).toHaveLength(0);
  });

  it("places select fields in inlineFields", () => {
    const { inlineFields, gridFields } = partitionGroupFields([field("a", "select")]);
    expect(inlineFields).toHaveLength(1);
    expect(gridFields).toHaveLength(0);
  });

  it("places number fields in gridFields", () => {
    const { inlineFields, gridFields } = partitionGroupFields([field("a", "number")]);
    expect(inlineFields).toHaveLength(0);
    expect(gridFields).toHaveLength(1);
  });

  it("places text fields in gridFields", () => {
    const { inlineFields, gridFields } = partitionGroupFields([field("a", "text")]);
    expect(inlineFields).toHaveLength(0);
    expect(gridFields).toHaveLength(1);
  });

  it("places slider fields in gridFields", () => {
    const { inlineFields, gridFields } = partitionGroupFields([field("a", "slider")]);
    expect(inlineFields).toHaveLength(0);
    expect(gridFields).toHaveLength(1);
  });

  it("partitions mixed fields correctly", () => {
    const fields = [
      field("lock", "switch"),
      field("format", "select"),
      field("width", "number"),
      field("height", "number"),
    ];

    const { inlineFields, gridFields } = partitionGroupFields(fields);

    expect(inlineFields.map((f) => f.paramName)).toEqual(["lock", "format"]);
    expect(gridFields.map((f) => f.paramName)).toEqual(["width", "height"]);
  });

  it("returns both empty for empty input", () => {
    const { inlineFields, gridFields } = partitionGroupFields([]);
    expect(inlineFields).toHaveLength(0);
    expect(gridFields).toHaveLength(0);
  });

  it("preserves field order within each partition", () => {
    const fields = [
      field("a", "number"),
      field("b", "switch"),
      field("c", "text"),
      field("d", "select"),
    ];

    const { inlineFields, gridFields } = partitionGroupFields(fields);

    expect(inlineFields.map((f) => f.paramName)).toEqual(["b", "d"]);
    expect(gridFields.map((f) => f.paramName)).toEqual(["a", "c"]);
  });
});
