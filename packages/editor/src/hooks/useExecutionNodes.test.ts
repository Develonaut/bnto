import { describe, it, expect } from "vitest";
import type { BentoNode, CompartmentNodeData } from "../adapters/types";

/**
 * Test the pure logic behind useExecutionNodes — given nodes and
 * an executionState map, produce nodes with updated data.status.
 *
 * Extracted as a pure function test since the hook is a thin wrapper
 * around useMemo with this logic.
 */

function applyExecutionState(
  nodes: BentoNode[],
  executionState: Record<string, CompartmentNodeData["status"]>,
): BentoNode[] {
  const keys = Object.keys(executionState);
  if (keys.length === 0) return nodes;

  return nodes.map((node) => {
    const status = executionState[node.id];
    if (!status || status === node.data.status) return node;

    return {
      ...node,
      data: { ...node.data, status },
    };
  });
}

function makeNode(id: string, status: CompartmentNodeData["status"] = "idle"): BentoNode {
  return {
    id,
    type: "compartment",
    position: { x: 0, y: 0 },
    data: {
      label: id,
      variant: "primary",
      width: 200,
      height: 200,
      status,
    },
  } as BentoNode;
}

describe("applyExecutionState", () => {
  it("returns same array reference when executionState is empty", () => {
    const nodes = [makeNode("a"), makeNode("b")];
    const result = applyExecutionState(nodes, {});
    expect(result).toBe(nodes);
  });

  it("updates status when executionState has entries", () => {
    const nodes = [makeNode("a", "idle"), makeNode("b", "idle")];
    const result = applyExecutionState(nodes, { a: "active", b: "pending" });
    expect(result[0].data.status).toBe("active");
    expect(result[1].data.status).toBe("pending");
  });

  it("preserves node reference when status already matches", () => {
    const nodes = [makeNode("a", "completed")];
    const result = applyExecutionState(nodes, { a: "completed" });
    expect(result[0]).toBe(nodes[0]);
  });

  it("only updates nodes present in executionState", () => {
    const nodes = [makeNode("a", "idle"), makeNode("b", "idle")];
    const result = applyExecutionState(nodes, { a: "active" });
    expect(result[0].data.status).toBe("active");
    expect(result[1]).toBe(nodes[1]); // unchanged — same reference
  });
});
