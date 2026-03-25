import { describe, expect, it } from "vitest";
import { formatFileSize } from "./formatFileSize";

describe("formatFileSize", () => {
  it("formats zero bytes", () => {
    expect(formatFileSize(0)).toBe("0 B");
  });

  it("formats bytes", () => {
    expect(formatFileSize(512)).toBe("512 B");
  });

  it("formats kilobytes (decimal, k=1000)", () => {
    expect(formatFileSize(1000)).toBe("1 KB");
    expect(formatFileSize(1500)).toBe("1.5 KB");
  });

  it("formats megabytes", () => {
    expect(formatFileSize(1000000)).toBe("1 MB");
    expect(formatFileSize(2500000)).toBe("2.5 MB");
  });

  it("formats gigabytes", () => {
    expect(formatFileSize(1000000000)).toBe("1 GB");
  });

  it("rounds to one decimal place", () => {
    expect(formatFileSize(1200000)).toBe("1.2 MB");
  });
});
