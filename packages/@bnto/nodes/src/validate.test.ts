import { describe, expect, it } from "vitest";

import type { Definition } from "./definition";
import { validateDefinition } from "./validate";
import { CURRENT_FORMAT_VERSION } from "./formatVersion";

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

  it("accepts all 12 registered types with valid params", () => {
    const typeParams: Record<string, Record<string, unknown>> = {
      "edit-fields": { values: { name: "test" } },
      "file-system": { operation: "rename" },
      group: {},
      "http-request": { url: "https://example.com", method: "GET" },
      image: { operation: "resize" },
      input: {},
      loop: { mode: "times", count: 3 },
      output: {},
      parallel: { tasks: [{ a: 1 }] },
      "shell-command": { command: "echo hello" },
      spreadsheet: { operation: "clean" },
      transform: {},
    };
    for (const [type, params] of Object.entries(typeParams)) {
      const def = validDef({ type, parameters: params });
      const errors = validateDefinition(def);
      const typeErrors = errors.filter((e) => e.message.includes("unknown type"));
      expect(typeErrors).toHaveLength(0);
    }
  });
});

describe("validateDefinition — http-request (no schema, no type-specific validation)", () => {
  it("accepts http-request as a known type with valid params", () => {
    const def = validDef({
      type: "http-request",
      parameters: { url: "https://example.com", method: "GET" },
    });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });

  it("passes with any params (no schema to validate against)", () => {
    const def = validDef({ type: "http-request", parameters: {} });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
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

  it("forEach mode with no items is valid (engine iterates incoming files)", () => {
    const def = validDef({ type: "loop", parameters: { mode: "forEach" } });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
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

  it("passes with forEach and optional items param", () => {
    const def = validDef({ type: "loop", parameters: { mode: "forEach", items: "{{.files}}" } });
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

  it("accepts engine-backed rename operation", () => {
    const def = validDef({ type: "file-system", parameters: { operation: "rename" } });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });

  it("rejects legacy operations no longer in engine", () => {
    for (const op of ["read", "write", "copy", "move", "delete"]) {
      const def = validDef({ type: "file-system", parameters: { operation: op } });
      const errors = validateDefinition(def);
      expect(errors.some((e) => e.field === "operation")).toBe(true);
    }
  });
});

describe("validateDefinition — shell-command (no schema, no type-specific validation)", () => {
  it("accepts shell-command as a known type", () => {
    const def = validDef({ type: "shell-command", parameters: { command: "echo hello" } });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });

  it("passes with any params (no schema to validate against)", () => {
    const def = validDef({ type: "shell-command", parameters: {} });
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

  it("parallel requires tasks (Zod)", () => {
    const def = validDef({ type: "parallel" });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "tasks")).toBe(true);
  });

  it("spreadsheet requires operation (Zod, engine-only)", () => {
    const def = validDef({ type: "spreadsheet" });
    const errors = validateDefinition(def);
    expect(errors.length).toBeGreaterThanOrEqual(1);
    expect(errors.some((e) => e.field === "operation")).toBe(true);
  });

  it("image requires operation (Zod)", () => {
    const def = validDef({ type: "image" });
    const errors = validateDefinition(def);
    expect(errors.some((e) => e.field === "operation")).toBe(true);
  });

  it("transform with no params passes", () => {
    const def = validDef({ type: "transform" });
    const errors = validateDefinition(def);
    expect(errors).toHaveLength(0);
  });
});
