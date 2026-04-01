import { describe, it, expect } from "vitest";
import {
  POSITIONS,
  ENGINE_MARGIN_PERCENT,
  imageAreaStyle,
  overlayPositionStyle,
} from "./watermarkLayout";

/**
 * Contract tests — verify CSS positioning matches the Rust engine's
 * `calculate_position()` in `engine/crates/bnto-image/src/overlay.rs`.
 *
 * The engine uses gravity-based positioning: overlay placed fully visible
 * within its gravity region, with a 2% margin from nearest edges.
 */

describe("POSITIONS matches engine position enum", () => {
  it("has all 9 positions", () => {
    expect(POSITIONS).toHaveLength(9);
  });

  it("includes corner, edge, and center positions", () => {
    expect(POSITIONS).toContain("top-left");
    expect(POSITIONS).toContain("center");
    expect(POSITIONS).toContain("bottom-right");
  });
});

describe("overlayPositionStyle — gravity-based (matches engine)", () => {
  const M = `${ENGINE_MARGIN_PERCENT}%`;

  it("bottom-right: right/bottom with margin", () => {
    const style = overlayPositionStyle("bottom-right", 25);
    expect(style.right).toBe(M);
    expect(style.bottom).toBe(M);
    expect(style.left).toBeUndefined();
    expect(style.top).toBeUndefined();
  });

  it("top-left: left/top with margin", () => {
    const style = overlayPositionStyle("top-left", 25);
    expect(style.left).toBe(M);
    expect(style.top).toBe(M);
    expect(style.right).toBeUndefined();
    expect(style.bottom).toBeUndefined();
  });

  it("center: left 50% + top 50% with translate centering", () => {
    const style = overlayPositionStyle("center", 25);
    expect(style.left).toBe("50%");
    expect(style.top).toBe("50%");
    expect(style.transform).toBe("translate(calc(-50% + 0px), calc(-50% + 0px))");
  });

  it("top-center: centered horizontally, margin from top", () => {
    const style = overlayPositionStyle("top-center", 25);
    expect(style.left).toBe("50%");
    expect(style.top).toBe(M);
    expect(style.transform).toBe("translate(calc(-50% + 0px), 0px)");
  });

  it("middle-right: margin from right, centered vertically", () => {
    const style = overlayPositionStyle("middle-right", 25);
    expect(style.right).toBe(M);
    expect(style.top).toBe("50%");
    expect(style.transform).toBe("translate(0px, calc(-50% + 0px))");
  });

  it("applies pixel offsets via transform", () => {
    const style = overlayPositionStyle("center", 25, 10, -20);
    expect(style.transform).toBe("translate(calc(-50% + 10px), calc(-50% + -20px))");
  });

  it("edge position offsets are pure pixel values", () => {
    const style = overlayPositionStyle("bottom-right", 25, 5, -10);
    expect(style.transform).toBe("translate(5px, -10px)");
  });

  it("sets width as sizePercent", () => {
    const style = overlayPositionStyle("center", 30);
    expect(style.width).toBe("30%");
  });

  it("unknown position falls back to center/middle", () => {
    const style = overlayPositionStyle("unknown", 25);
    expect(style.left).toBe("50%");
    expect(style.top).toBe("50%");
  });
});

describe("imageAreaStyle aspect ratio containment", () => {
  it("landscape (16:9) — full width, reduced height", () => {
    const style = imageAreaStyle(16 / 9);
    expect(style.left).toBe(0);
    expect(style.right).toBe(0);
    expect(style.top).toMatch(/21\.8/);
  });

  it("portrait (3:4) — full height, reduced width", () => {
    const style = imageAreaStyle(3 / 4);
    expect(style.top).toBe(0);
    expect(style.bottom).toBe(0);
    expect(style.left).toBe("12.5%");
  });

  it("square (1:1) — no inset", () => {
    const style = imageAreaStyle(1);
    expect(style.top).toBe("0%");
    expect(style.bottom).toBe("0%");
    expect(style.left).toBe(0);
    expect(style.right).toBe(0);
  });

  it("extreme landscape (10:1) — very thin horizontal strip", () => {
    const style = imageAreaStyle(10);
    expect(style.left).toBe(0);
    expect(style.right).toBe(0);
    expect(style.top).toBe("45%");
    expect(style.bottom).toBe("45%");
  });
});
