import { describe, expect, it } from "vitest";
import { bucketFileSize } from "./bucketFileSize";

describe("bucketFileSize", () => {
  it("returns <100KB for 0 bytes", () => {
    expect(bucketFileSize(0)).toBe("<100KB");
  });

  it("returns <100KB for 99,999 bytes", () => {
    expect(bucketFileSize(99_999)).toBe("<100KB");
  });

  it("returns 100KB-1MB at exactly 100,000 bytes", () => {
    expect(bucketFileSize(100_000)).toBe("100KB-1MB");
  });

  it("returns 100KB-1MB for 999,999 bytes", () => {
    expect(bucketFileSize(999_999)).toBe("100KB-1MB");
  });

  it("returns 1-10MB at exactly 1,000,000 bytes", () => {
    expect(bucketFileSize(1_000_000)).toBe("1-10MB");
  });

  it("returns 1-10MB for 9,999,999 bytes", () => {
    expect(bucketFileSize(9_999_999)).toBe("1-10MB");
  });

  it("returns 10-100MB at exactly 10,000,000 bytes", () => {
    expect(bucketFileSize(10_000_000)).toBe("10-100MB");
  });

  it("returns >100MB at exactly 100,000,000 bytes", () => {
    expect(bucketFileSize(100_000_000)).toBe(">100MB");
  });

  it("returns >100MB for 500,000,000 bytes", () => {
    expect(bucketFileSize(500_000_000)).toBe(">100MB");
  });
});
