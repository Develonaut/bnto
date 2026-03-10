import { describe, it, expect } from "vitest";
import { injectPlaceholder, PLACEHOLDER_ID } from "./injectPlaceholder";
import { SLOTS } from "../adapters/bentoSlots";
import type { BentoNode } from "../adapters/types";

function makeNode(id: string, slotIndex: number): BentoNode {
  return {
    id,
    type: "io",
    position: { x: SLOTS[slotIndex]!.x, y: SLOTS[slotIndex]!.y },
    data: {
      label: id,
      variant: "info",
      width: 200,
      height: 200,
      status: "idle",
    },
  };
}

function ioNodes(): BentoNode[] {
  return [makeNode("input", 0), makeNode("output", 1)];
}

describe("injectPlaceholder", () => {
  it("injects placeholder between input and output (IO-only)", () => {
    const result = injectPlaceholder(ioNodes());
    expect(result).toHaveLength(3);
    expect(result[1]!.id).toBe(PLACEHOLDER_ID);
    expect(result[1]!.type).toBe("placeholder");
  });

  it("places placeholder at slot 1, shifts output to slot 2 (IO-only)", () => {
    const result = injectPlaceholder(ioNodes());
    expect(result[1]!.position).toEqual({ x: SLOTS[1]!.x, y: SLOTS[1]!.y });
    expect(result[2]!.position).toEqual({ x: SLOTS[2]!.x, y: SLOTS[2]!.y });
  });

  it("injects placeholder when processing nodes exist", () => {
    const nodes = [makeNode("input", 0), makeNode("image", 1), makeNode("output", 2)];
    const result = injectPlaceholder(nodes);
    expect(result).toHaveLength(4);
    expect(result[0]!.id).toBe("input");
    expect(result[1]!.id).toBe("image");
    expect(result[2]!.id).toBe(PLACEHOLDER_ID);
    expect(result[3]!.id).toBe("output");
  });

  it("places placeholder at correct slot index with processing nodes", () => {
    const nodes = [makeNode("input", 0), makeNode("image", 1), makeNode("output", 2)];
    const result = injectPlaceholder(nodes);
    expect(result[2]!.position).toEqual({ x: SLOTS[2]!.x, y: SLOTS[2]!.y });
    expect(result[3]!.position).toEqual({ x: SLOTS[3]!.x, y: SLOTS[3]!.y });
  });

  it("returns nodes unchanged when fewer than 2 nodes", () => {
    const nodes = [ioNodes()[0]!];
    expect(injectPlaceholder(nodes)).toBe(nodes);
  });

  it("marks placeholder as non-selectable and non-draggable", () => {
    const result = injectPlaceholder(ioNodes());
    expect(result[1]!.selectable).toBe(false);
    expect(result[1]!.draggable).toBe(false);
  });

  it("returns nodes unchanged when no slot available for output", () => {
    const nodes = Array.from({ length: SLOTS.length }, (_, i) => makeNode(`n${i}`, i));
    expect(injectPlaceholder(nodes)).toBe(nodes);
  });
});
