import { describe, expect, it } from "vitest";

import type { Definition } from "./definition";
import { validateDefinition, validateEdges } from "./validate";
import { CURRENT_FORMAT_VERSION } from "./formatVersion";

/** Creates a minimal valid definition for testing. */
function validDef(overrides: Partial<Definition> = {}): Definition {
  return {
    id: "test-node",
    type: "image",
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

describe("validateDefinition — full tree", () => {
  it("returns no errors for a valid definition", () => {
    const errors = validateDefinition(validDef());
    expect(errors).toHaveLength(0);
  });

  it("validates the entire tree recursively", () => {
    const def = validDef({
      type: "group",
      nodes: [
        validDef({ id: "child-1" }),
        validDef({ id: "", type: "image" }), // missing id
      ],
      edges: [],
    });
    const errors = validateDefinition(def);
    expect(errors.length).toBeGreaterThan(0);
    expect(errors.some((e) => e.field === "id")).toBe(true);
  });
});

describe("validateDefinition — core fields", () => {
  it("catches missing id", () => {
    const errors = validateDefinition(validDef({ id: "" }));
    expect(errors.some((e) => e.field === "id")).toBe(true);
  });

  it("catches missing type", () => {
    const errors = validateDefinition(validDef({ type: "" }));
    expect(errors.some((e) => e.field === "type")).toBe(true);
  });

  it("catches missing version", () => {
    const errors = validateDefinition(validDef({ version: "" }));
    expect(errors.some((e) => e.field === "version")).toBe(true);
  });

  it("catches all three missing at once", () => {
    const errors = validateDefinition(validDef({ id: "", type: "", version: "" }));
    expect(errors.length).toBeGreaterThanOrEqual(2); // id + type stops further checks
  });
});

describe("validateDefinition — version compatibility", () => {
  it("accepts current format version", () => {
    const errors = validateDefinition(validDef({ version: CURRENT_FORMAT_VERSION }));
    expect(errors.filter((e) => e.field === "version")).toHaveLength(0);
  });

  it("accepts same major with higher minor", () => {
    const errors = validateDefinition(validDef({ version: "1.5.0" }));
    expect(errors.filter((e) => e.field === "version")).toHaveLength(0);
  });

  it("rejects incompatible major version", () => {
    const errors = validateDefinition(validDef({ version: "2.0.0" }));
    expect(errors.some((e) => e.field === "version" && e.message.includes("unsupported"))).toBe(
      true,
    );
  });
});

describe("validateDefinition — unknown type", () => {
  it("catches unknown node types", () => {
    const errors = validateDefinition(validDef({ type: "banana" }));
    expect(errors).toHaveLength(1);
    expect(errors[0].field).toBe("type");
    expect(errors[0].message).toContain("unknown type 'banana'");
  });

  it("accepts all 10 registered types", () => {
    const types = [
      "edit-fields",
      "file-system",
      "group",
      "http-request",
      "image",
      "loop",
      "parallel",
      "shell-command",
      "spreadsheet",
      "transform",
    ];
    for (const type of types) {
      const def = validDef({ type });
      // Add required params so type-specific validators pass
      if (type === "http-request") {
        def.parameters = { url: "https://example.com", method: "GET" };
      } else if (type === "loop") {
        def.parameters = { mode: "times", count: 3 };
      } else if (type === "file-system") {
        def.parameters = { operation: "read" };
      } else if (type === "shell-command") {
        def.parameters = { command: "echo hello" };
      } else if (type === "edit-fields") {
        def.parameters = { values: { name: "test" } };
      }
      const errors = validateDefinition(def);
      const typeErrors = errors.filter((e) => e.message.includes("unknown type"));
      expect(typeErrors).toHaveLength(0);
    }
  });
});

describe("validateDefinition — http-request", () => {
  it("requires url parameter", () => {
    const def = validDef({ type: "http-request", parameters: { method: "GET" } });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "url")).toBe(true);
  });

  it("requires method parameter", () => {
    const def = validDef({ type: "http-request", parameters: { url: "https://example.com" } });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "method")).toBe(true);
  });

  it("rejects invalid HTTP method", () => {
    const def = validDef({
      type: "http-request",
      parameters: { url: "https://example.com", method: "YEET" },
    });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.message.includes("invalid method 'YEET'"))).toBe(true);
  });

  it("accepts all valid HTTP methods", () => {
    for (const method of ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"]) {
      const def = validDef({
        type: "http-request",
        parameters: { url: "https://example.com", method },
      });
      const errors = validateDefinition(def);
      expect(errors).toHaveLength(0);
    }
  });

  it("reports both missing url and method", () => {
    const def = validDef({ type: "http-request", parameters: {} });
    const errors = validateDefinition(def);
    expect(errors.filter((e) => e.field === "url")).toHaveLength(1);
    expect(errors.filter((e) => e.field === "method")).toHaveLength(1);
  });
});

