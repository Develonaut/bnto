import { describe, it, expect } from "vitest";

import { toRad, polarToOffset, arcPath } from "./geometry";

describe("toRad", () => {
  it("converts 0 degrees to 0 radians", () => {
    expect(toRad(0)).toBe(0);
  });

  it("converts 180 degrees to PI radians", () => {
    expect(toRad(180)).toBeCloseTo(Math.PI);
  });

  it("converts 360 degrees to 2*PI radians", () => {
    expect(toRad(360)).toBeCloseTo(2 * Math.PI);
  });

  it("converts 90 degrees to PI/2 radians", () => {
    expect(toRad(90)).toBeCloseTo(Math.PI / 2);
  });
});

describe("polarToOffset", () => {
  it("returns {0, -r} for 0 degrees (12 o'clock)", () => {
    const result = polarToOffset(0, 100);
    expect(result.x).toBeCloseTo(0);
    expect(result.y).toBeCloseTo(-100);
  });

  it("returns {r, 0} for 90 degrees (3 o'clock)", () => {
    const result = polarToOffset(90, 100);
    expect(result.x).toBeCloseTo(100);
    expect(result.y).toBeCloseTo(0);
  });

  it("returns {0, r} for 180 degrees (6 o'clock)", () => {
    const result = polarToOffset(180, 100);
    expect(result.x).toBeCloseTo(0);
    expect(result.y).toBeCloseTo(100);
  });

  it("returns {-r, 0} for 270 degrees (9 o'clock)", () => {
    const result = polarToOffset(270, 100);
    expect(result.x).toBeCloseTo(-100);
    expect(result.y).toBeCloseTo(0);
  });
});

describe("arcPath", () => {
  it("returns empty string for zero sweep", () => {
    expect(arcPath(0, 0, 50, 50, 50)).toBe("");
  });

  it("returns a valid SVG path for a partial arc", () => {
    const path = arcPath(0, 90, 50, 50, 50);
    expect(path).toContain("M ");
    expect(path).toContain("A ");
    expect(path).not.toContain("undefined");
  });

  it("uses large-arc flag for arcs > 180 degrees", () => {
    const path = arcPath(0, 270, 50, 50, 50);
    // large-arc flag should be 1
    expect(path).toMatch(/A \d+ \d+ 0 1 1/);
  });

  it("uses small-arc flag for arcs <= 180 degrees", () => {
    const path = arcPath(0, 90, 50, 50, 50);
    // large-arc flag should be 0
    expect(path).toMatch(/A \d+ \d+ 0 0 1/);
  });

  it("renders a full circle as two semicircles", () => {
    const path = arcPath(0, 360, 50, 50, 50);
    // Full circle has two A commands
    const arcCount = (path.match(/A /g) || []).length;
    expect(arcCount).toBe(2);
  });
});
