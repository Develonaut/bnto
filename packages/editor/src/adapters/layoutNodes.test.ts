/**
 * layoutNodes tests — pure function, no React or store needed.
 */

import { describe, it, expect } from "vitest";
import { layoutNodes, GROUP_ID_PREFIX } from "./layoutNodes";
import type { BentoNode } from "./types";
import { STRIDE, ROW_OFFSET, CELL, GAP_X } from "./bentoSlots";

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

  it("positions children horizontally when parent depth is odd", () => {
    const parent = makeNode("p", 0, 0, { isContainer: true, isExpanded: true, depth: 1 });
    const c1 = makeChild("c1", "p", 2);
    const c2 = makeChild("c2", "p", 2);
    const result = layoutNodes([parent, c1, c2], new Set(["p"]));

    const positioned = result.filter((n) => n.data.parentContainerId === "p");
    expect(positioned).toHaveLength(2);

    // Children spread right from parent at same y
    expect(positioned[0]!.position.x).toBe(STRIDE);
    expect(positioned[0]!.position.y).toBe(0);
    expect(positioned[1]!.position.x).toBe(STRIDE * 2);
    expect(positioned[1]!.position.y).toBe(0);
  });

  it("alternates direction at each nesting level", () => {
    // Depth 0 parent → vertical children
    const outer = makeNode("outer", 0, 0, { isContainer: true, isExpanded: true, depth: 0 });
    // Depth 1 child that is also a container → horizontal grandchildren
    const inner = makeChild("inner", "outer", 1, { isContainer: true, isExpanded: true });
    const gc1 = makeChild("gc1", "inner", 2);
    const gc2 = makeChild("gc2", "inner", 2);

    const result = layoutNodes([outer, inner, gc1, gc2], new Set(["outer", "inner"]));

    // inner is positioned vertically below outer
    const posInner = result.find((n) => n.id === "inner")!;
    expect(posInner.position.x).toBe(0);
    expect(posInner.position.y).toBe(ROW_OFFSET);

    // grandchildren are positioned horizontally from inner
    const posGc1 = result.find((n) => n.id === "gc1")!;
    const posGc2 = result.find((n) => n.id === "gc2")!;
    expect(posGc1.position.x).toBe(posInner.position.x + STRIDE);
    expect(posGc1.position.y).toBe(posInner.position.y);
    expect(posGc2.position.x).toBe(posInner.position.x + STRIDE * 2);
    expect(posGc2.position.y).toBe(posInner.position.y);
  });

  it("group overlay covers horizontal children spread", () => {
    const parent = makeNode("p", 100, 50, { isContainer: true, isExpanded: true, depth: 1 });
    const c1 = makeChild("c1", "p", 2);
    const c2 = makeChild("c2", "p", 2);
    const c3 = makeChild("c3", "p", 2);
    const result = layoutNodes([parent, c1, c2, c3], new Set(["p"]));

    const group = result.find((n) => n.id === `${GROUP_ID_PREFIX}p`)!;
    expect(group).toBeDefined();

    // Overlay width must encompass parent at x=100 and last child at x=100+3*STRIDE
    const lastChildX = 100 + STRIDE * 3;
    const expectedWidth = lastChildX - 100 + CELL + (GAP_X - GAP_X / 2) * 2;
    expect(group.data.width).toBe(expectedWidth);
  });
});
