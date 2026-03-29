import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { readAsBase64 } from "./readAsBase64";

/**
 * readAsBase64 wraps FileReader.readAsDataURL. These tests verify the
 * contract that callers depend on: the returned string is a complete
 * data URI (data:<mime>;base64,...) — NOT raw base64 that needs prefixing.
 *
 * We mock FileReader because Vitest's Node environment doesn't provide it.
 */

let capturedOnload: (() => void) | null;

class MockFileReader {
  result: string | null = null;
  onload: (() => void) | null = null;
  onerror: (() => void) | null = null;

  readAsDataURL(file: File) {
    const mime = file.type || "application/octet-stream";
    this.result = `data:${mime};base64,dGVzdA==`;
    capturedOnload = this.onload;
    queueMicrotask(() => this.onload?.());
  }
}

beforeEach(() => {
  capturedOnload = null;
  vi.stubGlobal("FileReader", MockFileReader);
});

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("readAsBase64", () => {
  it("returns a complete data URI with mime type", async () => {
    const file = new File(["hello"], "test.txt", { type: "text/plain" });
    const result = await readAsBase64(file);

    expect(result).toMatch(/^data:text\/plain;base64,/);
  });

  it("preserves the original file mime type", async () => {
    const file = new File(["img"], "photo.jpg", { type: "image/jpeg" });
    const result = await readAsBase64(file);

    expect(result).toMatch(/^data:image\/jpeg;base64,/);
  });

  it("contains exactly one data: prefix (no double-encoding)", async () => {
    const file = new File(["x"], "img.png", { type: "image/png" });
    const result = await readAsBase64(file);

    const dataCount = result.split("data:").length - 1;
    expect(dataCount).toBe(1);
  });

  it("result is directly usable as an img src", async () => {
    const file = new File(["x"], "logo.png", { type: "image/png" });
    const result = await readAsBase64(file);

    expect(result).toMatch(/^data:[^;]+;base64,[A-Za-z0-9+/=]+$/);
  });
});
