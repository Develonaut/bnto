import { describe, it, expect } from "vitest";
import { getPercentSmaller } from "./getPercentSmaller";
import type { BrowserFileResult } from "../types/browser";

function result(blobSize: number, metadata: BrowserFileResult["metadata"]): BrowserFileResult {
  return {
    blob: new Blob(["x".repeat(blobSize)]),
    filename: "a.jpg",
    mimeType: "image/jpeg",
    metadata,
  };
}

describe("getPercentSmaller", () => {
  it("measures against the uploaded file", () => {
    expect(getPercentSmaller(result(250, { originalSize: 300, inputSize: 1000 }))).toBe(75);
  });

  it("is undefined when the output grew, even if a step reported a ratio", () => {
    // compress → overlay: compress's ratio survives, but the file got bigger.
    const r = result(1200, { originalSize: 1000, compressionRatio: 40 });
    expect(getPercentSmaller(r)).toBeUndefined();
  });

  it("is undefined when no node reported sizes", () => {
    expect(getPercentSmaller(result(250, { inputSize: 1000 }))).toBeUndefined();
  });
});
