/**
 * deriveAcceptedTypes tests — verify Definition → AcceptedTypes conversion.
 */

import { describe, it, expect } from "vitest";
import { deriveAcceptedTypes } from "./deriveAcceptedTypes";
import { createBlankDefinition, addNode } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";

describe("deriveAcceptedTypes", () => {
  it("returns wildcard for a blank definition", () => {
    const def = createBlankDefinition();
    const result = deriveAcceptedTypes(def);
    expect(result.accept).toBe("*/*");
    expect(result.label).toBe("Any files");
  });

  it("returns wildcard when no input node exists", () => {
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
    const result = deriveAcceptedTypes(def);
    expect(result.accept).toBe("*/*");
    expect(result.label).toBe("files");
  });

  it("derives accept from input node with MIME types", () => {
    const def = createBlankDefinition();
    const inputNode = def.nodes!.find((n) => n.type === "input")!;
    inputNode.parameters = {
      mode: "file-upload",
      accept: ["image/jpeg", "image/png", "image/webp"],
      extensions: [".jpg", ".jpeg", ".png", ".webp"],
      label: "JPEG, PNG, or WebP images",
    };

    const result = deriveAcceptedTypes(def);
    // All MIME types share "image/" prefix, so mimePrefix is set
    // and extensions are excluded from accept string
    expect(result.accept).toBe("image/jpeg,image/png,image/webp");
    expect(result.label).toBe("JPEG, PNG, or WebP images");
    expect(result.mimePrefix).toBe("image/");
  });

  it("includes extensions when MIME types have mixed prefixes", () => {
    const def = createBlankDefinition();
    const inputNode = def.nodes!.find((n) => n.type === "input")!;
    inputNode.parameters = {
      mode: "file-upload",
      accept: ["text/csv", "application/vnd.ms-excel"],
      extensions: [".csv"],
      label: "CSV files",
    };

    const result = deriveAcceptedTypes(def);
    expect(result.accept).toBe("text/csv,application/vnd.ms-excel,.csv");
    expect(result.mimePrefix).toBeUndefined();
  });

  it("returns wildcard for wildcard MIME type", () => {
    const def = createBlankDefinition();
    const inputNode = def.nodes!.find((n) => n.type === "input")!;
    inputNode.parameters = {
      mode: "file-upload",
      accept: ["*/*"],
      label: "Any files",
    };

    const result = deriveAcceptedTypes(def);
    expect(result.accept).toBe("*/*");
  });

  it("handles input node with empty accept arrays", () => {
    const def = createBlankDefinition();
    const inputNode = def.nodes!.find((n) => n.type === "input")!;
    inputNode.parameters = {
      mode: "file-upload",
      accept: [],
      extensions: [],
      label: "Drop files",
    };

    const result = deriveAcceptedTypes(def);
    expect(result.accept).toBe("*/*");
    expect(result.label).toBe("Drop files");
  });

  it("works with definitions that have non-IO nodes too", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const inputNode = def.nodes!.find((n) => n.type === "input")!;
    inputNode.parameters = {
      mode: "file-upload",
      accept: ["image/jpeg"],
      extensions: [".jpg"],
      label: "JPEG images",
    };

    const result = deriveAcceptedTypes(def);
    expect(result.accept).toBe("image/jpeg");
    expect(result.mimePrefix).toBe("image/");
  });
});
