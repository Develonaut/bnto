import { describe, expect, it } from "vitest";

import { createBlankDefinition } from "./createBlankDefinition";
import { validateDefinition } from "./validate";

describe("createBlankDefinition", () => {
  it("returns a valid definition with no validation errors", () => {
    const def = createBlankDefinition();
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });

  it("has type 'group' (root container)", () => {
    const def = createBlankDefinition();
    expect(def.type).toBe("group");
  });

  it("has a unique UUID as id", () => {
    const def1 = createBlankDefinition();
    const def2 = createBlankDefinition();
    expect(def1.id).toBeTruthy();
    expect(def2.id).toBeTruthy();
    expect(def1.id).not.toBe(def2.id);
  });

  it("has version 1.0.0", () => {
    const def = createBlankDefinition();
    expect(def.version).toBe("1.0.0");
  });

  it("has one input port and one output port with unique IDs", () => {
    const def = createBlankDefinition();
    expect(def.inputPorts).toHaveLength(1);
    expect(def.outputPorts).toHaveLength(1);
    expect(def.inputPorts[0]!.name).toBe("input");
    expect(def.outputPorts[0]!.name).toBe("output");
    // Port IDs are UUIDs, not hardcoded
    expect(def.inputPorts[0]!.id).toBeTruthy();
    expect(def.outputPorts[0]!.id).toBeTruthy();
    expect(def.inputPorts[0]!.id).not.toBe(def.outputPorts[0]!.id);
  });

  it("starts with empty nodes and edges", () => {
    const def = createBlankDefinition();
    expect(def.nodes).toEqual([]);
    expect(def.edges).toEqual([]);
  });

  it("has a createdAt timestamp in metadata", () => {
    const before = new Date().toISOString();
    const def = createBlankDefinition();
    const after = new Date().toISOString();

    expect(def.metadata.createdAt).toBeTruthy();
    expect(def.metadata.createdAt! >= before).toBe(true);
    expect(def.metadata.createdAt! <= after).toBe(true);
  });

  it("has a default name", () => {
    const def = createBlankDefinition();
    expect(def.name).toBe("New Recipe");
  });

  it("has position at origin", () => {
    const def = createBlankDefinition();
    expect(def.position).toEqual({ x: 0, y: 0 });
  });
});
