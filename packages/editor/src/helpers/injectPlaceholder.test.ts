import { describe, it, expect } from "vitest";
import { injectPlaceholder, PLACEHOLDER_ID } from "./injectPlaceholder";
import { SLOTS } from "../adapters/bentoSlots";
import type { BentoNode } from "../adapters/types";

function ioNodes(): BentoNode[] {
  return [
    {
      id: "input",
      type: "compartment",
      position: { x: SLOTS[0]!.x, y: SLOTS[0]!.y },
      data: {
        label: "Input",
        variant: "info",
        width: 200,
        height: 200,
        status: "idle",
      },
    },
    {
      id: "output",
      type: "compartment",
      position: { x: SLOTS[1]!.x, y: SLOTS[1]!.y },
      data: {
        label: "Output",
        variant: "info",
        width: 200,
        height: 200,
        status: "idle",
      },
    },
  ];
}

describe("injectPlaceholder", () => {
  it("injects placeholder between input and output when onlyIoNodes", () => {
    const result = injectPlaceholder(ioNodes(), true);
    expect(result).toHaveLength(3);
    expect(result[1]!.id).toBe(PLACEHOLDER_ID);
    expect(result[1]!.type).toBe("placeholder");
  });

  it("places placeholder at slot 1, shifts output to slot 2", () => {
    const result = injectPlaceholder(ioNodes(), true);
    expect(result[1]!.position).toEqual({ x: SLOTS[1]!.x, y: SLOTS[1]!.y });
    expect(result[2]!.position).toEqual({ x: SLOTS[2]!.x, y: SLOTS[2]!.y });
  });

  it("returns nodes unchanged when onlyIoNodes is false", () => {
    const nodes = ioNodes();
    expect(injectPlaceholder(nodes, false)).toBe(nodes);
  });

  it("returns nodes unchanged when fewer than 2 nodes", () => {
    const nodes = [ioNodes()[0]!];
    expect(injectPlaceholder(nodes, true)).toBe(nodes);
  });

  it("marks placeholder as non-selectable and non-draggable", () => {
    const result = injectPlaceholder(ioNodes(), true);
    expect(result[1]!.selectable).toBe(false);
    expect(result[1]!.draggable).toBe(false);
  });
});
