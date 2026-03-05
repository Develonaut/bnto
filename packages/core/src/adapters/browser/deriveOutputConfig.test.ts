/**
 * deriveOutputConfig tests — verify Definition → OutputConfig conversion.
 */

import { describe, it, expect } from "vitest";
import { deriveOutputConfig } from "./deriveOutputConfig";
import { createBlankDefinition } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";

describe("deriveOutputConfig", () => {
  it("returns defaults for a blank definition", () => {
    const def = createBlankDefinition();
    const result = deriveOutputConfig(def);
    expect(result.mode).toBe("download");
    expect(result.zip).toBe(true);
    expect(result.autoDownload).toBe(false);
    expect(result.label).toBe("Output Files");
  });

  it("returns defaults when no output node exists", () => {
    const def: Definition = {
      id: "no-io",
      name: "No IO",
      type: "group",
      version: "1.0.0",
      nodes: [],
      parameters: {},
      position: { x: 0, y: 0 },
      metadata: {},
      inputPorts: [],
      outputPorts: [],
    };
    const result = deriveOutputConfig(def);
    expect(result.mode).toBe("download");
    expect(result.zip).toBe(true);
  });

  it("reads mode from output node", () => {
    const def = createBlankDefinition();
    const outputNode = def.nodes!.find((n) => n.type === "output")!;
    outputNode.parameters = { mode: "display", label: "Preview" };

    const result = deriveOutputConfig(def);
    expect(result.mode).toBe("display");
    expect(result.label).toBe("Preview");
  });

  it("reads zip and autoDownload from output node", () => {
    const def = createBlankDefinition();
    const outputNode = def.nodes!.find((n) => n.type === "output")!;
    outputNode.parameters = {
      mode: "download",
      zip: false,
      autoDownload: true,
      label: "Compressed Images",
    };

    const result = deriveOutputConfig(def);
    expect(result.zip).toBe(false);
    expect(result.autoDownload).toBe(true);
    expect(result.label).toBe("Compressed Images");
  });

  it("reads filename template from output node", () => {
    const def = createBlankDefinition();
    const outputNode = def.nodes!.find((n) => n.type === "output")!;
    outputNode.parameters = {
      mode: "download",
      filename: "compressed-{{name}}",
    };

    const result = deriveOutputConfig(def);
    expect(result.filename).toBe("compressed-{{name}}");
  });

  it("uses defaults for missing parameters", () => {
    const def = createBlankDefinition();
    const outputNode = def.nodes!.find((n) => n.type === "output")!;
    outputNode.parameters = {};

    const result = deriveOutputConfig(def);
    expect(result.mode).toBe("download");
    expect(result.zip).toBe(true);
    expect(result.autoDownload).toBe(false);
    expect(result.label).toBe("Results");
    expect(result.filename).toBeUndefined();
  });
});
