/**
 * Tests for individual node type schemas — validates parameter shapes
 * match the Go engine's validator rules and node implementations.
 */
import { describe, expect, it } from "vitest";

import { NODE_SCHEMAS } from "./index";
import type { NodeSchema, ParameterSchema } from "./types";

function findParam(schema: NodeSchema, name: string): ParameterSchema {
  const param = schema.parameters.find((p) => p.name === name);
  if (!param) throw new Error(`"${name}" not found in ${schema.nodeType}`);
  return param;
}

describe("http-request schema", () => {
  const schema = NODE_SCHEMAS["http-request"];

  it("requires url and method", () => {
    const names = schema.parameters.filter((p) => p.required).map((p) => p.name).sort();
    expect(names).toEqual(["method", "url"]);
  });

  it("method enum has 7 values matching Go validator", () => {
    const m = findParam(schema, "method");
    expect(m.enumValues).toHaveLength(7);
    expect(m.enumValues).toContain("GET");
    expect(m.enumValues).toContain("OPTIONS");
  });

  it("timeout defaults to 30", () => {
    expect(findParam(schema, "timeout").default).toBe(30);
  });
});

describe("file-system schema", () => {
  const schema = NODE_SCHEMAS["file-system"];

  it("requires only operation", () => {
    const required = schema.parameters.filter((p) => p.required);
    expect(required).toHaveLength(1);
    expect(required[0].name).toBe("operation");
  });

  it("operation enum has 8 values matching Go validator", () => {
    const op = findParam(schema, "operation");
    expect(op.enumValues).toHaveLength(8);
    expect(op.enumValues).toContain("list");
    expect(op.enumValues).toContain("delete");
  });

  it("content is conditionally required for write", () => {
    expect(findParam(schema, "content").requiredWhen).toEqual({
      param: "operation", equals: "write",
    });
  });
});

describe("loop schema", () => {
  const schema = NODE_SCHEMAS["loop"];

  it("requires mode", () => {
    const required = schema.parameters.filter((p) => p.required);
    expect(required).toHaveLength(1);
    expect(required[0].name).toBe("mode");
  });

  it("mode-specific params are conditionally required", () => {
    expect(findParam(schema, "items").requiredWhen).toEqual({ param: "mode", equals: "forEach" });
    expect(findParam(schema, "count").requiredWhen).toEqual({ param: "mode", equals: "times" });
    expect(findParam(schema, "condition").requiredWhen).toEqual({ param: "mode", equals: "while" });
  });
});

describe("shell-command schema", () => {
  const schema = NODE_SCHEMAS["shell-command"];

  it("requires command", () => {
    const required = schema.parameters.filter((p) => p.required);
    expect(required).toHaveLength(1);
    expect(required[0].name).toBe("command");
  });

  it("has correct defaults for timeout, retry, stall", () => {
    expect(findParam(schema, "timeout").default).toBe(120);
    expect(findParam(schema, "retry").default).toBe(0);
    expect(findParam(schema, "retryDelay").default).toBe(5);
    expect(findParam(schema, "stallTimeout").default).toBe(0);
  });
});

describe("edit-fields schema", () => {
  const schema = NODE_SCHEMAS["edit-fields"];

  it("requires values as object", () => {
    const required = schema.parameters.filter((p) => p.required);
    expect(required).toHaveLength(1);
    expect(required[0].name).toBe("values");
    expect(required[0].type).toBe("object");
  });

  it("keepOnlySet defaults to false", () => {
    expect(findParam(schema, "keepOnlySet").default).toBe(false);
  });
});

describe("image schema", () => {
  const schema = NODE_SCHEMAS["image"];

  it("requires operation", () => {
    const required = schema.parameters.filter((p) => p.required);
    expect(required).toHaveLength(1);
    expect(required[0].name).toBe("operation");
  });

  it("quality defaults to 80 with 1-100 range", () => {
    const q = findParam(schema, "quality");
    expect(q.default).toBe(80);
    expect(q.min).toBe(1);
    expect(q.max).toBe(100);
  });

  it("resize params visible only for resize", () => {
    for (const name of ["width", "height", "maintainAspect"]) {
      expect(findParam(schema, name).visibleWhen).toEqual({
        param: "operation", equals: "resize",
      });
    }
  });

  it("composite requires base and overlay", () => {
    expect(findParam(schema, "base").requiredWhen).toEqual({
      param: "operation", equals: "composite",
    });
    expect(findParam(schema, "overlay").requiredWhen).toEqual({
      param: "operation", equals: "composite",
    });
  });
});

describe("spreadsheet schema", () => {
  const schema = NODE_SCHEMAS["spreadsheet"];

  it("requires operation, format, and path", () => {
    const names = schema.parameters.filter((p) => p.required).map((p) => p.name).sort();
    expect(names).toEqual(["format", "operation", "path"]);
  });

  it("rows conditionally required for write", () => {
    expect(findParam(schema, "rows").requiredWhen).toEqual({
      param: "operation", equals: "write",
    });
  });
});

describe("transform schema", () => {
  it("has expression and mappings, neither required", () => {
    const schema = NODE_SCHEMAS["transform"];
    expect(findParam(schema, "expression").required).toBe(false);
    expect(findParam(schema, "mappings").required).toBe(false);
  });
});

describe("group schema", () => {
  const schema = NODE_SCHEMAS["group"];

  it("has no required parameters", () => {
    expect(schema.parameters.filter((p) => p.required)).toHaveLength(0);
  });

  it("mode defaults to sequential", () => {
    expect(findParam(schema, "mode").default).toBe("sequential");
  });
});

describe("parallel schema", () => {
  const schema = NODE_SCHEMAS["parallel"];

  it("requires tasks", () => {
    const required = schema.parameters.filter((p) => p.required);
    expect(required).toHaveLength(1);
    expect(required[0].name).toBe("tasks");
  });

  it("errorStrategy defaults to failFast", () => {
    expect(findParam(schema, "errorStrategy").default).toBe("failFast");
  });
});
