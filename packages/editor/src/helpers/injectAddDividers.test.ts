/**
 * injectAddDividers tests — pure function, no React or store needed.
 */

import { describe, it, expect } from "vitest";
import { injectAddDividers, ADD_DIVIDER_PREFIX } from "./injectAddDividers";
import type { BentoNode } from "../adapters/types";
import { CELL, GAP_X, STRIDE, ROW_OFFSET, DIVIDER_THIN } from "../adapters/bentoSlots";

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
      width: CELL,
      height: CELL,
      status: "idle",
      depth: 0,
      ...overrides,
    },
  };
}

function makeChild(id: string, parentId: string, x: number, y: number, depth = 1): BentoNode {
  return {
    id,
    type: "compartment",
    position: { x, y },
    data: {
      label: id,
      variant: "primary",
      width: CELL,
      height: CELL,
      status: "idle",
      parentContainerId: parentId,
      depth,
    },
  };
}

function dividers(result: BentoNode[]) {
  return result.filter((n) => n.type === "addDivider");
}

describe("injectAddDividers", () => {
  it("returns nodes unchanged when no top-level gaps and no expanded containers", () => {
    const nodes = [makeNode("a", 0, 0)];
    const result = injectAddDividers(nodes, new Set());
    expect(result).toBe(nodes); // referentially stable
  });

  it("injects horizontal dividers between top-level nodes", () => {
    const a = makeNode("a", 0, 0);
    const b = makeNode("b", STRIDE, 0);
    const c = makeNode("c", STRIDE * 2, 0);
    const result = injectAddDividers([a, b, c], new Set());
    const divs = dividers(result);

    expect(divs).toHaveLength(2);
    // Dividers are thin vertically, full height
    expect(divs[0]!.data.width).toBe(DIVIDER_THIN);
    expect(divs[0]!.data.height).toBe(CELL);
    expect(divs[0]!.data.dividerDirection).toBe("horizontal");
    // Top-level dividers show the dashed line
    expect(divs[0]!.data.dividerHideLine).toBeFalsy();
  });

  it("positions top-level divider centered in the gap", () => {
    const a = makeNode("a", 0, 0);
    const b = makeNode("b", STRIDE, 0);
    const result = injectAddDividers([a, b], new Set());
    const div = dividers(result)[0]!;

    const expectedX = CELL + (GAP_X - DIVIDER_THIN) / 2;
    expect(div.position.x).toBe(expectedX);
    expect(div.position.y).toBe(0);
  });

  describe("vertical container (depth 0)", () => {
    it("injects divider below empty container", () => {
      const parent = makeNode("p", 0, 0, { isContainer: true, isExpanded: true, depth: 0 });
      const result = injectAddDividers([parent], new Set(["p"]));
      const divs = dividers(result);

      expect(divs).toHaveLength(1);
      expect(divs[0]!.data.dividerDirection).toBe("vertical");
      expect(divs[0]!.data.width).toBe(CELL);
      expect(divs[0]!.data.height).toBe(DIVIDER_THIN);
      expect(divs[0]!.data.dividerIntoContainerId).toBe("p");
      expect(divs[0]!.data.dividerAfterNodeId).toBeNull();
      // Edge divider hides line (overlaps group border)
      expect(divs[0]!.data.dividerHideLine).toBe(true);
    });

    it("injects 3 dividers for a container with 2 children", () => {
      const parent = makeNode("p", 0, 0, { isContainer: true, isExpanded: true, depth: 0 });
      const c1 = makeChild("c1", "p", 0, ROW_OFFSET);
      const c2 = makeChild("c2", "p", 0, ROW_OFFSET * 2);
      const result = injectAddDividers([parent, c1, c2], new Set(["p"]));
      const divs = dividers(result);

      // before-first + between + after-last = 3
      expect(divs).toHaveLength(3);
      divs.forEach((d) => {
        expect(d.data.dividerDirection).toBe("vertical");
        expect(d.data.width).toBe(CELL);
        expect(d.data.height).toBe(DIVIDER_THIN);
      });
    });

    it("sets correct afterNodeId and intoContainerId", () => {
      const parent = makeNode("p", 0, 0, { isContainer: true, isExpanded: true, depth: 0 });
      const c1 = makeChild("c1", "p", 0, ROW_OFFSET);
      const c2 = makeChild("c2", "p", 0, ROW_OFFSET * 2);
      const result = injectAddDividers([parent, c1, c2], new Set(["p"]));
      const divs = dividers(result);

      // before-first: afterNodeId=null, shows line (between parent and child)
      const beforeFirst = divs.find((d) => d.id.includes("first__"))!;
      expect(beforeFirst.data.dividerAfterNodeId).toBeNull();
      expect(beforeFirst.data.dividerIntoContainerId).toBe("p");
      expect(beforeFirst.data.dividerHideLine).toBeFalsy();

      // between: afterNodeId=c1, shows line (between children)
      const between = divs.find((d) => d.id === `${ADD_DIVIDER_PREFIX}c1`)!;
      expect(between.data.dividerAfterNodeId).toBe("c1");
      expect(between.data.dividerHideLine).toBeFalsy();

      // after-last: afterNodeId=c2, hides line (edge of group)
      const afterLast = divs.find((d) => d.id.includes("last__"))!;
      expect(afterLast.data.dividerAfterNodeId).toBe("c2");
      expect(afterLast.data.dividerHideLine).toBe(true);
    });
  });

  describe("horizontal container (depth 1)", () => {
    it("injects divider right of empty container", () => {
      const parent = makeNode("p", 0, 0, { isContainer: true, isExpanded: true, depth: 1 });
      const result = injectAddDividers([parent], new Set(["p"]));
      const divs = dividers(result);

      expect(divs).toHaveLength(1);
      expect(divs[0]!.data.dividerDirection).toBe("horizontal");
      expect(divs[0]!.data.width).toBe(DIVIDER_THIN);
      expect(divs[0]!.data.height).toBe(CELL);
      // Positioned to the right of parent, at parent's y
      expect(divs[0]!.position.y).toBe(0);
    });

    it("injects 3 dividers for a container with 2 children", () => {
      const parent = makeNode("p", 0, 0, { isContainer: true, isExpanded: true, depth: 1 });
      const c1 = makeChild("c1", "p", STRIDE, 0, 2);
      const c2 = makeChild("c2", "p", STRIDE * 2, 0, 2);
      const result = injectAddDividers([parent, c1, c2], new Set(["p"]));
      const divs = dividers(result);

      expect(divs).toHaveLength(3);
      divs.forEach((d) => {
        expect(d.data.dividerDirection).toBe("horizontal");
        expect(d.data.width).toBe(DIVIDER_THIN);
        expect(d.data.height).toBe(CELL);
      });
    });

    it("positions dividers at parent y for horizontal flow", () => {
      const parent = makeNode("p", 0, 100, { isContainer: true, isExpanded: true, depth: 1 });
      const c1 = makeChild("c1", "p", STRIDE, 100, 2);
      const result = injectAddDividers([parent, c1], new Set(["p"]));
      const divs = dividers(result);

      divs.forEach((d) => {
        expect(d.position.y).toBe(100);
      });
    });
  });

  it("hides line on top-level dividers adjacent to expanded containers", () => {
    const a = makeNode("a", 0, 0);
    const container = makeNode("p", STRIDE, 0, { isContainer: true, isExpanded: true, depth: 0 });
    const b = makeNode("b", STRIDE * 2, 0);
    const result = injectAddDividers([a, container, b], new Set(["p"]));
    const divs = dividers(result).filter((d) => !d.data.dividerIntoContainerId);

    expect(divs).toHaveLength(2);
    // Both dividers adjacent to the container hide their line
    divs.forEach((d) => {
      expect(d.data.dividerHideLine).toBe(true);
    });
  });

  it("shows line on top-level dividers between non-container nodes", () => {
    const a = makeNode("a", 0, 0);
    const b = makeNode("b", STRIDE, 0);
    const result = injectAddDividers([a, b], new Set());
    const divs = dividers(result);

    expect(divs).toHaveLength(1);
    expect(divs[0]!.data.dividerHideLine).toBeFalsy();
  });

  it("skips divider injection when placeholder is present", () => {
    const a = makeNode("a", 0, 0);
    const b = makeNode("b", STRIDE, 0);
    const placeholder: BentoNode = {
      id: "__placeholder__",
      type: "placeholder",
      position: { x: STRIDE * 2, y: 0 },
      data: { label: "", variant: "muted", width: CELL, height: CELL, status: "idle" },
    };
    const result = injectAddDividers([a, b, placeholder], new Set());
    const divs = dividers(result);

    // Top-level dividers suppressed when placeholder is present
    expect(divs).toHaveLength(0);
  });

  it("filters out synthetic nodes from partition", () => {
    const a = makeNode("a", 0, 0);
    const group: BentoNode = {
      id: "__group__p",
      type: "containerGroup",
      position: { x: 0, y: 0 },
      data: { label: "", variant: "muted", width: 200, height: 200, status: "idle" },
    };
    const result = injectAddDividers([a, group], new Set());
    // No dividers — only one real top-level node
    expect(dividers(result)).toHaveLength(0);
  });
});