describe("validateDefinition — loop", () => {
  it("requires mode parameter", () => {
    const def = validDef({ type: "loop", parameters: {} });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "mode")).toBe(true);
  });

  it("rejects invalid mode", () => {
    const def = validDef({ type: "loop", parameters: { mode: "infinite" } });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.message.includes("invalid mode 'infinite'"))).toBe(true);
  });

  it("requires items for forEach mode", () => {
    const def = validDef({ type: "loop", parameters: { mode: "forEach" } });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "items")).toBe(true);
  });

  it("requires count for times mode", () => {
    const def = validDef({ type: "loop", parameters: { mode: "times" } });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "count")).toBe(true);
  });

  it("requires condition for while mode", () => {
    const def = validDef({ type: "loop", parameters: { mode: "while" } });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "condition")).toBe(true);
  });

  it("passes with valid forEach params", () => {
    const def = validDef({ type: "loop", parameters: { mode: "forEach", items: [1, 2, 3] } });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });

  it("passes with valid times params", () => {
    const def = validDef({ type: "loop", parameters: { mode: "times", count: 5 } });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });

  it("passes with valid while params", () => {
    const def = validDef({ type: "loop", parameters: { mode: "while", condition: "{{.done}}" } });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });
});

describe("validateDefinition — file-system", () => {
  it("requires operation parameter", () => {
    const def = validDef({ type: "file-system", parameters: {} });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "operation")).toBe(true);
  });

  it("rejects invalid operation", () => {
    const def = validDef({ type: "file-system", parameters: { operation: "format-c" } });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.message.includes("invalid operation 'format-c'"))).toBe(true);
  });

  it("accepts all valid operations", () => {
    for (const op of ["read", "write", "copy", "move", "delete", "mkdir", "exists", "list"]) {
      const def = validDef({ type: "file-system", parameters: { operation: op } });
      const errors = validateDefinition(def);
      expect(errors).toHaveLength(0);
    }
  });
});

describe("validateDefinition — shell-command", () => {
  it("requires command parameter", () => {
    const def = validDef({ type: "shell-command", parameters: {} });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "command")).toBe(true);
  });

  it("rejects empty string command", () => {
    const def = validDef({ type: "shell-command", parameters: { command: "" } });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "command")).toBe(true);
  });

  it("passes with valid command", () => {
    const def = validDef({ type: "shell-command", parameters: { command: "echo hello" } });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });
});

describe("validateDefinition — edit-fields", () => {
  it("requires values parameter", () => {
    const def = validDef({ type: "edit-fields", parameters: {} });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "values")).toBe(true);
  });

  it("passes with values present", () => {
    const def = validDef({ type: "edit-fields", parameters: { values: { name: "test" } } });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });
});

describe("validateDefinition — minimal validation types", () => {
  it("group with no children passes", () => {
    const def = validDef({ type: "group" });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });

  it("parallel with no children passes", () => {
    const def = validDef({ type: "parallel" });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });

  it("spreadsheet with no params passes", () => {
    const def = validDef({ type: "spreadsheet" });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });

  it("image with no params passes", () => {
    const def = validDef({ type: "image" });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });

  it("transform with no params passes", () => {
    const def = validDef({ type: "transform" });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });
});

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
