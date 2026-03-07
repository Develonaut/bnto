/**
 * useExecutionNodes — unit tests for execution state → node data bridging.
 *
 * Tests the pure mapping logic: when executionState has status for a node,
 * the returned node should have data.status reflecting it.
 */

import { describe, it, expect } from "vitest";
import type { BentoNode } from "../adapters/types";

type NodeStatus = BentoNode["data"]["status"];

// Extract the pure mapping logic for testing without React hooks
function applyExecutionState(
  nodes: BentoNode[],
  executionState: Record<string, NodeStatus>,
): BentoNode[] {
  const hasExecution = Object.keys(executionState).length > 0;
  if (!hasExecution) return nodes;

  return nodes.map((node) => {
    const status = executionState[node.id];
    if (!status || status === node.data.status) return node;
    return { ...node, data: { ...node.data, status } };
  });
}

function makeNode(id: string, status = "idle"): BentoNode {
  return {
    id,
    type: "compartment",
    position: { x: 0, y: 0 },
    data: {
      label: id,
      variant: "primary",
      width: 120,
      height: 120,
      status: status as BentoNode["data"]["status"],
    },
  };
}

describe("applyExecutionState", () => {
  it("returns same nodes when executionState is empty", () => {
    const nodes = [makeNode("a"), makeNode("b")];
    const result = applyExecutionState(nodes, {});
    expect(result).toBe(nodes);
  });

  it("updates node status from executionState", () => {
    const nodes = [makeNode("a"), makeNode("b")];
    const result = applyExecutionState(nodes, { a: "active", b: "pending" });
    expect(result[0]!.data.status).toBe("active");
    expect(result[1]!.data.status).toBe("pending");
  });

  it("preserves node reference when status unchanged", () => {
    const nodes = [makeNode("a", "active"), makeNode("b")];
    const result = applyExecutionState(nodes, { a: "active" });
    expect(result[0]).toBe(nodes[0]);
  });

  it("handles failed status", () => {
    const nodes = [makeNode("a")];
    const result = applyExecutionState(nodes, { a: "failed" });
    expect(result[0]!.data.status).toBe("failed");
  });

  it("handles completed status", () => {
    const nodes = [makeNode("a")];
    const result = applyExecutionState(nodes, { a: "completed" });
    expect(result[0]!.data.status).toBe("completed");
  });

  it("skips nodes not in executionState", () => {
    const nodes = [makeNode("a"), makeNode("b")];
    const result = applyExecutionState(nodes, { a: "active" });
    expect(result[0]!.data.status).toBe("active");
    expect(result[1]).toBe(nodes[1]);
  });
});
