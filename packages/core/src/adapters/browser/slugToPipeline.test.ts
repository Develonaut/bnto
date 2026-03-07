import { describe, it, expect } from "vitest";
import { slugToPipeline } from "./slugToPipeline";

describe("slugToPipeline", () => {
  it("returns a 3-node definition with nodeType + operation for a known slug", () => {
    const result = slugToPipeline("compress-images", { quality: 80 });

    expect(result).not.toBeNull();
    expect(result!.nodes).toHaveLength(3);
    expect(result!.nodes[0]).toEqual({ id: "input", type: "input", params: {} });
    expect(result!.nodes[1]).toEqual({
      id: "process",
      type: "image",
      params: { quality: 80, operation: "compress" },
    });
    expect(result!.nodes[2]).toEqual({ id: "output", type: "output", params: {} });
  });

  it("returns null for an unknown slug", () => {
    expect(slugToPipeline("unknown-slug")).toBeNull();
  });

  it("defaults params to empty object with operation added", () => {
    const result = slugToPipeline("clean-csv");

    expect(result).not.toBeNull();
    expect(result!.nodes[1].params).toEqual({ operation: "clean" });
  });

  it("maps all Tier 1 slugs to correct nodeType:operation pairs", () => {
    const expected: Record<string, { nodeType: string; operation: string }> = {
      "compress-images": { nodeType: "image", operation: "compress" },
      "resize-images": { nodeType: "image", operation: "resize" },
      "convert-image-format": { nodeType: "image", operation: "convert" },
      "clean-csv": { nodeType: "spreadsheet", operation: "clean" },
      "rename-csv-columns": { nodeType: "spreadsheet", operation: "rename" },
      "rename-files": { nodeType: "file-system", operation: "rename" },
    };

    for (const [slug, { nodeType, operation }] of Object.entries(expected)) {
      const result = slugToPipeline(slug);
      expect(result).not.toBeNull();
      expect(result!.nodes).toHaveLength(3);
      expect(result!.nodes[1].type).toBe(nodeType);
      expect(result!.nodes[1].params).toHaveProperty("operation", operation);
    }
  });
});
