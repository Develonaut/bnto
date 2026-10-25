import { describe, expect, it } from "vitest";
import { categorizeError } from "./categorizeError";

describe("categorizeError", () => {
  it("categorizes RuntimeError as wasm_crash", () => {
    expect(categorizeError("RuntimeError: unreachable")).toBe("wasm_crash");
  });

  it("categorizes CompileError as wasm_crash", () => {
    expect(categorizeError("CompileError: wasm validation error")).toBe("wasm_crash");
  });

  it("categorizes timeout errors", () => {
    expect(categorizeError("timed out")).toBe("timeout");
    expect(categorizeError("Operation timed out after 30s")).toBe("timeout");
    expect(categorizeError("timeout exceeded")).toBe("timeout");
  });

  it("categorizes unsupported format errors", () => {
    expect(categorizeError("unsupported format: .bmp")).toBe("unsupported_format");
    expect(categorizeError("Unsupported format")).toBe("unsupported_format");
    expect(categorizeError("unsupported input type")).toBe("unsupported_format");
  });

  it("categorizes network errors", () => {
    expect(categorizeError("NetworkError: failed to fetch")).toBe("network");
    expect(categorizeError("Failed to fetch")).toBe("network");
    expect(categorizeError("net::ERR_CONNECTION_REFUSED")).toBe("network");
  });

  it("categorizes out of memory errors", () => {
    expect(categorizeError("out of memory")).toBe("out_of_memory");
    expect(categorizeError("RangeError: Out of memory")).toBe("out_of_memory");
  });

  // The next three messages are copied verbatim from `recipe_run_failed` events in
  // PostHog, so each category is pinned to what browsers and the engine really say.

  it("categorizes Safari's failed engine download as network", () => {
    expect(categorizeError("Failed to initialize WASM: Load failed")).toBe("network");
  });

  it("categorizes Chrome's unreadable dropped file as file_unreadable", () => {
    const chrome =
      "The requested file could not be read, typically due to permission problems " +
      "that have occurred after a reference to a file was acquired.";
    expect(categorizeError(chrome)).toBe("file_unreadable");
  });

  it("categorizes the engine's decode failure as decode_failed", () => {
    const engine =
      "Processing failed: Failed to decode overlay image: The image format could not be determined";
    expect(categorizeError(engine)).toBe("decode_failed");
  });

  // Not yet seen in PostHog: the engine's other decode wording, with a made-up cause.

  it("categorizes the engine's image decode failure as decode_failed", () => {
    expect(categorizeError("Processing failed: Failed to decode image: bad header")).toBe(
      "decode_failed",
    );
  });

  it("returns unknown for unrecognized errors", () => {
    expect(categorizeError("something weird")).toBe("unknown");
    expect(categorizeError("")).toBe("unknown");
  });
});
