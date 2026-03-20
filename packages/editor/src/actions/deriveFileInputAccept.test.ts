import { describe, it, expect } from "vitest";
import { deriveFileInputAccept } from "./deriveFileInputAccept";
import type { NodeConfigs } from "../adapters/types";

describe("deriveFileInputAccept", () => {
  it("returns undefined when configs has no input node", () => {
    const configs: NodeConfigs = {
      "node-1": { nodeType: "image-compress", name: "Compress Images", parameters: {} },
    };
    expect(deriveFileInputAccept(configs)).toBeUndefined();
  });

  it("returns undefined when input node has no accept or extensions", () => {
    const configs: NodeConfigs = {
      "input-1": { nodeType: "input", name: "Input", parameters: { mode: "file-upload" } },
    };
    expect(deriveFileInputAccept(configs)).toBeUndefined();
  });

  it("returns undefined when accept is wildcard", () => {
    const configs: NodeConfigs = {
      "input-1": {
        nodeType: "input",
        name: "Input",
        parameters: { accept: ["*/*"] },
      },
    };
    expect(deriveFileInputAccept(configs)).toBeUndefined();
  });

  it("returns MIME types when only accept is set", () => {
    const configs: NodeConfigs = {
      "input-1": {
        nodeType: "input",
        name: "Input",
        parameters: { accept: ["image/jpeg", "image/png"] },
      },
    };
    expect(deriveFileInputAccept(configs)).toBe("image/jpeg,image/png");
  });

  it("returns extensions when only extensions is set", () => {
    const configs: NodeConfigs = {
      "input-1": {
        nodeType: "input",
        name: "Input",
        parameters: { extensions: [".jpg", ".png", ".webp"] },
      },
    };
    expect(deriveFileInputAccept(configs)).toBe(".jpg,.png,.webp");
  });

  it("merges both accept and extensions", () => {
    const configs: NodeConfigs = {
      "input-1": {
        nodeType: "input",
        name: "Input",
        parameters: {
          accept: ["image/jpeg", "image/png"],
          extensions: [".jpg", ".png"],
        },
      },
    };
    expect(deriveFileInputAccept(configs)).toBe("image/jpeg,image/png,.jpg,.png");
  });

  it("ignores non-array accept/extensions values", () => {
    const configs: NodeConfigs = {
      "input-1": {
        nodeType: "input",
        name: "Input",
        parameters: { accept: "image/jpeg", extensions: ".jpg" },
      },
    };
    expect(deriveFileInputAccept(configs)).toBeUndefined();
  });

  it("finds input node among other nodes", () => {
    const configs: NodeConfigs = {
      "node-1": { nodeType: "image-compress", name: "Compress Images", parameters: {} },
      "input-1": {
        nodeType: "input",
        name: "Input",
        parameters: { extensions: [".csv"] },
      },
      "output-1": { nodeType: "output", name: "Output", parameters: {} },
    };
    expect(deriveFileInputAccept(configs)).toBe(".csv");
  });
});
