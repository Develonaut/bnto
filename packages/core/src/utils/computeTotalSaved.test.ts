import { describe, it, expect } from "vitest";
import { computeTotalSaved } from "./computeTotalSaved";
import type { BrowserFileResult } from "../types/browser";

/** Helper to build a minimal BrowserFileResult with the fields computeTotalSaved reads. */
function makeResult(blobSize: number, originalSize?: number): BrowserFileResult {
  return {
    blob: new Blob(["x".repeat(blobSize)]),
    filename: "test.jpg",
    metadata: originalSize != null ? { originalSize } : {},
  } as BrowserFileResult;
}

describe("computeTotalSaved", () => {
  it("returns 0 for an empty array", () => {
    expect(computeTotalSaved([])).toBe(0);
  });

  it("sums savings across multiple results", () => {
    const results = [
      makeResult(600, 1000), // saved 400
      makeResult(800, 1200), // saved 400
    ];
    expect(computeTotalSaved(results)).toBe(800);
  });

  it("returns 0 when no results have originalSize metadata", () => {
    const results = [makeResult(500), makeResult(300)];
    expect(computeTotalSaved(results)).toBe(0);
  });

  it("skips results without originalSize and sums the rest", () => {
    const results = [
      makeResult(600, 1000), // saved 400
      makeResult(500), // no metadata — skipped
      makeResult(200, 500), // saved 300
    ];
    expect(computeTotalSaved(results)).toBe(700);
  });

  it("returns negative when output is larger than original", () => {
    const results = [makeResult(1500, 1000)]; // grew by 500
    expect(computeTotalSaved(results)).toBe(-500);
  });

  it("returns 0 when output equals original", () => {
    const results = [makeResult(1000, 1000)];
    expect(computeTotalSaved(results)).toBe(0);
  });
});
