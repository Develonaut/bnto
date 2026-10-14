import { describe, it, expect } from "vitest";
import { getOriginalSize } from "./getOriginalSize";

describe("getOriginalSize", () => {
  it("is undefined when no node reported sizes", () => {
    expect(getOriginalSize({})).toBeUndefined();
    expect(getOriginalSize({ inputSize: 900 })).toBeUndefined();
  });

  it("uses the uploaded file's size when a node reported sizes", () => {
    // resize → convert: convert's originalSize is the resized image, not the upload.
    expect(getOriginalSize({ originalSize: 300, inputSize: 900 })).toBe(900);
  });

  it("falls back to the reported originalSize without an upload size", () => {
    expect(getOriginalSize({ originalSize: 300 })).toBe(300);
  });

  it("ignores a non-numeric upload size from the engine's JSON", () => {
    // Metadata arrives via JSON.parse, so its types aren't guaranteed.
    expect(getOriginalSize(JSON.parse('{"originalSize":300,"inputSize":"900"}'))).toBe(300);
  });
});
