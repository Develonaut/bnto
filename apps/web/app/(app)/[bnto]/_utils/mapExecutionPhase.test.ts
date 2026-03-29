import { describe, expect, it } from "vitest";
import { mapExecutionPhase } from "./mapExecutionPhase";

describe("mapExecutionPhase", () => {
  it("maps 'processing' to 'running'", () => {
    expect(mapExecutionPhase("processing")).toBe("running");
  });

  it("maps 'completed' to 'completed'", () => {
    expect(mapExecutionPhase("completed")).toBe("completed");
  });

  it("maps 'failed' to 'failed'", () => {
    expect(mapExecutionPhase("failed")).toBe("failed");
  });

  it("maps unknown status to 'idle'", () => {
    expect(mapExecutionPhase("idle")).toBe("idle");
    expect(mapExecutionPhase("unknown")).toBe("idle");
    expect(mapExecutionPhase("")).toBe("idle");
  });
});
