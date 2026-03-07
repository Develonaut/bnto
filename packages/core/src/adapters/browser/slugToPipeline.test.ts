import { describe, it, expect } from "vitest";
import { slugToPipeline } from "./slugToPipeline";

describe("slugToPipeline", () => {
  it("returns a 3-node definition for a known slug", () => {
    const result = slugToPipeline("compress-images", { quality: 80 });

    expect(result).not.toBeNull();
    expect(result!.nodes).toHaveLength(3);
    expect(result!.nodes[0]).toEqual({ id: "input", type: "input", params: {} });
    expect(result!.nodes[1]).toEqual({
      id: "process",
      type: "compress-images",
      params: { quality: 80 },
    });
    expect(result!.nodes[2]).toEqual({ id: "output", type: "output", params: {} });
  });

  it("returns null for an unknown slug", () => {
    expect(slugToPipeline("unknown-slug")).toBeNull();
  });

  it("defaults params to empty object", () => {
    const result = slugToPipeline("clean-csv");

    expect(result).not.toBeNull();
    expect(result!.nodes[1].params).toEqual({});
  });

  it("works for all Tier 1 slugs", () => {
    const slugs = [
      "compress-images",
      "resize-images",
      "convert-image-format",
      "clean-csv",
      "rename-csv-columns",
      "rename-files",
    ];

    for (const slug of slugs) {
      const result = slugToPipeline(slug);
      expect(result).not.toBeNull();
      expect(result!.nodes).toHaveLength(3);
      expect(result!.nodes[1].type).toBe(slug);
    }
  });
});
