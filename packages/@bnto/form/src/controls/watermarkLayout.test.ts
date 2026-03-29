import { describe, expect, it } from "vitest";
import { ANCHOR_PERCENT, imageAreaStyle } from "./watermarkLayout";

/**
 * Contract test: preview anchors must match engine's calculate_position().
 *
 * The Rust engine (watermark.rs) uses these anchor points on a source image
 * of dimensions (sw, sh):
 *   top-left     → (0, 0)
 *   top-center   → (sw/2, 0)
 *   bottom-right → (sw, sh)
 *   etc.
 *
 * Expressed as percentages of the image dimensions:
 *   top-left     → (0%, 0%)
 *   top-center   → (50%, 0%)
 *   bottom-right → (100%, 100%)
 *
 * If these drift apart, the preview will misrepresent the output.
 */

describe("ANCHOR_PERCENT", () => {
  /** Engine anchor points as percentages — derived from watermark.rs calculate_position(). */
  const ENGINE_ANCHORS: Record<string, { x: number; y: number }> = {
    "top-left": { x: 0, y: 0 },
    "top-center": { x: 50, y: 0 },
    "top-right": { x: 100, y: 0 },
    "middle-left": { x: 0, y: 50 },
    center: { x: 50, y: 50 },
    "middle-right": { x: 100, y: 50 },
    "bottom-left": { x: 0, y: 100 },
    "bottom-center": { x: 50, y: 100 },
    "bottom-right": { x: 100, y: 100 },
  };

  it("has all 9 positions", () => {
    expect(Object.keys(ANCHOR_PERCENT)).toHaveLength(9);
  });

  it.each(Object.entries(ENGINE_ANCHORS))(
    "position '%s' matches engine anchor (%o)",
    (pos, expected) => {
      const actual = ANCHOR_PERCENT[pos];
      expect(actual).toBeDefined();
      expect(actual.x).toBe(expected.x);
      expect(actual.y).toBe(expected.y);
    },
  );
});

describe("imageAreaStyle", () => {
  it("returns inset 0 when no ratio", () => {
    expect(imageAreaStyle(undefined)).toEqual({ inset: 0 });
  });

  it("returns inset 0 for square images", () => {
    expect(imageAreaStyle(1)).toEqual({ inset: 0 });
  });

  it("adds top/bottom padding for landscape images", () => {
    const style = imageAreaStyle(2);
    expect(style).toEqual({
      left: 0,
      right: 0,
      top: "25%",
      bottom: "25%",
    });
  });

  it("adds left/right padding for portrait images", () => {
    const style = imageAreaStyle(0.5);
    expect(style).toEqual({
      top: 0,
      bottom: 0,
      left: "25%",
      right: "25%",
    });
  });

  it("computes correct insets for 16:9 aspect ratio", () => {
    const ratio = 16 / 9;
    const style = imageAreaStyle(ratio);
    const expectedH = (1 / ratio) * 100;
    const expectedPad = (100 - expectedH) / 2;
    expect(style).toEqual({
      left: 0,
      right: 0,
      top: `${expectedPad}%`,
      bottom: `${expectedPad}%`,
    });
  });
});
