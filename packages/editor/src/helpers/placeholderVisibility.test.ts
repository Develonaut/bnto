/**
 * PlaceholderSlot visibility logic tests.
 *
 * The PlaceholderSlot shows when all configs are I/O nodes (only input + output).
 * It disappears when any non-I/O node exists.
 * Tests the pure visibility computation extracted from CanvasShell.
 */

import { describe, it, expect } from "vitest";
import { isIoNodeType } from "@bnto/nodes";
import type { NodeConfigs } from "../adapters/types";

/** Mirror the store selector logic from CanvasShell. */
function shouldShowPlaceholder(configs: NodeConfigs): boolean {
  return Object.values(configs).every((c) => isIoNodeType(c.nodeType));
}

describe("PlaceholderSlot visibility", () => {
  it("shows when only I/O nodes exist", () => {
    const configs: NodeConfigs = {
      input: { nodeType: "input", name: "Input", parameters: {} },
      output: { nodeType: "output", name: "Output", parameters: {} },
    };
    expect(shouldShowPlaceholder(configs)).toBe(true);
  });

  it("hides when a processing node is added", () => {
    const configs: NodeConfigs = {
      input: { nodeType: "input", name: "Input", parameters: {} },
      output: { nodeType: "output", name: "Output", parameters: {} },
      img1: { nodeType: "image", name: "Image", parameters: {} },
    };
    expect(shouldShowPlaceholder(configs)).toBe(false);
  });

  it("shows when empty (edge case)", () => {
    expect(shouldShowPlaceholder({})).toBe(true);
  });

  it("hides when any single non-I/O node exists", () => {
    const configs: NodeConfigs = {
      input: { nodeType: "input", name: "Input", parameters: {} },
      transform1: { nodeType: "transform", name: "Transform", parameters: {} },
      output: { nodeType: "output", name: "Output", parameters: {} },
    };
    expect(shouldShowPlaceholder(configs)).toBe(false);
  });
});
