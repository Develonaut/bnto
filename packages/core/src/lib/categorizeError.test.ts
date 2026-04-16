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

  it("returns unknown for unrecognized errors", () => {
    expect(categorizeError("something weird")).toBe("unknown");
    expect(categorizeError("")).toBe("unknown");
  });
});
