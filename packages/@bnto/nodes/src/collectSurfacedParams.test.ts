import { describe, expect, it } from "vitest";

import type { Definition } from "./definition";
import { collectSurfacedParams } from "./collectSurfacedParams";
import { CURRENT_FORMAT_VERSION } from "./formatVersion";
import {
  batchCompress,
  batchConvert,
  batchRename,
  batchResize,
  csvCleaner,
  columnRenamer,
} from "./recipes";

/** Creates a minimal valid definition for testing. */
function validDef(overrides: Partial<Definition> = {}): Definition {
  return {
    id: "test-node",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Test Node",
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    ...overrides,
  };
}

describe("collectSurfacedParams — predefined recipes", () => {
  it("surfaces quality param from batchCompress", () => {
    const groups = collectSurfacedParams(batchCompress.definition);
    expect(groups.length).toBeGreaterThanOrEqual(1);
    const qualityGroup = groups.find((g) => g.visibleParams.includes("quality"));
    expect(qualityGroup).toBeDefined();
    expect(qualityGroup!.leafNodeId).toBe("compress-image");
    expect(qualityGroup!.nodeType).toBe("image");
  });

  it("surfaces width/height/maintainAspect from batchResize", () => {
    const groups = collectSurfacedParams(batchResize.definition);
    expect(groups.length).toBeGreaterThanOrEqual(1);
    const resizeGroup = groups.find((g) => g.leafNodeId === "resize-image");
    expect(resizeGroup).toBeDefined();
    expect(resizeGroup!.visibleParams).toContain("width");
    expect(resizeGroup!.visibleParams).toContain("height");
    expect(resizeGroup!.visibleParams).toContain("maintainAspect");
  });

  it("surfaces format/quality from batchConvert", () => {
    const groups = collectSurfacedParams(batchConvert.definition);
    const convertGroup = groups.find((g) => g.leafNodeId === "convert-image");
    expect(convertGroup).toBeDefined();
    expect(convertGroup!.visibleParams).toContain("format");
  });

  it("surfaces rename params from batchRename", () => {
    const groups = collectSurfacedParams(batchRename.definition);
    const renameGroup = groups.find((g) => g.leafNodeId === "rename-file");
    expect(renameGroup).toBeDefined();
    // file-system:rename has find, replace, case, prefix, suffix, pattern
    expect(renameGroup!.visibleParams.length).toBeGreaterThanOrEqual(1);
  });

  it("surfaces cleaning params from csvCleaner", () => {
    const groups = collectSurfacedParams(csvCleaner.definition);
    const cleanGroup = groups.find((g) => g.leafNodeId === "clean");
    expect(cleanGroup).toBeDefined();
    expect(cleanGroup!.visibleParams).toContain("trimWhitespace");
    expect(cleanGroup!.visibleParams).toContain("removeEmptyRows");
    expect(cleanGroup!.visibleParams).toContain("removeDuplicates");
  });

  it("surfaces columns param from columnRenamer", () => {
    const groups = collectSurfacedParams(columnRenamer.definition);
    const renameGroup = groups.find((g) => g.leafNodeId === "rename-columns");
    expect(renameGroup).toBeDefined();
    expect(renameGroup!.visibleParams).toContain("columns");
  });
});

describe("collectSurfacedParams — auto mode (default)", () => {
  it("returns empty for non-container nodes", () => {
    const def = validDef({ type: "image", parameters: { operation: "compress" } });
    expect(collectSurfacedParams(def)).toHaveLength(0);
  });

  it("returns empty for container with no leaf children", () => {
    const def = validDef({ nodes: [], edges: [] });
    expect(collectSurfacedParams(def)).toHaveLength(0);
  });

  it("excludes operation param from surfaced results", () => {
    const groups = collectSurfacedParams(batchCompress.definition);
    for (const group of groups) {
      expect(group.visibleParams).not.toContain("operation");
    }
  });

  it("walks deeply nested containers (group → loop → leaf)", () => {
    const deeplyNested = validDef({
      id: "outer-group",
      type: "group",
      nodes: [
        validDef({
          id: "inner-loop",
          type: "loop",
          parameters: { mode: "forEach" },
          nodes: [
            validDef({
              id: "deep-leaf",
              type: "image",
              parameters: { operation: "compress", quality: 60 },
            }),
          ],
        }),
      ],
    });
    const groups = collectSurfacedParams(deeplyNested);
    expect(groups.length).toBeGreaterThanOrEqual(1);
    expect(groups[0].leafNodeId).toBe("deep-leaf");
    expect(groups[0].values.quality).toBe(60);
  });
});

describe("collectSurfacedParams — manual override", () => {
  it("mode: manual with include filters to specified params", () => {
    const def = validDef({
      ...batchCompress.definition,
      surfacedParams: {
        mode: "manual",
        include: [{ nodeId: "compress-image", params: ["quality"] }],
      },
    });
    const groups = collectSurfacedParams(def);
    expect(groups).toHaveLength(1);
    expect(groups[0].visibleParams).toEqual(["quality"]);
  });

  it("mode: auto with exclude hides specified params", () => {
    const def = validDef({
      ...batchResize.definition,
      surfacedParams: {
        mode: "auto",
        exclude: [{ nodeId: "resize-image", params: ["maintainAspect"] }],
      },
    });
    const groups = collectSurfacedParams(def);
    const resizeGroup = groups.find((g) => g.leafNodeId === "resize-image");
    expect(resizeGroup).toBeDefined();
    expect(resizeGroup!.visibleParams).not.toContain("maintainAspect");
    // Other params should still be visible
    expect(resizeGroup!.visibleParams).toContain("width");
  });

  it("mode: manual with no matching include returns empty", () => {
    const def = validDef({
      ...batchCompress.definition,
      surfacedParams: {
        mode: "manual",
        include: [{ nodeId: "nonexistent-node" }],
      },
    });
    const groups = collectSurfacedParams(def);
    expect(groups).toHaveLength(0);
  });
});

describe("collectSurfacedParams — displayName from customData", () => {
  it("uses customData.displayName when present", () => {
    const def = validDef({
      type: "group",
      nodes: [
        validDef({
          id: "leaf",
          type: "image",
          parameters: { operation: "compress", quality: 80 },
          metadata: { customData: { displayName: "Compression" } },
        }),
      ],
    });
    const groups = collectSurfacedParams(def);
    expect(groups[0].label).toBe("Compression");
  });

  it("falls back to node name when displayName absent", () => {
    const groups = collectSurfacedParams(batchCompress.definition);
    // The leaf node "compress-image" has no customData.displayName, falls back to node.name
    const compressGroup = groups.find((g) => g.leafNodeId === "compress-image");
    expect(compressGroup!.label).toBe("Compress Image");
  });
});
