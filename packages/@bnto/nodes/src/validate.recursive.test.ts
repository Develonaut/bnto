import { describe, expect, it } from "vitest";

import type { Definition } from "./definition";
import { validateDefinition, validateEdges } from "./validate";

/** Creates a minimal valid definition for testing. */
function validDef(overrides: Partial<Definition> = {}): Definition {
  return {
    id: "test-node",
    type: "group",
    version: "1.0.0",
    name: "Test Node",
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    ...overrides,
  };
}

describe("validateDefinition — recursive group validation", () => {
  it("validates child nodes inside a group", () => {
    const def = validDef({
      type: "group",
      nodes: [validDef({ id: "good-child" }), validDef({ id: "bad-child", type: "banana" })],
      edges: [],
    });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.nodeId === "bad-child")).toBe(true);
    expect(errors.some((e) => e.message.includes("unknown type 'banana'"))).toBe(true);
  });

  it("validates deeply nested groups", () => {
    const def = validDef({
      type: "group",
      nodes: [
        validDef({
          id: "inner-group",
          type: "group",
          nodes: [
            validDef({ id: "deep-child", version: "" }), // missing version
          ],
          edges: [],
        }),
      ],
      edges: [],
    });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.nodeId === "deep-child" && e.field === "version")).toBe(true);
  });

  it("validates children inside loop nodes", () => {
    const def = validDef({
      type: "loop",
      parameters: { mode: "times", count: 3 },
      nodes: [validDef({ id: "loop-child", type: "" })],
      edges: [],
    });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.nodeId === "loop-child" && e.field === "type")).toBe(true);
  });

  it("validates children inside parallel nodes", () => {
    const def = validDef({
      type: "parallel",
      nodes: [validDef({ id: "" })],
      edges: [],
    });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "id")).toBe(true);
  });
});

describe("validateEdges", () => {
  it("returns no errors when edges are empty", () => {
    const def = validDef({ edges: [] });
    expect(validateEdges(def)).toHaveLength(0);
  });

  it("returns no errors when edges are undefined", () => {
    const def = validDef();
    expect(validateEdges(def)).toHaveLength(0);
  });

  it("catches invalid source node", () => {
    const def = validDef({
      type: "group",
      nodes: [validDef({ id: "a" }), validDef({ id: "b" })],
      edges: [{ id: "e1", source: "nonexistent", target: "b" }],
    });
    const errors = validateEdges(def);
    expect(errors).toHaveLength(1);
    expect(errors[0].message).toContain("invalid source 'nonexistent'");
  });

  it("catches invalid target node", () => {
    const def = validDef({
      type: "group",
      nodes: [validDef({ id: "a" }), validDef({ id: "b" })],
      edges: [{ id: "e1", source: "a", target: "nonexistent" }],
    });
    const errors = validateEdges(def);
    expect(errors).toHaveLength(1);
    expect(errors[0].message).toContain("invalid target 'nonexistent'");
  });

  it("catches both invalid source and target in same edge", () => {
    const def = validDef({
      type: "group",
      nodes: [validDef({ id: "a" })],
      edges: [{ id: "e1", source: "ghost1", target: "ghost2" }],
    });
    const errors = validateEdges(def);
    expect(errors).toHaveLength(2);
  });

  it("validates multiple edges", () => {
    const def = validDef({
      type: "group",
      nodes: [validDef({ id: "a" }), validDef({ id: "b" })],
      edges: [
        { id: "e1", source: "a", target: "b" },
        { id: "e2", source: "b", target: "missing" },
      ],
    });
    const errors = validateEdges(def);
    expect(errors).toHaveLength(1);
    expect(errors[0].message).toContain("invalid target 'missing'");
  });

  it("passes with valid edges", () => {
    const def = validDef({
      type: "group",
      nodes: [validDef({ id: "a" }), validDef({ id: "b" }), validDef({ id: "c" })],
      edges: [
        { id: "e1", source: "a", target: "b" },
        { id: "e2", source: "b", target: "c" },
      ],
    });
    const errors = validateEdges(def);
    expect(errors).toHaveLength(0);
  });
});

describe("validateDefinition — edge validation in groups", () => {
  it("validates edges as part of group validation", () => {
    const def = validDef({
      type: "group",
      nodes: [validDef({ id: "a" }), validDef({ id: "b" })],
      edges: [{ id: "e1", source: "a", target: "ghost" }],
    });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.message.includes("invalid target 'ghost'"))).toBe(true);
  });
});
