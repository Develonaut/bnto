import { describe, it, expect } from "vitest";
import { z } from "zod";
import type { NodeSchema, NodeParamFields } from "@bnto/core";
import { NODE_SCHEMAS } from "@bnto/core";
import { buildFormEntries } from "./buildFormEntries";

/** Minimal NodeSchema factory for testing. */
function makeSchema(
  params: Record<string, { label: string; description: string }>,
  shape: Record<string, z.ZodTypeAny>,
): NodeSchema {
  return {
    nodeType: "test",
    schemaVersion: 1,
    schema: z.object(shape),
    params,
  };
}

const meta = (label: string) => ({ label, description: `${label} description` });

describe("buildFormEntries", () => {
  it("returns empty array for empty visibleParams", () => {
    const schema = makeSchema({ quality: meta("Quality") }, { quality: z.number() });
    expect(buildFormEntries(schema, [])).toEqual([]);
  });

  it("creates single entries for ungrouped params", () => {
    const schema = makeSchema(
      { quality: meta("Quality"), format: meta("Format") },
      { quality: z.number().default(80), format: z.enum(["jpeg", "png"]).default("jpeg") },
    );

    const entries = buildFormEntries(schema, ["quality", "format"]);

    expect(entries).toHaveLength(2);
    expect(entries[0]).toMatchObject({ kind: "single", paramName: "quality" });
    expect(entries[1]).toMatchObject({ kind: "single", paramName: "format" });
  });

  it("groups consecutive params with the same group", () => {
    const schema = makeSchema(
      { width: meta("Width"), height: meta("Height") },
      { width: z.number().default(1920), height: z.number().default(1080) },
    );
    const fields: NodeParamFields = {
      width: { group: "dimensions" },
      height: { group: "dimensions" },
    };

    const entries = buildFormEntries(schema, ["width", "height"], fields);

    expect(entries).toHaveLength(1);
    expect(entries[0]).toMatchObject({ kind: "group", groupName: "dimensions" });
    if (entries[0].kind === "group") {
      expect(entries[0].fields).toHaveLength(2);
      expect(entries[0].fields[0].paramName).toBe("width");
      expect(entries[0].fields[1].paramName).toBe("height");
    }
  });

  it("splits non-consecutive same-group params into separate groups", () => {
    const schema = makeSchema(
      { a: meta("A"), b: meta("B"), c: meta("C") },
      { a: z.number(), b: z.string(), c: z.number() },
    );
    const fields: NodeParamFields = {
      a: { group: "dims" },
      c: { group: "dims" },
    };

    const entries = buildFormEntries(schema, ["a", "b", "c"], fields);

    expect(entries).toHaveLength(3);
    expect(entries[0]).toMatchObject({ kind: "group", groupName: "dims" });
    expect(entries[1]).toMatchObject({ kind: "single", paramName: "b" });
    expect(entries[2]).toMatchObject({ kind: "group", groupName: "dims" });
    if (entries[0].kind === "group") expect(entries[0].fields).toHaveLength(1);
    if (entries[2].kind === "group") expect(entries[2].fields).toHaveLength(1);
  });

  it("interleaves singles and groups correctly", () => {
    const schema = makeSchema(
      { format: meta("Format"), width: meta("W"), height: meta("H"), quality: meta("Q") },
      {
        format: z.enum(["jpeg", "png"]),
        width: z.number(),
        height: z.number(),
        quality: z.number(),
      },
    );
    const fields: NodeParamFields = {
      width: { group: "dimensions" },
      height: { group: "dimensions" },
    };

    const entries = buildFormEntries(schema, ["format", "width", "height", "quality"], fields);

    expect(entries.map((e) => e.kind)).toEqual(["single", "group", "single"]);
    expect(entries[0]).toMatchObject({ kind: "single", paramName: "format" });
    expect(entries[1]).toMatchObject({ kind: "group", groupName: "dimensions" });
    expect(entries[2]).toMatchObject({ kind: "single", paramName: "quality" });
  });

  it("skips params not present in schema.params", () => {
    const schema = makeSchema({ quality: meta("Quality") }, { quality: z.number() });

    const entries = buildFormEntries(schema, ["quality", "nonexistent"]);

    expect(entries).toHaveLength(1);
    expect(entries[0]).toMatchObject({ kind: "single", paramName: "quality" });
  });

  it("falls back to text control when param not in static field info", () => {
    const schema: NodeSchema = {
      nodeType: "test",
      schemaVersion: 1,
      schema: z.object({}),
      params: { orphan: meta("Orphan") },
    };

    const entries = buildFormEntries(schema, ["orphan"]);

    expect(entries).toHaveLength(1);
    if (entries[0].kind === "single") {
      expect(entries[0].fieldInfo).toMatchObject({
        type: "string",
        control: "text",
        required: true,
      });
    }
  });

  it("populates fieldInfo from static param field info for real node types", () => {
    const schema = NODE_SCHEMAS["image-compress"]!;

    const entries = buildFormEntries(schema, ["quality"]);

    expect(entries).toHaveLength(1);
    if (entries[0].kind === "single") {
      expect(entries[0].fieldInfo.control).toBe("slider");
      expect(entries[0].fieldInfo.type).toBe("number");
    }
  });

  it("passes fieldConfig through to entries", () => {
    const schema = makeSchema(
      { quality: meta("Quality") },
      { quality: z.number().min(1).max(100).default(80) },
    );
    const fields: NodeParamFields = {
      quality: { suffix: "%", presets: [{ value: 80, label: "High" }] },
    };

    const entries = buildFormEntries(schema, ["quality"], fields);

    expect(entries).toHaveLength(1);
    if (entries[0].kind === "single") {
      expect(entries[0].fieldConfig).toEqual(fields.quality);
    }
  });
});
