import { describe, expect, it } from "vitest";
import {
  isAllowedMimeType,
  isFileSizeAllowed,
  getMaxFileSize,
  sanitizeFileName,
  validateUploadBatch,
  ALLOWED_MIME_TYPES,
  MAX_UPLOAD_BATCH_SIZE,
  PRESIGNED_URL_EXPIRY_SECONDS,
} from "./_helpers/upload_validation";
import type { UploadFileInput } from "./_helpers/upload_validation";

/** Helper to create a valid file input for tests. */
function validFile(overrides?: Partial<UploadFileInput>): UploadFileInput {
  return {
    name: "photo.png",
    contentType: "image/png",
    sizeBytes: 1024,
    ...overrides,
  };
}

describe("upload validation", () => {
  describe("isAllowedMimeType", () => {
    it("accepts image MIME types", () => {
      expect(isAllowedMimeType("image/png")).toBe(true);
      expect(isAllowedMimeType("image/jpeg")).toBe(true);
      expect(isAllowedMimeType("image/webp")).toBe(true);
      expect(isAllowedMimeType("image/gif")).toBe(true);
      expect(isAllowedMimeType("image/svg+xml")).toBe(true);
      expect(isAllowedMimeType("image/tiff")).toBe(true);
      expect(isAllowedMimeType("image/bmp")).toBe(true);
    });

    it("accepts CSV and spreadsheet MIME types", () => {
      expect(isAllowedMimeType("text/csv")).toBe(true);
      expect(isAllowedMimeType("text/tab-separated-values")).toBe(true);
      expect(isAllowedMimeType("application/vnd.ms-excel")).toBe(true);
      expect(
        isAllowedMimeType(
          "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ),
      ).toBe(true);
    });

    it("accepts text and JSON MIME types", () => {
      expect(isAllowedMimeType("text/plain")).toBe(true);
      expect(isAllowedMimeType("application/json")).toBe(true);
    });

    it("rejects unknown MIME types", () => {
      expect(isAllowedMimeType("application/pdf")).toBe(false);
      expect(isAllowedMimeType("application/zip")).toBe(false);
      expect(isAllowedMimeType("video/mp4")).toBe(false);
      expect(isAllowedMimeType("audio/mpeg")).toBe(false);
      expect(isAllowedMimeType("application/octet-stream")).toBe(false);
    });

    it("rejects empty string", () => {
      expect(isAllowedMimeType("")).toBe(false);
    });

    it("is case-sensitive (MIME types are lowercase by spec)", () => {
      expect(isAllowedMimeType("IMAGE/PNG")).toBe(false);
      expect(isAllowedMimeType("Image/Png")).toBe(false);
    });
  });

  describe("getMaxFileSize", () => {
    it("returns 25MB for free plan", () => {
      expect(getMaxFileSize("free")).toBe(25 * 1024 * 1024);
    });

    it("returns 100MB for starter plan", () => {
      expect(getMaxFileSize("starter")).toBe(100 * 1024 * 1024);
    });

    it("returns 500MB for pro plan", () => {
      expect(getMaxFileSize("pro")).toBe(500 * 1024 * 1024);
    });

    it("defaults to free plan for unknown plan names", () => {
      expect(getMaxFileSize("enterprise")).toBe(25 * 1024 * 1024);
      expect(getMaxFileSize("")).toBe(25 * 1024 * 1024);
    });
  });

  describe("isFileSizeAllowed", () => {
    it("allows files within the free plan limit", () => {
      expect(isFileSizeAllowed(1024, "free")).toBe(true);
      expect(isFileSizeAllowed(25 * 1024 * 1024, "free")).toBe(true);
    });

    it("rejects files exceeding the free plan limit", () => {
      expect(isFileSizeAllowed(25 * 1024 * 1024 + 1, "free")).toBe(false);
    });

    it("allows larger files for pro plan", () => {
      expect(isFileSizeAllowed(100 * 1024 * 1024, "pro")).toBe(true);
      expect(isFileSizeAllowed(500 * 1024 * 1024, "pro")).toBe(true);
    });

    it("rejects files exceeding the pro plan limit", () => {
      expect(isFileSizeAllowed(500 * 1024 * 1024 + 1, "pro")).toBe(false);
    });

    it("rejects zero-byte files", () => {
      expect(isFileSizeAllowed(0, "free")).toBe(false);
    });

    it("rejects negative sizes", () => {
      expect(isFileSizeAllowed(-1, "free")).toBe(false);
    });
  });

  describe("sanitizeFileName", () => {
    it("preserves safe filenames", () => {
      expect(sanitizeFileName("photo.png")).toBe("photo.png");
      expect(sanitizeFileName("my-file.csv")).toBe("my-file.csv");
      expect(sanitizeFileName("report_2024.xlsx")).toBe("report_2024.xlsx");
    });

    it("replaces special characters with underscores", () => {
      expect(sanitizeFileName("my file (1).png")).toBe("my_file_1.png");
      expect(sanitizeFileName("résumé.pdf")).toBe("r_sum.pdf");
    });

    it("collapses consecutive underscores", () => {
      expect(sanitizeFileName("a___b.png")).toBe("a_b.png");
      expect(sanitizeFileName("a   b.png")).toBe("a_b.png");
    });

    it("strips path components to prevent traversal", () => {
      expect(sanitizeFileName("../../../etc/passwd")).toBe("passwd");
      expect(sanitizeFileName("foo/bar/baz.txt")).toBe("baz.txt");
      expect(sanitizeFileName("C:\\Users\\evil\\file.txt")).toBe("file.txt");
    });

    it("strips leading dots (no hidden files)", () => {
      expect(sanitizeFileName(".hidden")).toBe("hidden");
      expect(sanitizeFileName("..double")).toBe("double");
    });

    it("returns 'file' for empty or all-special inputs", () => {
      expect(sanitizeFileName("")).toBe("file");
      expect(sanitizeFileName("...")).toBe("file");
      expect(sanitizeFileName("___")).toBe("file");
    });
  });

  describe("validateUploadBatch", () => {
    it("returns null for a valid single-file batch", () => {
      expect(validateUploadBatch([validFile()], "free")).toBeNull();
    });

    it("returns null for a valid multi-file batch", () => {
      const files = [
        validFile({ name: "a.png" }),
        validFile({ name: "b.csv", contentType: "text/csv" }),
        validFile({ name: "c.json", contentType: "application/json" }),
      ];
      expect(validateUploadBatch(files, "free")).toBeNull();
    });

    it("rejects empty batch", () => {
      const error = validateUploadBatch([], "free");
      expect(error).not.toBeNull();
      expect(error!.code).toBe("EMPTY_BATCH");
    });

    it("rejects batch exceeding max size", () => {
      const files = Array.from({ length: 51 }, (_, i) =>
        validFile({ name: `file-${i}.png` }),
      );
      const error = validateUploadBatch(files, "free");
      expect(error).not.toBeNull();
      expect(error!.code).toBe("BATCH_TOO_LARGE");
    });

    it("accepts batch at exactly max size", () => {
      const files = Array.from({ length: 50 }, (_, i) =>
        validFile({ name: `file-${i}.png` }),
      );
      expect(validateUploadBatch(files, "free")).toBeNull();
    });

    it("rejects file with empty name", () => {
      const error = validateUploadBatch([validFile({ name: "" })], "free");
      expect(error).not.toBeNull();
      expect(error!.code).toBe("INVALID_FILE_NAME");
    });

    it("rejects file with whitespace-only name", () => {
      const error = validateUploadBatch([validFile({ name: "   " })], "free");
      expect(error).not.toBeNull();
      expect(error!.code).toBe("INVALID_FILE_NAME");
    });

    it("rejects unsupported MIME type", () => {
      const error = validateUploadBatch(
        [validFile({ contentType: "application/pdf" })],
        "free",
      );
      expect(error).not.toBeNull();
      expect(error!.code).toBe("INVALID_FILE_TYPE");
      expect(error!.message).toContain("application/pdf");
      expect(error!.fileName).toBe("photo.png");
    });

    it("rejects file exceeding free plan size limit", () => {
      const error = validateUploadBatch(
        [validFile({ name: "big.png", sizeBytes: 26 * 1024 * 1024 })],
        "free",
      );
      expect(error).not.toBeNull();
      expect(error!.code).toBe("FILE_TOO_LARGE");
      expect(error!.fileName).toBe("big.png");
      expect(error!.maxSizeBytes).toBe(25 * 1024 * 1024);
      expect(error!.message).toContain("25MB");
      expect(error!.message).toContain("free");
    });

    it("allows file within pro plan limit that exceeds free limit", () => {
      const files = [validFile({ sizeBytes: 100 * 1024 * 1024 })];
      expect(validateUploadBatch(files, "pro")).toBeNull();
    });

    it("rejects file exceeding pro plan size limit", () => {
      const error = validateUploadBatch(
        [validFile({ name: "huge.png", sizeBytes: 501 * 1024 * 1024 })],
        "pro",
      );
      expect(error).not.toBeNull();
      expect(error!.code).toBe("FILE_TOO_LARGE");
      expect(error!.message).toContain("500MB");
      expect(error!.message).toContain("pro");
    });

    it("defaults to free plan limits for unknown plans", () => {
      const error = validateUploadBatch(
        [validFile({ sizeBytes: 26 * 1024 * 1024 })],
        "enterprise",
      );
      expect(error).not.toBeNull();
      expect(error!.code).toBe("FILE_TOO_LARGE");
    });

    it("rejects zero-byte files", () => {
      const error = validateUploadBatch(
        [validFile({ sizeBytes: 0 })],
        "free",
      );
      expect(error).not.toBeNull();
      expect(error!.code).toBe("FILE_TOO_LARGE");
    });

    it("returns first error when multiple files are invalid", () => {
      const files = [
        validFile({ name: "good.png" }),
        validFile({ name: "bad.exe", contentType: "application/x-msdownload" }),
        validFile({ name: "   " }),
      ];
      const error = validateUploadBatch(files, "free");
      expect(error).not.toBeNull();
      // Should catch the invalid MIME type first (second file)
      expect(error!.code).toBe("INVALID_FILE_TYPE");
      expect(error!.fileName).toBe("bad.exe");
    });

    it("validates all supported MIME types pass in a batch", () => {
      const types = [
        "image/png",
        "image/jpeg",
        "image/webp",
        "text/csv",
        "application/json",
        "text/plain",
      ];
      const files = types.map((contentType, i) =>
        validFile({ name: `file-${i}.txt`, contentType }),
      );
      expect(validateUploadBatch(files, "free")).toBeNull();
    });
  });

  describe("constants", () => {
    it("has a reasonable batch size limit", () => {
      expect(MAX_UPLOAD_BATCH_SIZE).toBe(50);
    });

    it("has a 15-minute URL expiry", () => {
      expect(PRESIGNED_URL_EXPIRY_SECONDS).toBe(900);
    });

    it("includes all expected image types", () => {
      const imageTypes = [
        "image/png",
        "image/jpeg",
        "image/webp",
        "image/gif",
      ];
      for (const type of imageTypes) {
        expect(ALLOWED_MIME_TYPES.has(type)).toBe(true);
      }
    });
  });
});
