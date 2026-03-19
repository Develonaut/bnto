/**
 * generatePipelineEdges tests — verify edge derivation from node order.
 */

import { describe, it, expect } from "vitest";
import { generatePipelineEdges } from "./generatePipelineEdges";
import type { BentoNode } from "./types";

/** Helper to create a minimal BentoNode for testing. */
function node(
  id: string,
  type: BentoNode["type"],
  x: number,
  overrides: Partial<BentoNode["data"]> = {},
): BentoNode {
  return {
    id,
    type,
    position: { x, y: 0 },
    data: {
      label: id,
      variant: "muted",
      width: 100,
      height: 100,
      status: "idle",
      ...overrides,
    },
  };
}

describe("generatePipelineEdges", () => {
  it("connects sequential nodes left-to-right", () => {
    const nodes = [
      node("input", "io", 0),
      node("rename", "compartment", 200),
      node("output", "io", 400),
    ];
    const edges = generatePipelineEdges(nodes);
    expect(edges).toHaveLength(2);
    expect(edges[0]).toMatchObject({ source: "input", target: "rename", type: "pipeline" });
    expect(edges[1]).toMatchObject({ source: "rename", target: "output", type: "pipeline" });
  });

  it("generates correct edge IDs", () => {
    const nodes = [node("a", "compartment", 0), node("b", "compartment", 200)];
    const edges = generatePipelineEdges(nodes);
    expect(edges[0]!.id).toBe("pipe-a-b");
  });

  it("treats containers as opaque boxes — no internal edges", () => {
    const nodes = [
      node("input", "io", 0),
      node("forEach", "containerGroup", 200),
      node("child1", "compartment", 250, { parentContainerId: "forEach" }),
      node("child2", "compartment", 300, { parentContainerId: "forEach" }),
      node("output", "io", 500),
    ];
    const edges = generatePipelineEdges(nodes);
    expect(edges).toHaveLength(2);
    expect(edges[0]).toMatchObject({ source: "input", target: "forEach" });
    expect(edges[1]).toMatchObject({ source: "forEach", target: "output" });
  });

  it("returns empty array for a single node", () => {
    const nodes = [node("input", "io", 0)];
    expect(generatePipelineEdges(nodes)).toHaveLength(0);
  });

  it("returns empty array for no nodes", () => {
    expect(generatePipelineEdges([])).toHaveLength(0);
  });

  it("skips addDivider and placeholder nodes", () => {
    const nodes = [
      node("input", "io", 0),
      node("div-1", "addDivider", 100),
      node("rename", "compartment", 200),
      node("placeholder", "placeholder", 300),
      node("output", "io", 400),
    ];
    const edges = generatePipelineEdges(nodes);
    expect(edges).toHaveLength(2);
    expect(edges[0]).toMatchObject({ source: "input", target: "rename" });
    expect(edges[1]).toMatchObject({ source: "rename", target: "output" });
  });

  it("sorts by x-position regardless of array order", () => {
    const nodes = [
      node("output", "io", 400),
      node("input", "io", 0),
      node("middle", "compartment", 200),
    ];
    const edges = generatePipelineEdges(nodes);
    expect(edges[0]).toMatchObject({ source: "input", target: "middle" });
    expect(edges[1]).toMatchObject({ source: "middle", target: "output" });
  });

  describe("progress derivation", () => {
    const nodes = [
      node("input", "io", 0),
      node("compress", "compartment", 200),
      node("output", "io", 400),
    ];

    it("sets progress=1.0 when source is completed", () => {
      const edges = generatePipelineEdges(
        nodes,
        { input: "completed", compress: "active", output: "pending" },
        {},
      );
      expect(edges[0]!.data!.progress).toBe(1);
    });

    it("sets progress based on nodeProgress when source is active", () => {
      const edges = generatePipelineEdges(nodes, { input: "active" }, { input: 50 });
      expect(edges[0]!.data!.progress).toBe(0.5);
    });

    it("sets progress=0 when source is pending", () => {
      const edges = generatePipelineEdges(nodes, { input: "pending" }, {});
      expect(edges[0]!.data!.progress).toBe(0);
    });

    it("sets progress=0 when no execution state", () => {
      const edges = generatePipelineEdges(nodes);
      expect(edges[0]!.data!.progress).toBe(0);
      expect(edges[1]!.data!.progress).toBe(0);
    });

    it("sets progress=1.0 when source has failed", () => {
      const edges = generatePipelineEdges(nodes, { input: "failed" }, {});
      expect(edges[0]!.data!.progress).toBe(1);
    });
  });
});
