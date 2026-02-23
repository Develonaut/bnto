import { describe, expect, it } from "vitest";
import {
  getAcceptedTypes,
  isFileAccepted,
  toDropzoneAccept,
} from "../getAcceptedTypes";

describe("getAcceptedTypes", () => {
  it("returns image types for compress-images", () => {
    const result = getAcceptedTypes("compress-images");
    expect(result.accept).toContain("image/jpeg");
    expect(result.accept).toContain("image/png");
    expect(result.accept).toContain("image/webp");
    expect(result.label).toContain("JPEG");
  });

  it("returns CSV type for clean-csv", () => {
    const result = getAcceptedTypes("clean-csv");
    expect(result.accept).toContain(".csv");
    expect(result.label).toContain("CSV");
  });

  it("returns wildcard for rename-files", () => {
    const result = getAcceptedTypes("rename-files");
    expect(result.accept).toBe("*/*");
  });

  it("returns default for unknown slugs", () => {
    const result = getAcceptedTypes("unknown-bnto");
    expect(result.accept).toBe("*/*");
    expect(result.label).toBe("files");
  });
});

describe("isFileAccepted", () => {
  function makeFile(name: string, type: string): File {
    return new File(["content"], name, { type });
  }

  it("accepts JPEG for compress-images", () => {
    expect(isFileAccepted(makeFile("photo.jpg", "image/jpeg"), "compress-images")).toBe(true);
  });

  it("accepts PNG for compress-images", () => {
    expect(isFileAccepted(makeFile("icon.png", "image/png"), "compress-images")).toBe(true);
  });

  it("rejects PDF for compress-images", () => {
    expect(isFileAccepted(makeFile("doc.pdf", "application/pdf"), "compress-images")).toBe(false);
  });

  it("accepts CSV by extension for clean-csv", () => {
    expect(isFileAccepted(makeFile("data.csv", "text/csv"), "clean-csv")).toBe(true);
  });

  it("accepts CSV by extension when MIME is generic", () => {
    expect(isFileAccepted(makeFile("data.csv", "application/octet-stream"), "clean-csv")).toBe(true);
  });

  it("rejects non-CSV for clean-csv", () => {
    expect(isFileAccepted(makeFile("data.json", "application/json"), "clean-csv")).toBe(false);
  });

  it("accepts any file for rename-files", () => {
    expect(isFileAccepted(makeFile("anything.xyz", "application/octet-stream"), "rename-files")).toBe(true);
  });

  it("accepts GIF for convert-image-format", () => {
    expect(isFileAccepted(makeFile("anim.gif", "image/gif"), "convert-image-format")).toBe(true);
  });
});

describe("toDropzoneAccept", () => {
  it("returns MIME keys for compress-images", () => {
    const result = toDropzoneAccept("compress-images");
    expect(result).toEqual({
      "image/jpeg": [],
      "image/png": [],
      "image/webp": [],
    });
  });

  it("returns undefined for wildcard slugs", () => {
    expect(toDropzoneAccept("rename-files")).toBeUndefined();
  });

  it("returns undefined for unknown slugs", () => {
    expect(toDropzoneAccept("unknown-bnto")).toBeUndefined();
  });

  it("attaches extensions to first MIME for clean-csv", () => {
    const result = toDropzoneAccept("clean-csv");
    expect(result).toEqual({
      "text/csv": [".csv"],
    });
  });

  it("includes all four MIME types for convert-image-format", () => {
    const result = toDropzoneAccept("convert-image-format");
    expect(result).toEqual({
      "image/jpeg": [],
      "image/png": [],
      "image/webp": [],
      "image/gif": [],
    });
  });
});
