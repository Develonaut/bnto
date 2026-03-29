import { describe, expect, it } from "vitest";
import type { Definition } from "@bnto/core";
import { deriveDefaults } from "./deriveDefaults";

/** Minimal definition factory. */
function defn(nodes: Partial<Definition>[] = []): Definition {
  return {
    id: "root",
    type: "group",
    version: "1.0.0",
    name: "Test",
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    edges: [],
    nodes: nodes.map((n) => ({
      id: n.id ?? "node",
      type: n.type ?? "unknown",
      version: "1.0.0",
      name: n.name ?? "Node",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: n.parameters ?? {},
      inputPorts: [],
      outputPorts: [],
    })),
  };
}

describe("deriveDefaults", () => {
  it("returns empty object for definition with no processing nodes", () => {
    const result = deriveDefaults(
      defn([
        { id: "input", type: "input", parameters: { mode: "file-upload" } },
        { id: "output", type: "output", parameters: { mode: "download" } },
      ]),
    );
    expect(result).toEqual({});
  });

  it("collects parameters from a single processing node", () => {
    const result = deriveDefaults(
      defn([
        { id: "input", type: "input", parameters: {} },
        { id: "compress", type: "image-compress", parameters: { quality: 80 } },
        { id: "output", type: "output", parameters: {} },
      ]),
    );
    expect(result).toEqual({ quality: 80 });
  });

  it("flat-merges parameters from multiple processing nodes", () => {
    const result = deriveDefaults(
      defn([
        { id: "input", type: "input", parameters: {} },
        { id: "compress", type: "image-compress", parameters: { quality: 80 } },
        { id: "rename", type: "file-rename", parameters: { suffix: "-min" } },
        { id: "output", type: "output", parameters: {} },
      ]),
    );
    expect(result).toEqual({ quality: 80, suffix: "-min" });
  });

  it("later node parameters override earlier ones with the same key", () => {
    const result = deriveDefaults(
      defn([
        { id: "input", type: "input", parameters: {} },
        { id: "compress", type: "image-compress", parameters: { quality: 80 } },
        { id: "convert", type: "image-convert", parameters: { quality: 90, format: "webp" } },
        { id: "output", type: "output", parameters: {} },
      ]),
    );
    expect(result.quality).toBe(90);
    expect(result.format).toBe("webp");
  });

  it("handles definition with no nodes", () => {
    const result = deriveDefaults(defn());
    expect(result).toEqual({});
  });
});
