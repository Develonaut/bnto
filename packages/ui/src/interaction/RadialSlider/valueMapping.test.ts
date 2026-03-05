import { describe, it, expect } from "vitest";

import { valueToAngle, angleToValue, pointerToAngle } from "./valueMapping";

describe("valueToAngle", () => {
  it("maps min value to startAngle", () => {
    expect(valueToAngle(0, 0, 100, 135, 405)).toBe(135);
  });

  it("maps max value to endAngle", () => {
    expect(valueToAngle(100, 0, 100, 135, 405)).toBe(405);
  });

  it("maps midpoint to middle of arc", () => {
    expect(valueToAngle(50, 0, 100, 135, 405)).toBe(270);
  });

  it("clamps below min to startAngle", () => {
    expect(valueToAngle(-10, 0, 100, 135, 405)).toBe(135);
  });

  it("clamps above max to endAngle", () => {
    expect(valueToAngle(150, 0, 100, 135, 405)).toBe(405);
  });

  it("handles wrapping arc (endAngle < startAngle)", () => {
    // startAngle=300, endAngle=60 → span wraps around, should be 120 degrees
    const angle = valueToAngle(50, 0, 100, 300, 60);
    // t=0.5, span = 60-300 = -240 → +360 = 120, so 300 + 0.5*120 = 360
    expect(angle).toBeCloseTo(360);
  });
});

describe("angleToValue", () => {
  it("maps startAngle to min", () => {
    expect(angleToValue(135, 0, 100, 135, 405)).toBe(0);
  });

  it("maps endAngle to max", () => {
    expect(angleToValue(405, 0, 100, 135, 405)).toBe(100);
  });

  it("maps middle angle to midpoint", () => {
    expect(angleToValue(270, 0, 100, 135, 405)).toBe(50);
  });

  it("clamps to max when past end in dead zone (closer to end)", () => {
    // Dead zone is 360-270 = 90 degrees. Angle at 410 → offset = 275, past span of 270
    // pastEnd = 5, deadZone = 90, 5 < 45 → clamp to span (max)
    expect(angleToValue(410, 0, 100, 135, 405)).toBe(100);
  });

  it("clamps to min when past end in dead zone (closer to start)", () => {
    // For arc 135-405 (span 270), dead zone is 90 degrees
    // Angle at 90 → offset = 90 - 135 = -45 → +360 = 315
    // pastEnd = 315 - 270 = 45, deadZone = 90, 45 >= 45 → clamp to 0 (min)
    expect(angleToValue(90, 0, 100, 135, 405)).toBe(0);
  });

  it("returns integer values (rounds)", () => {
    const result = angleToValue(200, 0, 100, 135, 405);
    expect(Number.isInteger(result)).toBe(true);
  });
});

describe("pointerToAngle", () => {
  const rect = { left: 0, top: 0, width: 100, height: 100 } as DOMRect;

  it("returns 0 for pointer directly above center (12 o'clock)", () => {
    // Center is (50, 50), pointer at (50, 0) → straight up
    expect(pointerToAngle(50, 0, rect)).toBeCloseTo(0);
  });

  it("returns 90 for pointer directly right of center (3 o'clock)", () => {
    expect(pointerToAngle(100, 50, rect)).toBeCloseTo(90);
  });

  it("returns 180 for pointer directly below center (6 o'clock)", () => {
    expect(pointerToAngle(50, 100, rect)).toBeCloseTo(180);
  });

  it("returns 270 for pointer directly left of center (9 o'clock)", () => {
    expect(pointerToAngle(0, 50, rect)).toBeCloseTo(270);
  });
});
