/**
 * layoutNodes tests — pure function, no React or store needed.
 */

import { describe, it, expect } from "vitest";
import { layoutNodes, GROUP_ID_PREFIX } from "./layoutNodes";
import type { BentoNode } from "./types";
import { STRIDE, ROW_OFFSET } from "./bentoSlots";

function makeNode(
  id: string,
  x: number,
  y: number,
  overrides?: Partial<BentoNode["data"]>,
): BentoNode {
  return {
    id,
    type: "compartment",
    position: { x, y },
    data: {
      label: id,
      variant: "primary",
      width: 140,
      height: 140,
      status: "idle",
      depth: 0,
      ...overrides,
    },
  };
}

function makeChild(
  id: string,
  parentId: string,
  depth = 1,
  overrides?: Partial<BentoNode["data"]>,
): BentoNode {
  return {
    id,
    type: "compartment",
    position: { x: 0, y: 0 },
    data: {
      label: id,
      variant: "primary",
      width: 140,
      height: 140,
      status: "idle",
      parentContainerId: parentId,
      depth,
      ...overrides,
    },
  };
}

describe("layoutNodes", () => {
  it("returns nodes unchanged when no containers are expanded", () => {
    const nodes = [makeNode("a", 0, 0), makeNode("b", STRIDE, 0)];
    const result = layoutNodes(nodes, new Set());
    expect(result).toBe(nodes); // referentially stable
  });

  it("positions single child centered under parent", () => {
    const parent = makeNode("p", STRIDE, 0, { isContainer: true, isExpanded: true });
    const child = makeChild("c1", "p");
    const result = layoutNodes([parent, child], new Set(["p"]));

    const positionedChild = result.find((n) => n.id === "c1");
    expect(positionedChild).toBeDefined();
    expect(positionedChild!.position.x).toBe(STRIDE); // centered = same as parent
    expect(positionedChild!.position.y).toBe(ROW_OFFSET);
  });

  it("stacks multiple children vertically under parent", () => {
    const parent = makeNode("p", STRIDE * 2, 0, { isContainer: true, isExpanded: true });
    const c1 = makeChild("c1", "p");
    const c2 = makeChild("c2", "p");
    const c3 = makeChild("c3", "p");
    const result = layoutNodes([parent, c1, c2, c3], new Set(["p"]));

    const positioned = result.filter((n) => n.data.parentContainerId === "p");
    expect(positioned).toHaveLength(3);

    // All children at parent's x, stacked vertically by ROW_OFFSET
    expect(positioned[0]!.position.x).toBe(STRIDE * 2);
    expect(positioned[0]!.position.y).toBe(ROW_OFFSET);
    expect(positioned[1]!.position.x).toBe(STRIDE * 2);
    expect(positioned[1]!.position.y).toBe(ROW_OFFSET * 2);
    expect(positioned[2]!.position.x).toBe(STRIDE * 2);
    expect(positioned[2]!.position.y).toBe(ROW_OFFSET * 3);
  });

  it("injects a containerGroup overlay node", () => {
    const parent = makeNode("p", 0, 0, { isContainer: true, isExpanded: true });
    const child = makeChild("c1", "p");
    const result = layoutNodes([parent, child], new Set(["p"]));

    const group = result.find((n) => n.id === `${GROUP_ID_PREFIX}p`);
    expect(group).toBeDefined();
    expect(group!.type).toBe("containerGroup");
    expect(group!.data.width).toBeGreaterThan(0);
    expect(group!.data.height).toBeGreaterThan(0);
  });

  it("does not position children of non-expanded containers", () => {
    const parent = makeNode("p", 0, 0, { isContainer: true, isExpanded: false });
    // This shouldn't happen (children shouldn't be in store when collapsed),
    // but test defensively
    const child = makeChild("c1", "p");
    const result = layoutNodes([parent, child], new Set());
    // Original nodes returned unchanged
    expect(result).toEqual([parent, child]);
  });
});
