import { describe, it, expect } from "vitest";
import { deriveFileResultProps } from "./deriveFileResultProps";
import type { BrowserFileResult } from "../types/browser";

function mockResult(
  overrides: Partial<BrowserFileResult> & { blobSize?: number } = {},
): BrowserFileResult {
  const { blobSize = 500, ...rest } = overrides;
  return {
    blob: new Blob(["x".repeat(blobSize)], { type: "image/png" }),
    filename: "photo.png",
    mimeType: "image/png",
    metadata: {},
    ...rest,
  };
}

describe("deriveFileResultProps", () => {
  it("extracts filename and extension", () => {
    const result = deriveFileResultProps(mockResult({ filename: "report.pdf" }));
    expect(result.filename).toBe("report.pdf");
    expect(result.extension).toBe("pdf");
  });

  it("handles files without extension", () => {
    const result = deriveFileResultProps(mockResult({ filename: "Makefile" }));
    expect(result.extension).toBeNull();
  });

  it("formats output size (decimal, k=1000)", () => {
    const result = deriveFileResultProps(mockResult({ blobSize: 1000 }));
    expect(result.outputSize).toBe("1 KB");
  });

  it("shows originalSize and savings when file got smaller", () => {
    const result = deriveFileResultProps(
      mockResult({
        blobSize: 500,
        metadata: { originalSize: 1000 },
      }),
    );
    expect(result.originalSize).toBe("1 KB");
    expect(result.savings).toBe("-50%");
  });

  it("omits savings when file got larger", () => {
    const result = deriveFileResultProps(
      mockResult({
        blobSize: 1500,
        metadata: { originalSize: 1000 },
      }),
    );
    expect(result.originalSize).toBeDefined();
    expect(result.savings).toBeUndefined();
  });

  it("omits originalSize when same as output", () => {
    const result = deriveFileResultProps(
      mockResult({
        blobSize: 1000,
        metadata: { originalSize: 1000 },
      }),
    );
    expect(result.originalSize).toBeUndefined();
    expect(result.savings).toBeUndefined();
  });

  it("omits originalSize when not provided", () => {
    const result = deriveFileResultProps(mockResult({ metadata: {} }));
    expect(result.originalSize).toBeUndefined();
    expect(result.savings).toBeUndefined();
  });

  it("handles zero-byte output", () => {
    const result = deriveFileResultProps(mockResult({ blobSize: 0 }));
    expect(result.outputSize).toBe("0 B");
  });
});
